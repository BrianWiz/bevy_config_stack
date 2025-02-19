use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Data, Fields};

#[proc_macro_derive(ConfigDocs, attributes(config_path))]
pub fn derive_config_docs(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    
    let path = input.attrs.iter()
        .find(|attr| attr.path().is_ident("config_path"))
        .map(|attr| {
            if let syn::Meta::NameValue(nv) = &attr.meta {
                if let syn::Expr::Lit(syn::ExprLit { lit: syn::Lit::Str(s), .. }) = &nv.value {
                    s.value()
                } else {
                    String::new()
                }
            } else {
                String::new()
            }
        })
        .unwrap_or_default();

    let fields = match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => &fields.named,
            _ => return TokenStream::new(),
        },
        _ => return TokenStream::new(),
    };

    let field_count = fields.iter().count();
    let field_docs = fields.iter().enumerate().map(|(i, field)| {
        let field_name = &field.ident;
        let field_type = &field.ty;
        let docs = field.attrs.iter()
            .filter(|attr| attr.path().is_ident("doc"))
            .filter_map(|attr| {
                if let syn::Meta::NameValue(nv) = &attr.meta {
                    if let syn::Expr::Lit(syn::ExprLit { lit: syn::Lit::Str(s), .. }) = &nv.value {
                        Some(s.value())
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        let spacing = if i < field_count - 1 { "\n" } else { "" };
        
        quote! {
            #(docs.push_str(&format!("  /// {}\n", #docs.trim()));)*
            docs.push_str(&format!("  /// - type: {}\n", stringify!(#field_type)));
            docs.push_str(&format!("  {}: {:?},{}\n", stringify!(#field_name), default.#field_name, #spacing));
        }
    });

    let struct_docs = input.attrs.iter()
        .filter(|attr| attr.path().is_ident("doc"))
        .filter_map(|attr| {
            if let syn::Meta::NameValue(nv) = &attr.meta {
                if let syn::Expr::Lit(syn::ExprLit { lit: syn::Lit::Str(s), .. }) = &nv.value {
                    Some(s.value())
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    // Optionally implement ConfigAsset trait if there's a config_path attribute value set
    let config_impl = if !path.is_empty() {
        Some(quote! {
            use bevy_config_stack::ConfigAsset;
            impl ConfigAsset for #name {
                const CONFIG_PATH: &'static str = #path;
            }
        })
    } else {
        None
    };

    TokenStream::from(quote! {
        impl #name {
            pub fn print_docs() {
                let default = Self::default();
                let mut docs = String::new();
                #(docs.push_str(&format!("/// {}\n", #struct_docs.trim()));)*
                if !#path.is_empty() {
                    docs.push_str(&format!("/// - type: Asset\n"));
                    docs.push_str(&format!("/// - path: {}\n", #path));
                } else {
                    docs.push_str(&format!("/// - type: Nested\n"));
                }
                docs.push_str(&format!("{} (\n", stringify!(#name)));
                #(#field_docs)*
                docs.push_str(")\n");
                println!("{}", docs);
            }
        }

        #config_impl
    })
}
