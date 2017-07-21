/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

//! Gecko's definition of a pseudo-element.
//!
//! Note that a few autogenerated bits of this live in
//! `pseudo_element_definition.mako.rs`. If you touch that file, you probably
//! need to update the checked-in files for Servo.

use cssparser::{ToCss, serialize_identifier};
use gecko_bindings::structs::{self, CSSPseudoElementType};
use properties::{PropertyFlags, APPLIES_TO_FIRST_LETTER, APPLIES_TO_FIRST_LINE};
use properties::APPLIES_TO_PLACEHOLDER;
use selector_parser::{NonTSPseudoClass, PseudoElementCascadeType, SelectorImpl};
use std::fmt;
use string_cache::Atom;

include!(concat!(env!("OUT_DIR"), "/gecko/pseudo_element_definition.rs"));

impl ::selectors::parser::PseudoElement for PseudoElement {
    type Impl = SelectorImpl;

    fn supports_pseudo_class(&self, pseudo_class: &NonTSPseudoClass) -> bool {
        if !self.supports_user_action_state() {
            return false;
        }

        return pseudo_class.is_safe_user_action_state();
    }
}

impl PseudoElement {
    /// Returns the kind of cascade type that a given pseudo is going to use.
    ///
    /// In Gecko we only compute ::before and ::after eagerly. We save the rules
    /// for anonymous boxes separately, so we resolve them as precomputed
    /// pseudos.
    ///
    /// We resolve the others lazily, see `Servo_ResolvePseudoStyle`.
    pub fn cascade_type(&self) -> PseudoElementCascadeType {
        if self.is_eager() {
            debug_assert!(!self.is_anon_box());
            return PseudoElementCascadeType::Eager
        }

        if self.is_anon_box() {
            return PseudoElementCascadeType::Precomputed
        }

        PseudoElementCascadeType::Lazy
    }

    /// Whether the pseudo-element should inherit from the default computed
    /// values instead of from the parent element.
    ///
    /// This is not the common thing, but there are some pseudos (namely:
    /// ::backdrop), that shouldn't inherit from the parent element.
    pub fn inherits_from_default_values(&self) -> bool {
        matches!(*self, PseudoElement::Backdrop)
    }

    /// Gets the canonical index of this eagerly-cascaded pseudo-element.
    #[inline]
    pub fn eager_index(&self) -> usize {
        EAGER_PSEUDOS.iter().position(|p| p == self)
            .expect("Not an eager pseudo")
    }

    /// Creates a pseudo-element from an eager index.
    #[inline]
    pub fn from_eager_index(i: usize) -> Self {
        EAGER_PSEUDOS[i].clone()
    }

    /// Whether this pseudo-element is ::before or ::after.
    #[inline]
    pub fn is_before_or_after(&self) -> bool {
        matches!(*self, PseudoElement::Before | PseudoElement::After)
    }

    /// Whether this pseudo-element is ::first-letter.
    #[inline]
    pub fn is_first_letter(&self) -> bool {
        *self == PseudoElement::FirstLetter
    }

    /// Whether this pseudo-element is ::-moz-fieldset-content.
    #[inline]
    pub fn is_fieldset_content(&self) -> bool {
        *self == PseudoElement::FieldsetContent
    }

    /// Whether this pseudo-element is lazily-cascaded.
    #[inline]
    pub fn is_lazy(&self) -> bool {
        !self.is_eager() && !self.is_precomputed()
    }

    /// Whether this pseudo-element is web-exposed.
    pub fn exposed_in_non_ua_sheets(&self) -> bool {
        (self.flags() & structs::CSS_PSEUDO_ELEMENT_UA_SHEET_ONLY) == 0
    }

    /// Whether this pseudo-element supports user action selectors.
    pub fn supports_user_action_state(&self) -> bool {
        (self.flags() & structs::CSS_PSEUDO_ELEMENT_SUPPORTS_USER_ACTION_STATE) != 0
    }

    /// Whether this pseudo-element is precomputed.
    #[inline]
    pub fn is_precomputed(&self) -> bool {
        self.is_anon_box()
    }

    /// Covert non-canonical pseudo-element to canonical one, and keep a
    /// canonical one as it is.
    pub fn canonical(&self) -> PseudoElement {
        match *self {
            PseudoElement::MozPlaceholder => PseudoElement::Placeholder,
            _ => self.clone(),
        }
    }

    /// Property flag that properties must have to apply to this pseudo-element.
    #[inline]
    pub fn property_restriction(&self) -> Option<PropertyFlags> {
        match *self {
            PseudoElement::FirstLetter => Some(APPLIES_TO_FIRST_LETTER),
            PseudoElement::FirstLine => Some(APPLIES_TO_FIRST_LINE),
            PseudoElement::Placeholder => Some(APPLIES_TO_PLACEHOLDER),
            _ => None,
        }
    }
}
