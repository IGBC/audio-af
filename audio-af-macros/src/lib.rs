use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput, Field};
use proc_macro_roids::{DeriveInputStructExt, FieldExt};
use quote::quote;

#[proc_macro_derive(AafBlock)]
pub fn derive_aaf(item: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(item as DeriveInput);
    let name = input.ident.clone();
    
    let fields = input.fields();
    let parameters = fields.iter().enumerate().filter(|(_, f)| f.type_name().to_string() == "Parameter");

    let num_param = parameters.clone().collect::<Vec<(usize, &Field)>>().len();
    let params = parameters.clone().map(|(_,f)| { f.ident.clone().unwrap() });

    let tokens = quote! {
        impl #name {
            pub fn num_parameter() -> usize {
                return #num_param;
            }

            pub fn list_parameter(&self) -> Vec::<&'static str> {
                return vec! [ #( self.#params.get_name(), )* ];
            }
        }
    };
    
    TokenStream::from(tokens)
}
