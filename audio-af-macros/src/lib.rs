use proc_macro::TokenStream;
use syn::{parse_macro_input, Result, DeriveInput, parse_str};
use proc_macro_roids::{DeriveInputStructExt, FieldExt, IdentExt};

#[proc_macro_derive(AafBlock)]
pub fn derive_aaf(item: TokenStream) -> TokenStream {
    print!("TokenSteam: ");
    println!("{}", item);
    let proc_item = item.clone();
    let input: DeriveInput = parse_macro_input!(proc_item as DeriveInput);
    println!("{:?}", input);
    let fields = input.fields();
    let parameters = fields.iter().enumerate().filter(|(_, f)| f.type_name().to_string() == "Parameter");
    for (i, f) in parameters {
        println!("{}, {:?}", i, f);
    }

    item
}