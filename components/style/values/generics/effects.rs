/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

//! Generic types for CSS values related to effects.

#[cfg(feature = "gecko")]
use values::specified::url::SpecifiedUrl;

/// A generic value for a single `filter`.
#[cfg_attr(feature = "servo", derive(Deserialize, HeapSizeOf, Serialize))]
#[derive(Clone, Debug, HasViewportPercentage, PartialEq, ToComputedValue, ToCss)]
pub enum Filter<Angle, Factor, Length, DropShadow> {
    /// `blur(<length>)`
    #[css(function)]
    Blur(Length),
    /// `brightness(<factor>)`
    #[css(function)]
    Brightness(Factor),
    /// `contrast(<factor>)`
    #[css(function)]
    Contrast(Factor),
    /// `grayscale(<factor>)`
    #[css(function)]
    Grayscale(Factor),
    /// `hue-rotate(<angle>)`
    #[css(function)]
    HueRotate(Angle),
    /// `invert(<factor>)`
    #[css(function)]
    Invert(Factor),
    /// `opacity(<factor>)`
    #[css(function)]
    Opacity(Factor),
    /// `saturate(<factor>)`
    #[css(function)]
    Saturate(Factor),
    /// `sepia(<factor>)`
    #[css(function)]
    Sepia(Factor),
    /// `drop-shadow(...)`
    #[css(function)]
    DropShadow(DropShadow),
    /// `<url>`
    #[cfg(feature = "gecko")]
    Url(SpecifiedUrl),
}

/// A generic value for the `drop-shadow()` filter and the `text-shadow` property.
///
/// Contrary to the canonical order from the spec, the color is serialised
/// first, like in Gecko and Webkit.
#[cfg_attr(feature = "servo", derive(HeapSizeOf))]
#[derive(Clone, Debug, HasViewportPercentage, PartialEq, ToCss)]
pub struct SimpleShadow<Color, SizeLength, ShapeLength> {
    /// Color.
    pub color: Color,
    /// Horizontal radius.
    pub horizontal: SizeLength,
    /// Vertical radius.
    pub vertical: SizeLength,
    /// Blur radius.
    pub blur: ShapeLength,
}
