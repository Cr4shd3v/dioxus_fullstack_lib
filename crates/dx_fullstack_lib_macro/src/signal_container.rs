use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput};

pub(super) fn derive_signal_collection_impl(input: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(input as DeriveInput);
    let Data::Struct(struct_data) = derive_input.data else {
        panic!("Only structs are supported")
    };
    
    let ident = derive_input.ident;
    let mut new_fields = quote! {};
    let mut reset_fields = quote! {};

    for field in struct_data.fields.iter() {
        let field_name = field.ident.as_ref().unwrap();
        
        new_fields.extend(quote! {
            #field_name: dioxus::prelude::use_signal(Default::default),
        });

        reset_fields.extend(quote! {
            self.#field_name.set(Default::default());
        });
    }
    
    quote! {
        impl dx_fullstack_lib::SignalContainer for #ident {
            fn new() -> Self where Self: Sized {
                Self {
                    #new_fields
                }
            }

            fn reset(&mut self) {
                #reset_fields
            }
        }
    }.into()
}