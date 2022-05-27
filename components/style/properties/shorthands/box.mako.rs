/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

<%namespace name="helpers" file="/helpers.mako.rs" />

${helpers.two_properties_shorthand(
    "overflow",
    "overflow-x",
    "overflow-y",
    engines="gecko servo-2013 servo-2020",
    flags="SHORTHAND_IN_GETCS",
    spec="https://drafts.csswg.org/css-overflow/#propdef-overflow",
)}

${helpers.two_properties_shorthand(
    "overflow-clip-box",
    "overflow-clip-box-block",
    "overflow-clip-box-inline",
    engines="gecko",
    enabled_in="ua",
    gecko_pref="layout.css.overflow-clip-box.enabled",
    spec="Internal, may be standardized in the future "
         "(https://developer.mozilla.org/en-US/docs/Web/CSS/overflow-clip-box)",
)}

${helpers.two_properties_shorthand(
    "overscroll-behavior",
    "overscroll-behavior-x",
    "overscroll-behavior-y",
    engines="gecko",
    gecko_pref="layout.css.overscroll-behavior.enabled",
    spec="https://wicg.github.io/overscroll-behavior/#overscroll-behavior-properties",
)}

<%helpers:shorthand
    engines="gecko"
    name="container"
    sub_properties="container-name container-type"
    gecko_pref="layout.css.container-queries.enabled",
    spec="https://drafts.csswg.org/css-contain-3/#container-shorthand"
>
    pub fn parse_value<'i>(
        context: &ParserContext,
        input: &mut Parser<'i, '_>,
    ) -> Result<Longhands, ParseError<'i>> {
        use crate::parser::Parse;
        use crate::values::specified::box_::{ContainerName, ContainerType};
        // See https://github.com/w3c/csswg-drafts/issues/7180 for why we don't
        // match the spec.
        let container_name = ContainerName::parse(context, input)?;
        let container_type = if input.try_parse(|input| input.expect_delim('/')).is_ok() {
            ContainerType::parse(context, input)?
        } else {
            ContainerType::NONE
        };
        Ok(expanded! {
            container_name: container_name,
            container_type: container_type,
        })
    }

    impl<'a> ToCss for LonghandsToSerialize<'a> {
        fn to_css<W>(&self, dest: &mut CssWriter<W>) -> fmt::Result where W: fmt::Write {
            self.container_name.to_css(dest)?;
            if !self.container_type.is_empty() {
                dest.write_str(" / ")?;
                self.container_type.to_css(dest)?;
            }
            Ok(())
        }
    }
</%helpers:shorthand>

<%helpers:shorthand
    engines="gecko"
    name="page-break-before"
    flags="SHORTHAND_IN_GETCS IS_LEGACY_SHORTHAND"
    sub_properties="break-before"
    spec="https://drafts.csswg.org/css-break-3/#page-break-properties"
>
    pub fn parse_value<'i>(
        context: &ParserContext,
        input: &mut Parser<'i, '_>,
    ) -> Result<Longhands, ParseError<'i>> {
        use crate::values::specified::box_::BreakBetween;
        Ok(expanded! {
            break_before: BreakBetween::parse_legacy(context, input)?,
        })
    }

    impl<'a> ToCss for LonghandsToSerialize<'a> {
        fn to_css<W>(&self, dest: &mut CssWriter<W>) -> fmt::Result where W: fmt::Write {
            self.break_before.to_css_legacy(dest)
        }
    }
</%helpers:shorthand>

<%helpers:shorthand
    engines="gecko"
    name="page-break-after"
    flags="SHORTHAND_IN_GETCS IS_LEGACY_SHORTHAND"
    sub_properties="break-after"
    spec="https://drafts.csswg.org/css-break-3/#page-break-properties"
>
    pub fn parse_value<'i>(
        context: &ParserContext,
        input: &mut Parser<'i, '_>,
    ) -> Result<Longhands, ParseError<'i>> {
        use crate::values::specified::box_::BreakBetween;
        Ok(expanded! {
            break_after: BreakBetween::parse_legacy(context, input)?,
        })
    }

    impl<'a> ToCss for LonghandsToSerialize<'a> {
        fn to_css<W>(&self, dest: &mut CssWriter<W>) -> fmt::Result where W: fmt::Write {
            self.break_after.to_css_legacy(dest)
        }
    }
</%helpers:shorthand>

<%helpers:shorthand
    engines="gecko"
    name="page-break-inside"
    flags="SHORTHAND_IN_GETCS IS_LEGACY_SHORTHAND"
    sub_properties="break-inside"
    spec="https://drafts.csswg.org/css-break-3/#page-break-properties"
>
    pub fn parse_value<'i>(
        context: &ParserContext,
        input: &mut Parser<'i, '_>,
    ) -> Result<Longhands, ParseError<'i>> {
        use crate::values::specified::box_::BreakWithin;
        Ok(expanded! {
            break_inside: BreakWithin::parse_legacy(context, input)?,
        })
    }

    impl<'a> ToCss for LonghandsToSerialize<'a> {
        fn to_css<W>(&self, dest: &mut CssWriter<W>) -> fmt::Result where W: fmt::Write {
            self.break_inside.to_css_legacy(dest)
        }
    }
</%helpers:shorthand>

<%helpers:shorthand name="offset"
                    engines="gecko"
                    sub_properties="offset-path offset-distance offset-rotate offset-anchor"
                    gecko_pref="layout.css.motion-path.enabled",
                    spec="https://drafts.fxtf.org/motion-1/#offset-shorthand">
    use crate::parser::Parse;
    use crate::values::specified::motion::{OffsetPath, OffsetRotate};
    use crate::values::specified::position::PositionOrAuto;
    use crate::values::specified::LengthPercentage;
    use crate::Zero;

    pub fn parse_value<'i, 't>(
        context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<Longhands, ParseError<'i>> {
        // FIXME: Bug 1559232: Support offset-position.
        // Per the spec, this must have offet-position and/or offset-path. However, we don't
        // support offset-position, so offset-path is necessary now.
        let offset_path = OffsetPath::parse(context, input)?;

        let mut offset_distance = None;
        let mut offset_rotate = None;
        loop {
            if offset_distance.is_none() {
                if let Ok(value) = input.try_parse(|i| LengthPercentage::parse(context, i)) {
                    offset_distance = Some(value);
                }
            }

            if offset_rotate.is_none() {
                if let Ok(value) = input.try_parse(|i| OffsetRotate::parse(context, i)) {
                    offset_rotate = Some(value);
                    continue;
                }
            }
            break;
        }

        let offset_anchor = input.try_parse(|i| {
            i.expect_delim('/')?;
            PositionOrAuto::parse(context, i)
        }).ok();

        Ok(expanded! {
            offset_path: offset_path,
            offset_distance: offset_distance.unwrap_or(LengthPercentage::zero()),
            offset_rotate: offset_rotate.unwrap_or(OffsetRotate::auto()),
            offset_anchor: offset_anchor.unwrap_or(PositionOrAuto::auto()),
        })
    }

    impl<'a> ToCss for LonghandsToSerialize<'a>  {
        fn to_css<W>(&self, dest: &mut CssWriter<W>) -> fmt::Result where W: fmt::Write {
            // FIXME: Bug 1559232: Support offset-position. We don't support offset-position,
            // so always serialize offset-path now.
            self.offset_path.to_css(dest)?;

            if !self.offset_distance.is_zero() {
                dest.write_str(" ")?;
                self.offset_distance.to_css(dest)?;
            }

            if !self.offset_rotate.is_auto() {
                dest.write_str(" ")?;
                self.offset_rotate.to_css(dest)?;
            }

            if *self.offset_anchor != PositionOrAuto::auto() {
                dest.write_str(" / ")?;
                self.offset_anchor.to_css(dest)?;
            }
            Ok(())
        }
    }
</%helpers:shorthand>

<%helpers:shorthand name="zoom" engines="gecko"
                    sub_properties="transform transform-origin"
                    gecko_pref="layout.css.zoom-transform-hack.enabled"
                    flags="SHORTHAND_IN_GETCS IS_LEGACY_SHORTHAND"
                    spec="Not a standard, only a compat hack">
    use crate::parser::Parse;
    use crate::values::specified::{Number, NumberOrPercentage, TransformOrigin};
    use crate::values::generics::transform::{Transform, TransformOperation};

    pub fn parse_value<'i, 't>(
        context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<Longhands, ParseError<'i>> {
        let zoom = match input.try_parse(|input| NumberOrPercentage::parse(context, input)) {
            Ok(number_or_percent) => number_or_percent.to_number(),
            Err(..) => {
                input.expect_ident_matching("normal")?;
                Number::new(1.0)
            },
        };

        // Make sure that the initial value matches the values for the
        // longhands, just for general sanity.  `zoom: 1` and `zoom: 0` are
        // ignored, see [1][2]. They are just hack for the "has layout" mode on
        // IE.
        //
        // [1]: https://bugs.webkit.org/show_bug.cgi?id=18467
        // [2]: https://bugzilla.mozilla.org/show_bug.cgi?id=1593009
        Ok(if zoom.get() == 1.0 || zoom.get() == 0.0 {
            expanded! {
                transform: Transform::none(),
                transform_origin: TransformOrigin::initial_value(),
            }
        } else {
            expanded! {
                transform: Transform(vec![TransformOperation::Scale(zoom, zoom)].into()),
                transform_origin: TransformOrigin::zero_zero(),
            }
        })
    }

    impl<'a> ToCss for LonghandsToSerialize<'a>  {
        fn to_css<W>(&self, dest: &mut CssWriter<W>) -> fmt::Result where W: fmt::Write {
            if self.transform.0.is_empty() && *self.transform_origin == TransformOrigin::initial_value() {
                return 1.0f32.to_css(dest);
            }
            if *self.transform_origin != TransformOrigin::zero_zero() {
                return Ok(())
            }
            match &*self.transform.0 {
                [TransformOperation::Scale(x, y)] if x == y => x.to_css(dest),
                _ => Ok(()),
            }
        }
    }
</%helpers:shorthand>
