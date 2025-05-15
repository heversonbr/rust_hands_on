use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_derive(HelloMacro)]
pub fn proc_macro_derive(input: TokenStream) -> TokenStream {

    // Construct the representation of Rust code as a Syntax tree
    // that can be manipulated 
    let ast = syn::parse(input).unwrap();

    // build the trait implementation
    impl_hello_macro(&ast)
}


fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream{

    let name = &ast.ident;
    let gen = quote! {

        impl HelloMacro for #name{
            fn hello_macro(){
                println!("Hello Macro! My name is {}!", stringify!(#name));
            }
        }
    };

    gen.into()

}