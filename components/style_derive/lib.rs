/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

extern crate proc_macro;
#[macro_use] extern crate quote;
extern crate syn;
extern crate synstructure;

use proc_macro::TokenStream;

mod compute_squared_distance;
mod has_viewport_percentage;
mod to_animated_value;
mod to_computed_value;
mod to_css;

#[proc_macro_derive(ComputeSquaredDistance)]
pub fn derive_compute_squared_distance(stream: TokenStream) -> TokenStream {
    let input = syn::parse_derive_input(&stream.to_string()).unwrap();
    compute_squared_distance::derive(input).to_string().parse().unwrap()
}

#[proc_macro_derive(HasViewportPercentage)]
pub fn derive_has_viewport_percentage(stream: TokenStream) -> TokenStream {
    let input = syn::parse_derive_input(&stream.to_string()).unwrap();
    has_viewport_percentage::derive(input).to_string().parse().unwrap()
}

#[proc_macro_derive(ToAnimatedValue)]
pub fn derive_to_animated_value(stream: TokenStream) -> TokenStream {
    let input = syn::parse_derive_input(&stream.to_string()).unwrap();
    to_animated_value::derive(input).to_string().parse().unwrap()
}

#[proc_macro_derive(ToComputedValue)]
pub fn derive_to_computed_value(stream: TokenStream) -> TokenStream {
    let input = syn::parse_derive_input(&stream.to_string()).unwrap();
    to_computed_value::derive(input).to_string().parse().unwrap()
}

#[proc_macro_derive(ToCss, attributes(css))]
pub fn derive_to_css(stream: TokenStream) -> TokenStream {
    let input = syn::parse_derive_input(&stream.to_string()).unwrap();
    to_css::derive(input).to_string().parse().unwrap()
}
