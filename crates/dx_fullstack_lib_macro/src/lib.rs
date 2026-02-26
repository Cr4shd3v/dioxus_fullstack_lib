mod signal_container;

extern crate proc_macro;

use proc_macro::TokenStream;
use crate::signal_container::derive_signal_collection_impl;

#[proc_macro_derive(SignalContainer)]
pub fn derive_signal_collection(input: TokenStream) -> TokenStream {
    derive_signal_collection_impl(input)
}