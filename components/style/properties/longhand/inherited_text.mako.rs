/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

<%namespace name="helpers" file="/helpers.mako.rs" />

<% data.new_style_struct("InheritedText", inherited=True, gecko_ffi_name="nsStyleText") %>

<%helpers:longhand name="line-height">
    use cssparser::ToCss;
    use std::fmt;
    use values::AuExtensionMethods;
    use values::CSSFloat;

    #[derive(Debug, Clone, PartialEq, Copy, HeapSizeOf)]
    pub enum SpecifiedValue {
        Normal,
        Number(CSSFloat),
        LengthOrPercentage(specified::LengthOrPercentage),
    }

    impl ToCss for SpecifiedValue {
        fn to_css<W>(&self, dest: &mut W) -> fmt::Result where W: fmt::Write {
            match *self {
                SpecifiedValue::Normal => dest.write_str("normal"),
                SpecifiedValue::LengthOrPercentage(value) => value.to_css(dest),
                SpecifiedValue::Number(number) => write!(dest, "{}", number),
            }
        }
    }
    /// normal | <number> | <length> | <percentage>
    pub fn parse(_context: &ParserContext, input: &mut Parser) -> Result<SpecifiedValue, ()> {
        use cssparser::Token;
        use std::ascii::AsciiExt;
        input.try(specified::LengthOrPercentage::parse_non_negative)
        .map(SpecifiedValue::LengthOrPercentage)
        .or_else(|()| {
            match try!(input.next()) {
                Token::Number(ref value) if value.value >= 0. => {
                    Ok(SpecifiedValue::Number(value.value))
                }
                Token::Ident(ref value) if value.eq_ignore_ascii_case("normal") => {
                    Ok(SpecifiedValue::Normal)
                }
                _ => Err(()),
            }
        })
    }
    pub mod computed_value {
        use app_units::Au;
        use std::fmt;
        use values::CSSFloat;
        #[derive(PartialEq, Copy, Clone, HeapSizeOf, Debug)]
        pub enum T {
            Normal,
            Length(Au),
            Number(CSSFloat),
        }
    }
    impl ToCss for computed_value::T {
        fn to_css<W>(&self, dest: &mut W) -> fmt::Result where W: fmt::Write {
            match *self {
                computed_value::T::Normal => dest.write_str("normal"),
                computed_value::T::Length(length) => length.to_css(dest),
                computed_value::T::Number(number) => write!(dest, "{}", number),
            }
        }
    }
     #[inline]
    pub fn get_initial_value() -> computed_value::T { computed_value::T::Normal }

    impl ToComputedValue for SpecifiedValue {
        type ComputedValue = computed_value::T;

        #[inline]
        fn to_computed_value<Cx: TContext>(&self, context: &Cx) -> computed_value::T {
            match *self {
                SpecifiedValue::Normal => computed_value::T::Normal,
                SpecifiedValue::Number(value) => computed_value::T::Number(value),
                SpecifiedValue::LengthOrPercentage(value) => {
                    match value {
                        specified::LengthOrPercentage::Length(value) =>
                            computed_value::T::Length(value.to_computed_value(context)),
                        specified::LengthOrPercentage::Percentage(specified::Percentage(value)) => {
                            let fr = specified::Length::FontRelative(specified::FontRelativeLength::Em(value));
                            computed_value::T::Length(fr.to_computed_value(context))
                        },
                        specified::LengthOrPercentage::Calc(calc) => {
                            let calc = calc.to_computed_value(context);
                            let fr = specified::FontRelativeLength::Em(calc.percentage());
                            let fr = specified::Length::FontRelative(fr);
                            computed_value::T::Length(calc.length() + fr.to_computed_value(context))
                        }
                    }
                }
            }
        }
    }
</%helpers:longhand>

<%helpers:longhand name="text-align">
    pub use self::computed_value::T as SpecifiedValue;
    use values::computed::ComputedValueAsSpecified;
    impl ComputedValueAsSpecified for SpecifiedValue {}
    pub mod computed_value {
        macro_rules! define_text_align {
            ( $( $name: ident ( $string: expr ) => $discriminant: expr, )+ ) => {
                define_css_keyword_enum! { T:
                    $(
                        $string => $name,
                    )+
                }
                impl T {
                    pub fn to_u32(self) -> u32 {
                        match self {
                            $(
                                T::$name => $discriminant,
                            )+
                        }
                    }
                    pub fn from_u32(discriminant: u32) -> Option<T> {
                        match discriminant {
                            $(
                                $discriminant => Some(T::$name),
                            )+
                            _ => None
                        }
                    }
                }
            }
        }
        define_text_align! {
            start("start") => 0,
            end("end") => 1,
            left("left") => 2,
            right("right") => 3,
            center("center") => 4,
            justify("justify") => 5,
            servo_center("-servo-center") => 6,
            servo_left("-servo-left") => 7,
            servo_right("-servo-right") => 8,
        }
    }
    #[inline] pub fn get_initial_value() -> computed_value::T {
        computed_value::T::start
    }
    pub fn parse(_context: &ParserContext, input: &mut Parser)
                 -> Result<SpecifiedValue, ()> {
        computed_value::T::parse(input)
    }
</%helpers:longhand>

<%helpers:longhand name="letter-spacing">
    use cssparser::ToCss;
    use std::fmt;
    use values::AuExtensionMethods;

    #[derive(Debug, Clone, Copy, PartialEq, HeapSizeOf)]
    pub enum SpecifiedValue {
        Normal,
        Specified(specified::Length),
    }

    impl ToCss for SpecifiedValue {
        fn to_css<W>(&self, dest: &mut W) -> fmt::Result where W: fmt::Write {
            match *self {
                SpecifiedValue::Normal => dest.write_str("normal"),
                SpecifiedValue::Specified(l) => l.to_css(dest),
            }
        }
    }

    pub mod computed_value {
        use app_units::Au;
        #[derive(Debug, Clone, PartialEq, HeapSizeOf)]
        pub struct T(pub Option<Au>);
    }

    impl ToCss for computed_value::T {
        fn to_css<W>(&self, dest: &mut W) -> fmt::Result where W: fmt::Write {
            match self.0 {
                None => dest.write_str("normal"),
                Some(l) => l.to_css(dest),
            }
        }
    }

    #[inline]
    pub fn get_initial_value() -> computed_value::T {
        computed_value::T(None)
    }

    impl ToComputedValue for SpecifiedValue {
        type ComputedValue = computed_value::T;

        #[inline]
        fn to_computed_value<Cx: TContext>(&self, context: &Cx) -> computed_value::T {
            match *self {
                SpecifiedValue::Normal => computed_value::T(None),
                SpecifiedValue::Specified(l) =>
                    computed_value::T(Some(l.to_computed_value(context)))
            }
        }
    }

    pub fn parse(_context: &ParserContext, input: &mut Parser) -> Result<SpecifiedValue, ()> {
        if input.try(|input| input.expect_ident_matching("normal")).is_ok() {
            Ok(SpecifiedValue::Normal)
        } else {
            specified::Length::parse_non_negative(input).map(SpecifiedValue::Specified)
        }
    }
</%helpers:longhand>

<%helpers:longhand name="word-spacing">
    use cssparser::ToCss;
    use std::fmt;
    use values::AuExtensionMethods;

    #[derive(Debug, Clone, Copy, PartialEq, HeapSizeOf)]
    pub enum SpecifiedValue {
        Normal,
        Specified(specified::Length),  // FIXME(SimonSapin) support percentages
    }

    impl ToCss for SpecifiedValue {
        fn to_css<W>(&self, dest: &mut W) -> fmt::Result where W: fmt::Write {
            match *self {
                SpecifiedValue::Normal => dest.write_str("normal"),
                SpecifiedValue::Specified(l) => l.to_css(dest),
            }
        }
    }

    pub mod computed_value {
        use app_units::Au;
        #[derive(Debug, Clone, PartialEq, HeapSizeOf)]
        pub struct T(pub Option<Au>);
    }

    impl ToCss for computed_value::T {
        fn to_css<W>(&self, dest: &mut W) -> fmt::Result where W: fmt::Write {
            match self.0 {
                None => dest.write_str("normal"),
                Some(l) => l.to_css(dest),
            }
        }
    }

    #[inline]
    pub fn get_initial_value() -> computed_value::T {
        computed_value::T(None)
    }

    impl ToComputedValue for SpecifiedValue {
        type ComputedValue = computed_value::T;

        #[inline]
        fn to_computed_value<Cx: TContext>(&self, context: &Cx) -> computed_value::T {
            match *self {
                SpecifiedValue::Normal => computed_value::T(None),
                SpecifiedValue::Specified(l) =>
                    computed_value::T(Some(l.to_computed_value(context)))
            }
        }
    }

    pub fn parse(_context: &ParserContext, input: &mut Parser) -> Result<SpecifiedValue, ()> {
        if input.try(|input| input.expect_ident_matching("normal")).is_ok() {
            Ok(SpecifiedValue::Normal)
        } else {
            specified::Length::parse_non_negative(input).map(SpecifiedValue::Specified)
        }
    }
</%helpers:longhand>

${helpers.predefined_type("text-indent",
                          "LengthOrPercentage",
                          "computed::LengthOrPercentage::Length(Au(0))")}

// Also known as "word-wrap" (which is more popular because of IE), but this is the preferred
// name per CSS-TEXT 6.2.
${helpers.single_keyword("overflow-wrap",
                         "normal break-word",
                         gecko_ffi_name="mWordWrap",
                         gecko_constant_prefix="NS_STYLE_WORDWRAP")}

// TODO(pcwalton): Support `word-break: keep-all` once we have better CJK support.
${helpers.single_keyword("word-break",
                         "normal break-all",
                         extra_gecko_values="keep-all",
                         gecko_constant_prefix="NS_STYLE_WORDBREAK")}

// TODO(pcwalton): Support `text-justify: distribute`.
${helpers.single_keyword("text-justify",
                         "auto none inter-word",
                         products="servo")}

<%helpers:longhand name="-servo-text-decorations-in-effect"
                derived_from="display text-decoration" need_clone="True" products="servo">
    use cssparser::{RGBA, ToCss};
    use std::fmt;

    use values::computed::ComputedValueAsSpecified;
    use properties::style_struct_traits::{Box, Color, Text};

    impl ComputedValueAsSpecified for SpecifiedValue {}

    #[derive(Clone, PartialEq, Copy, Debug, HeapSizeOf)]
    pub struct SpecifiedValue {
        pub underline: Option<RGBA>,
        pub overline: Option<RGBA>,
        pub line_through: Option<RGBA>,
    }

    pub mod computed_value {
        pub type T = super::SpecifiedValue;
    }

    impl ToCss for SpecifiedValue {
        fn to_css<W>(&self, _: &mut W) -> fmt::Result where W: fmt::Write {
            // Web compat doesn't matter here.
            Ok(())
        }
    }

    #[inline]
    pub fn get_initial_value() -> computed_value::T {
        SpecifiedValue {
            underline: None,
            overline: None,
            line_through: None,
        }
    }

    fn maybe<Cx: TContext>(flag: bool, context: &Cx) -> Option<RGBA> {
        if flag {
            Some(context.style().get_color().clone_color())
        } else {
            None
        }
    }

    fn derive<Cx: TContext>(context: &Cx) -> computed_value::T {
        // Start with no declarations if this is an atomic inline-level box; otherwise, start with the
        // declarations in effect and add in the text decorations that this block specifies.
        let mut result = match context.style().get_box().clone_display() {
            super::display::computed_value::T::inline_block |
            super::display::computed_value::T::inline_table => SpecifiedValue {
                underline: None,
                overline: None,
                line_through: None,
            },
            _ => context.inherited_style().get_inheritedtext().clone__servo_text_decorations_in_effect()
        };

        result.underline = maybe(context.style().get_text().has_underline()
                                 || result.underline.is_some(), context);
        result.overline = maybe(context.style().get_text().has_overline()
                                || result.overline.is_some(), context);
        result.line_through = maybe(context.style().get_text().has_line_through()
                                    || result.line_through.is_some(), context);

        result
    }

    #[inline]
    pub fn derive_from_text_decoration<Cx: TContext>(context: &mut Cx) {
        let derived = derive(context);
        context.mutate_style().mutate_inheritedtext().set__servo_text_decorations_in_effect(derived);
    }

    #[inline]
    pub fn derive_from_display<Cx: TContext>(context: &mut Cx) {
        let derived = derive(context);
        context.mutate_style().mutate_inheritedtext().set__servo_text_decorations_in_effect(derived);
    }
</%helpers:longhand>

<%helpers:single_keyword_computed name="white-space" values="normal pre nowrap pre-wrap pre-line",
                                  gecko_constant_prefix="NS_STYLE_WHITESPACE">
    use values::computed::ComputedValueAsSpecified;
    impl ComputedValueAsSpecified for SpecifiedValue {}

    impl SpecifiedValue {
        pub fn allow_wrap(&self) -> bool {
            match *self {
                SpecifiedValue::nowrap |
                SpecifiedValue::pre => false,
                SpecifiedValue::normal |
                SpecifiedValue::pre_wrap |
                SpecifiedValue::pre_line => true,
            }
        }

        pub fn preserve_newlines(&self) -> bool {
            match *self {
                SpecifiedValue::normal |
                SpecifiedValue::nowrap => false,
                SpecifiedValue::pre |
                SpecifiedValue::pre_wrap |
                SpecifiedValue::pre_line => true,
            }
        }

        pub fn preserve_spaces(&self) -> bool {
            match *self {
                SpecifiedValue::normal |
                SpecifiedValue::nowrap |
                SpecifiedValue::pre_line => false,
                SpecifiedValue::pre |
                SpecifiedValue::pre_wrap => true,
            }
        }
    }
</%helpers:single_keyword_computed>

<%helpers:longhand name="text-shadow">
    use cssparser::{self, ToCss};
    use std::fmt;
    use values::AuExtensionMethods;

    #[derive(Clone, PartialEq, Debug, HeapSizeOf)]
    pub struct SpecifiedValue(Vec<SpecifiedTextShadow>);

    #[derive(Clone, PartialEq, Debug, HeapSizeOf)]
    pub struct SpecifiedTextShadow {
        pub offset_x: specified::Length,
        pub offset_y: specified::Length,
        pub blur_radius: specified::Length,
        pub color: Option<specified::CSSColor>,
    }

    pub mod computed_value {
        use app_units::Au;
        use cssparser::Color;

        #[derive(Clone, PartialEq, Debug, HeapSizeOf)]
        pub struct T(pub Vec<TextShadow>);

        #[derive(Clone, PartialEq, Debug, HeapSizeOf)]
        pub struct TextShadow {
            pub offset_x: Au,
            pub offset_y: Au,
            pub blur_radius: Au,
            pub color: Color,
        }
    }

    impl ToCss for computed_value::T {
        fn to_css<W>(&self, dest: &mut W) -> fmt::Result where W: fmt::Write {
            let mut iter = self.0.iter();
            if let Some(shadow) = iter.next() {
                try!(shadow.to_css(dest));
            } else {
                try!(dest.write_str("none"));
                return Ok(())
            }
            for shadow in iter {
                try!(dest.write_str(", "));
                try!(shadow.to_css(dest));
            }
            Ok(())
        }
    }

    impl ToCss for computed_value::TextShadow {
        fn to_css<W>(&self, dest: &mut W) -> fmt::Result where W: fmt::Write {
            try!(self.offset_x.to_css(dest));
            try!(dest.write_str(" "));
            try!(self.offset_y.to_css(dest));
            try!(dest.write_str(" "));
            try!(self.blur_radius.to_css(dest));
            try!(dest.write_str(" "));
            try!(self.color.to_css(dest));
            Ok(())
        }
    }

    impl ToCss for SpecifiedValue {
        fn to_css<W>(&self, dest: &mut W) -> fmt::Result where W: fmt::Write {
            let mut iter = self.0.iter();
            if let Some(shadow) = iter.next() {
                try!(shadow.to_css(dest));
            } else {
                try!(dest.write_str("none"));
                return Ok(())
            }
            for shadow in iter {
                try!(dest.write_str(", "));
                try!(shadow.to_css(dest));
            }
            Ok(())
        }
    }

    impl ToCss for SpecifiedTextShadow {
        fn to_css<W>(&self, dest: &mut W) -> fmt::Result where W: fmt::Write {
            try!(self.offset_x.to_css(dest));
            try!(dest.write_str(" "));
            try!(self.offset_y.to_css(dest));
            try!(dest.write_str(" "));
            try!(self.blur_radius.to_css(dest));

            if let Some(ref color) = self.color {
                try!(dest.write_str(" "));
                try!(color.to_css(dest));
            }
            Ok(())
        }
    }

    #[inline]
    pub fn get_initial_value() -> computed_value::T {
        computed_value::T(Vec::new())
    }

    pub fn parse(_: &ParserContext, input: &mut Parser) -> Result<SpecifiedValue,()> {
        if input.try(|input| input.expect_ident_matching("none")).is_ok() {
            Ok(SpecifiedValue(Vec::new()))
        } else {
            input.parse_comma_separated(parse_one_text_shadow).map(SpecifiedValue)
        }
    }

    fn parse_one_text_shadow(input: &mut Parser) -> Result<SpecifiedTextShadow,()> {
        use app_units::Au;
        let mut lengths = [specified::Length::Absolute(Au(0)); 3];
        let mut lengths_parsed = false;
        let mut color = None;

        loop {
            if !lengths_parsed {
                if let Ok(value) = input.try(specified::Length::parse) {
                    lengths[0] = value;
                    let mut length_parsed_count = 1;
                    while length_parsed_count < 3 {
                        if let Ok(value) = input.try(specified::Length::parse) {
                            lengths[length_parsed_count] = value
                        } else {
                            break
                        }
                        length_parsed_count += 1;
                    }

                    // The first two lengths must be specified.
                    if length_parsed_count < 2 {
                        return Err(())
                    }

                    lengths_parsed = true;
                    continue
                }
            }
            if color.is_none() {
                if let Ok(value) = input.try(specified::CSSColor::parse) {
                    color = Some(value);
                    continue
                }
            }
            break
        }

        // Lengths must be specified.
        if !lengths_parsed {
            return Err(())
        }

        Ok(SpecifiedTextShadow {
            offset_x: lengths[0],
            offset_y: lengths[1],
            blur_radius: lengths[2],
            color: color,
        })
    }

    impl ToComputedValue for SpecifiedValue {
        type ComputedValue = computed_value::T;

        fn to_computed_value<Cx: TContext>(&self, context: &Cx) -> computed_value::T {
            computed_value::T(self.0.iter().map(|value| {
                computed_value::TextShadow {
                    offset_x: value.offset_x.to_computed_value(context),
                    offset_y: value.offset_y.to_computed_value(context),
                    blur_radius: value.blur_radius.to_computed_value(context),
                    color: value.color
                                .as_ref()
                                .map(|color| color.parsed)
                                .unwrap_or(cssparser::Color::CurrentColor),
                }
            }).collect())
        }
    }
</%helpers:longhand>



// TODO(pcwalton): `full-width`
${helpers.single_keyword("text-transform",
                         "none capitalize uppercase lowercase",
                         extra_gecko_values="full-width")}

${helpers.single_keyword("text-rendering", "auto optimizespeed optimizelegibility geometricprecision")}

// CSS Text Module Level 3
// https://www.w3.org/TR/css-text-3/
${helpers.single_keyword("hyphens", "none manual auto", products="gecko")}

// CSS Ruby Layout Module Level 1
// https://www.w3.org/TR/css-ruby-1/
${helpers.single_keyword("ruby-align", "start center space-between space-around", products="gecko")}

${helpers.single_keyword("ruby-position", "over under", products="gecko")}
