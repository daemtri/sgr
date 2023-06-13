use proc_macro::TokenStream;
use quote::quote;
use syn::spanned::Spanned;
use syn::{self, DeriveInput, Fields};
use syn::{parse_macro_input, Expr};

#[proc_macro_derive(ServiceRegistryAgent, attributes(driver))]
pub fn service_registry_agent(input: TokenStream) -> TokenStream {
    // 解析输入的类型定义
    let input = parse_macro_input!(input as DeriveInput);
    let enum_name = &input.ident;

    let arms = parse_enum_arms(&input);

    let news = arms.iter().map(|(driver_value, variant_name, variant_ty)| {
        quote! {
            #driver_value => {
                let mut instance = #variant_ty::default();
                sgr_component::init(&mut instance, args);
                Self::#variant_name(instance)
            },
        }
    });

    let finds = arms.iter().map(|(_, variant_name, _)| {
        quote! {
           Self::#variant_name(instance) => {
            sgr_component::ServiceRegistry::find(instance, service_name, id).await
           },
        }
    });
    let lookups = arms.iter().map(|(_, variant_name, _)| {
        quote! {
            Self::#variant_name(instance) => {
                sgr_component::ServiceRegistry::lookup(instance,service_name).await
            },
        }
    });
    let watchs = arms.iter().map(|(_, variant_name, _)| {
        quote! {
            Self::#variant_name(instance) => {
                sgr_component::ServiceRegistry::watch(instance,service_name).await
            },
        }
    });
    // 生成实现代码的 TokenStream
    let expanded = quote! {
        impl #enum_name {
            pub fn load(name: String, args: impl sgr_component::Args) -> Self {
                match name.as_str() {
                    #(#news)*
                    _ => panic!("unknown driver: {}", name),
                }
            }
        }

        #[async_trait::async_trait]
        impl sgr_component::ServiceRegistry for #enum_name {
            async fn find(&self, service_name: String, id: String) -> sgr_component::Result<sgr_component::ServiceEntry> {
                match self {
                    #(#finds)*
                }
            }

            async fn lookup(&self, service_name: String) -> sgr_component::Result<Vec<sgr_component::ServiceEntry>> {
                match self {
                    #(#lookups)*
                }
            }

            async fn watch(&self, service_name: String) -> sgr_component::ResultReceiver<Vec<sgr_component::ServiceEntry>> {
                match self {
                    #(#watchs)*
                }
            }
        }
    };
    TokenStream::from(expanded)
}

#[proc_macro_derive(CofiguratorAgent, attributes(driver))]
pub fn cofigurator_agent(input: TokenStream) -> TokenStream {
    // 解析输入的类型定义
    let input = parse_macro_input!(input as DeriveInput);
    let enum_name = &input.ident;

    let arms = parse_enum_arms(&input);

    let news = arms.iter().map(|(driver_value, variant_name, variant_ty)| {
        quote! {
            #driver_value => {
                let mut instance = #variant_ty::default();
                sgr_component::Component::init(&mut instance, args);
                Self::#variant_name(instance)
            },
        }
    });

    let reads = arms.iter().map(|(_, variant_name, _)| {
        quote! {
            Self::#variant_name(instance) => {
                sgr_component::Cofigurator::read_config(instance,path).await
            },
        }
    });
    let watchs = arms.iter().map(|(_, variant_name, _)| {
        quote! {
            Self::#variant_name(instance) => {
                sgr_component::Cofigurator::watch_config(instance,path).await
            },
        }
    });
    // 生成实现代码的 TokenStream
    let expanded = quote! {
        impl #enum_name {
            pub fn load(name: String, args: impl sgr_component::Args) -> Self {
                match name.as_str() {
                    #(#news)*
                    _ => panic!("unknown driver: {}", name),
                }
            }
        }

        #[async_trait::async_trait]
        impl sgr_component::Cofigurator for #enum_name {
            async fn read_config<T>(&self, path: String) -> sgr_component::Result<T>
            where
                T: serde::de::DeserializeOwned + Default {
                match self {
                    #(#reads)*
                }
            }

            async fn watch_config<T>(&self, path: String) -> sgr_component::ResultReceiver<T>
            where
                T: serde::de::DeserializeOwned + Default {
                match self {
                    #(#watchs)*
                }
            }
        }
    };
    TokenStream::from(expanded)
}

fn parse_enum_arms(input: &DeriveInput) -> Vec<(syn::LitStr, syn::Ident, syn::Field)> {
    // 获取 enum 的所有成员
    let enum_data = if let syn::Data::Enum(data) = &input.data {
        data
    } else {
        panic!("ServiceRegistryAgent only supports enums");
    };

    // 生成 match 语句的 arms
    let arms: Vec<_> = enum_data
        .variants
        .iter()
        .map(|variant| {
            // 获取属性
            let variant_attrs = &variant.attrs;
            let driver_attr = variant_attrs
                .iter()
                .find(|attr| attr.path().is_ident("driver"));

            // 获取 driver 属性的值
            let driver_value = parse_args_attr_value(&driver_attr.unwrap())
                .unwrap()
                .unwrap();

            let variant_name = &variant.ident;
            if let Fields::Unnamed(variant_ty) = &variant.fields {
                // 生成 match arm 的代码
                let variant_ty = variant_ty.unnamed.first().unwrap();
                (driver_value, variant_name.clone(), variant_ty.clone())
            } else {
                panic!("ServiceRegistryAgent only supports tuple variants");
            }
        })
        .collect();
    arms
}

fn parse_args_attr_value(attr: &syn::Attribute) -> Result<Option<syn::LitStr>, syn::Error> {
    if let Some(seg) = attr.path().segments.first() {
        if seg.ident == "driver" {
            let args: syn::Meta = attr.parse_args()?;
            if let syn::Meta::NameValue(values) = args {
                let arg_name = &values.path.segments.first().unwrap().ident;
                if arg_name == "name" {
                    let value = values.value;
                    let lit_str = match value {
                        Expr::Lit(expr_lit) => match expr_lit.lit {
                            syn::Lit::Str(lit_str) => lit_str,
                            _ => panic!("Value is not a string literal"),
                        },
                        _ => panic!("Value is not a literal expression"),
                    };

                    return Ok(Some(lit_str));
                }
                return Err(syn::Error::new(
                    attr.span(),
                    "expected `driver(name = \"...\")`".to_owned(),
                ));
            }
        }
    }
    return Ok(None);
}
