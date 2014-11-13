/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

//! Text layout.

#![deny(unsafe_blocks)]

use fragment::{Fragment, ScannedTextFragmentInfo, UnscannedTextFragment};
use inline::InlineFragments;

use gfx::font::{FontMetrics,RunMetrics};
use gfx::font_context::FontContext;
use gfx::text::glyph::CharIndex;
use gfx::text::text_run::TextRun;
use gfx::text::util::{mod, CompressWhitespaceNewline, CompressNone};
use servo_util::dlist;
use servo_util::geometry::Au;
use servo_util::logical_geometry::{LogicalSize, WritingMode};
use servo_util::range::Range;
use servo_util::smallvec::{SmallVec, SmallVec1};
use std::collections::DList;
use std::mem;
use style::ComputedValues;
use style::computed_values::{line_height, text_orientation, white_space};
use style::style_structs::Font as FontStyle;
use sync::Arc;

/// A stack-allocated object for scanning an inline flow into `TextRun`-containing `TextFragment`s.
pub struct TextRunScanner {
    pub clump: DList<Fragment>,
}

impl TextRunScanner {
    pub fn new() -> TextRunScanner {
        TextRunScanner {
            clump: DList::new(),
        }
    }

    pub fn scan_for_runs(&mut self, font_context: &mut FontContext, mut fragments: DList<Fragment>)
                         -> InlineFragments {
        debug!("TextRunScanner: scanning {:u} fragments for text runs...", fragments.len());

        // FIXME(pcwalton): We want to be sure not to allocate multiple times, since this is a
        // performance-critical spot, but this may overestimate and allocate too much memory.
        let mut new_fragments = Vec::with_capacity(fragments.len());
        let mut last_whitespace = true;
        while !fragments.is_empty() {
            // Create a clump.
            self.clump.append(dlist::split(&mut fragments));
            while !fragments.is_empty() && self.clump
                                               .back()
                                               .unwrap()
                                               .can_merge_with_fragment(fragments.front()
                                                                                 .unwrap()) {
                self.clump.append(dlist::split(&mut fragments));
            }

            // Flush that clump to the list of fragments we're building up.
            last_whitespace = self.flush_clump_to_list(font_context,
                                                       &mut new_fragments,
                                                       last_whitespace);
        }

        debug!("TextRunScanner: complete.");
        InlineFragments {
            fragments: new_fragments,
        }
    }

    /// A "clump" is a range of inline flow leaves that can be merged together into a single
    /// fragment. Adjacent text with the same style can be merged, and nothing else can.
    ///
    /// The flow keeps track of the fragments contained by all non-leaf DOM nodes. This is necessary
    /// for correct painting order. Since we compress several leaf fragments here, the mapping must
    /// be adjusted.
    fn flush_clump_to_list(&mut self,
                           font_context: &mut FontContext,
                           out_fragments: &mut Vec<Fragment>,
                           mut last_whitespace: bool)
                           -> bool {
        debug!("TextRunScanner: flushing {} fragments in range", self.clump.len());

        debug_assert!(!self.clump.is_empty());
        match self.clump.front().unwrap().specific {
            UnscannedTextFragment(_) => {}
            _ => {
                debug_assert!(self.clump.len() == 1,
                              "WAT: can't coalesce non-text nodes in flush_clump_to_list()!");
                out_fragments.push(self.clump.pop_front().unwrap());
                return last_whitespace
            }
        }

        // TODO(#177): Text run creation must account for the renderability of text by font group
        // fonts. This is probably achieved by creating the font group above and then letting
        // `FontGroup` decide which `Font` to stick into the text run.
        //
        // Concatenate all of the transformed strings together, saving the new character indices.
        let mut new_ranges: SmallVec1<Range<CharIndex>> = SmallVec1::new();
        let mut new_line_positions: SmallVec1<NewLinePositions> = SmallVec1::new();
        let mut char_total = CharIndex(0);
        let run = {
            let fontgroup;
            let compression;
            {
                let in_fragment = self.clump.front().unwrap();
                let font_style = in_fragment.style().get_font_arc();
                fontgroup = font_context.get_layout_font_group_for_style(font_style);
                compression = match in_fragment.white_space() {
                    white_space::normal | white_space::nowrap => CompressWhitespaceNewline,
                    white_space::pre => CompressNone,
                }
            }

            // First, transform/compress text of all the nodes.
            let mut run_text = String::new();
            for in_fragment in self.clump.iter() {
                let in_fragment = match in_fragment.specific {
                    UnscannedTextFragment(ref text_fragment_info) => &text_fragment_info.text,
                    _ => panic!("Expected an unscanned text fragment!"),
                };

                let mut new_line_pos = Vec::new();
                let old_length = CharIndex(run_text.as_slice().char_len() as int);
                last_whitespace = util::transform_text(in_fragment.as_slice(),
                                                       compression,
                                                       last_whitespace,
                                                       &mut run_text,
                                                       &mut new_line_pos);
                new_line_positions.push(NewLinePositions(new_line_pos));

                let added_chars = CharIndex(run_text.as_slice().char_len() as int) - old_length;
                new_ranges.push(Range::new(char_total, added_chars));
                char_total = char_total + added_chars;
            }

            // Now create the run.
            //
            // TextRuns contain a cycle which is usually resolved by the teardown sequence.
            // If no clump takes ownership, however, it will leak.
            if run_text.len() == 0 {
                self.clump = DList::new();
                return last_whitespace
            }
            Arc::new(box TextRun::new(&mut *fontgroup.fonts.get(0).borrow_mut(), run_text))
        };

        // Make new fragments with the run and adjusted text indices.
        debug!("TextRunScanner: pushing {} fragment(s)", self.clump.len());
        for (logical_offset, old_fragment) in
                mem::replace(&mut self.clump, DList::new()).into_iter().enumerate() {
            let range = *new_ranges.get(logical_offset);
            if range.is_empty() {
                debug!("Elided an `UnscannedTextFragment` because it was zero-length after \
                        compression; {}",
                       old_fragment);
                continue
            }

            let text_size = old_fragment.border_box.size;
            let &NewLinePositions(ref mut new_line_positions) =
                new_line_positions.get_mut(logical_offset);
            let new_text_fragment_info =
                box ScannedTextFragmentInfo::new(run.clone(),
                                                 range,
                                                 mem::replace(new_line_positions, Vec::new()),
                                                 text_size);
            let new_metrics = new_text_fragment_info.run.metrics_for_range(&range);
            let bounding_box_size = bounding_box_for_run_metrics(&new_metrics,
                                                                 old_fragment.style.writing_mode);
            let new_fragment = old_fragment.transform(bounding_box_size, new_text_fragment_info);
            out_fragments.push(new_fragment)
        }

        last_whitespace
    }
}

struct NewLinePositions(Vec<CharIndex>);

#[inline]
fn bounding_box_for_run_metrics(metrics: &RunMetrics, writing_mode: WritingMode)
                                -> LogicalSize<Au> {

    // This does nothing, but it will fail to build
    // when more values are added to the `text-orientation` CSS property.
    // This will be a reminder to update the code below.
    let dummy: Option<text_orientation::T> = None;
    match dummy {
        Some(text_orientation::sideways_right) |
        Some(text_orientation::sideways_left) |
        Some(text_orientation::sideways) |
        None => {}
    }

    // In vertical sideways or horizontal upgright text,
    // the "width" of text metrics is always inline
    // This will need to be updated when other text orientations are supported.
    LogicalSize::new(
        writing_mode,
        metrics.bounding_box.size.width,
        metrics.bounding_box.size.height)

}

/// Returns the metrics of the font represented by the given `FontStyle`, respectively.
///
/// `#[inline]` because often the caller only needs a few fields from the font metrics.
#[inline]
pub fn font_metrics_for_style(font_context: &mut FontContext, font_style: Arc<FontStyle>)
                              -> FontMetrics {
    let fontgroup = font_context.get_layout_font_group_for_style(font_style);
    fontgroup.fonts.get(0).borrow().metrics.clone()
}

/// Returns the line block-size needed by the given computed style and font size.
pub fn line_height_from_style(style: &ComputedValues, metrics: &FontMetrics) -> Au {
    let font_size = style.get_font().font_size;
    match style.get_inheritedbox().line_height {
        line_height::Normal => metrics.line_gap,
        line_height::Number(l) => font_size.scale_by(l),
        line_height::Length(l) => l
    }
}
