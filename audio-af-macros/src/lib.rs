use proc_macro::TokenStream;
use syn::{parse_macro_input, Result, DeriveInput, parse_str, Field};
use proc_macro_roids::{DeriveInputStructExt, FieldExt, IdentExt};

#[proc_macro_derive(AafBlock)]
pub fn derive_aaf(item: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(item as DeriveInput);
    let name = input.ident.clone();
    
    let fields = input.fields();
    let parameters = fields.iter().enumerate().filter(|(_, f)| f.type_name().to_string() == "Parameter");

    let mut output = format!("impl {} {{ 
        pub fn num_parameter() -> usize {{ 
            return {}; 
        }}
        
        pub fn list_parameter(&self) -> Vec::<&'static str> {{
            return vec![", name, parameters.clone().collect::<Vec<(usize, &Field)>>().len());
    for (_, f) in parameters.clone() {
        output.push_str(&format!("self.{}.get_name(), ", f.ident.clone().unwrap()));
    }


    output.push_str("        ];
        }
    }");
    let tokens = output.parse().unwrap();
    tokens
}