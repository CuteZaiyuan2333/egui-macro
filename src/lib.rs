use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemStruct, Expr, parse::Parser};

#[proc_macro_attribute]
pub fn app(args: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemStruct);
    let app_ident = input.ident.clone();
    let mut title = "Application".to_string();
    let mut width: f32 = 1280.0;
    let mut height: f32 = 720.0;
    let mut min_width: f32 = 640.0;
    let mut min_height: f32 = 360.0;
    
    let args_tokens = proc_macro2::TokenStream::from(args);
    let args_parsed = syn::punctuated::Punctuated::<syn::Meta, syn::Token![,]>::parse_terminated
        .parse2(args_tokens)
        .unwrap_or_default();
    for arg in args_parsed {
        if let syn::Meta::NameValue(nv) = arg {
            let ident = nv.path.get_ident().map(|i| i.to_string()).unwrap_or_default();
            match ident.as_str() {
                "title" => {
                    if let Expr::Lit(syn::ExprLit { lit: syn::Lit::Str(s), .. }) = nv.value {
                        title = s.value();
                    }
                }
                "size" => {
                    if let Expr::Array(arr) = nv.value {
                        if arr.elems.len() == 2 {
                            if let Expr::Lit(syn::ExprLit { lit: syn::Lit::Float(f1), .. }) = &arr.elems[0] {
                                width = f1.base10_parse::<f64>().unwrap_or(width as f64) as f32;
                            } else if let Expr::Lit(syn::ExprLit { lit: syn::Lit::Int(i1), .. }) = &arr.elems[0] {
                                width = i1.base10_parse::<f64>().unwrap_or(width as f64) as f32;
                            }
                            if let Expr::Lit(syn::ExprLit { lit: syn::Lit::Float(f2), .. }) = &arr.elems[1] {
                                height = f2.base10_parse::<f64>().unwrap_or(height as f64) as f32;
                            } else if let Expr::Lit(syn::ExprLit { lit: syn::Lit::Int(i2), .. }) = &arr.elems[1] {
                                height = i2.base10_parse::<f64>().unwrap_or(height as f64) as f32;
                            }
                        }
                    }
                }
                "min_size" => {
                    if let Expr::Array(arr) = nv.value {
                        if arr.elems.len() == 2 {
                            if let Expr::Lit(syn::ExprLit { lit: syn::Lit::Float(f1), .. }) = &arr.elems[0] {
                                min_width = f1.base10_parse::<f64>().unwrap_or(min_width as f64) as f32;
                            } else if let Expr::Lit(syn::ExprLit { lit: syn::Lit::Int(i1), .. }) = &arr.elems[0] {
                                min_width = i1.base10_parse::<f64>().unwrap_or(min_width as f64) as f32;
                            }
                            if let Expr::Lit(syn::ExprLit { lit: syn::Lit::Float(f2), .. }) = &arr.elems[1] {
                                min_height = f2.base10_parse::<f64>().unwrap_or(min_height as f64) as f32;
                            } else if let Expr::Lit(syn::ExprLit { lit: syn::Lit::Int(i2), .. }) = &arr.elems[1] {
                                min_height = i2.base10_parse::<f64>().unwrap_or(min_height as f64) as f32;
                            }
                        }
                    }
                }
                _ => {}
            }
        }
    }
    
    let main_fn = quote! {
        #input
        
        fn main() -> eframe::Result {
            let viewport_options = egui::ViewportBuilder::default()
                .with_inner_size([#width, #height])
                .with_min_inner_size([#min_width, #min_height]);
            let options = eframe::NativeOptions {
                viewport: viewport_options,
                ..Default::default()
            };
            eframe::run_native(
                #title,
                options,
                Box::new(|_cc| Ok(Box::new(#app_ident::default()))),
            )
        }
    };
    main_fn.into()
}
