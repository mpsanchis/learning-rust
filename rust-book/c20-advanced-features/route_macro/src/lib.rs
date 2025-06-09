use proc_macro::TokenStream;
use quote::{quote, format_ident};
use syn::{parse_macro_input, Lit, ItemFn};
use syn::Meta;
use syn::parse::Parser;
use syn::punctuated::Punctuated;
use syn::token::Comma;

/// Procedural macro: #[route(GET, "/")]
#[proc_macro_attribute]
pub fn route(attr: TokenStream, item: TokenStream) -> TokenStream {
    // Parse macro args: #[route(GET, "/")]
    let parser = Punctuated::<Meta, Comma>::parse_terminated;
    let args = parser.parse(attr).expect("Failed to parse macro args");

    // Parse the input function
    let input_fn = parse_macro_input!(item as ItemFn);
    let fn_name = &input_fn.sig.ident;

    // Parse method and path from args
    let method = match &args[0] {
        Meta::Path(path) => path.get_ident().unwrap().to_string(),
        _ => panic!("Expected method like GET"),
    };

    let path = match &args[1] {
      Meta::NameValue(meta) => {
        let name = &meta.path.get_ident().expect("Expected identifier").to_string();
        if name != "path" {
          panic!("Expected attribute 'path' but got {name}");
        }

        if let syn::Expr::Lit(expr_lit) = &meta.value {
            if let Lit::Str(litstr) = &expr_lit.lit {
                litstr.value()
            } else {
                panic!("Expected a string literal for the path");
            }
        } else {
            panic!("Expected a literal expression for the path");
        }
      }
      Meta::Path(_) => panic!("Path must be a name-value pair, like path = \"/\""),
      _ => panic!("Could not parse path attribute"),
  };

    let register_fn_name = format_ident!("register_{}", fn_name);

    let expanded = quote! {
      #input_fn

      #[ctor::ctor]
      fn #register_fn_name() {
        crate::macros::register_route(#method, #path, #fn_name);
      }
    };

    TokenStream::from(expanded)
}
