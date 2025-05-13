use anyhow::{Result, anyhow};
use attribute_derive::FromAttr;
use darling::{FromDeriveInput, FromMeta};
use proc_macro::TokenStream;
use proc_macro_rules::rules;
use quote::{ToTokens, TokenStreamExt, quote};
use syn::DeriveInput;

#[derive(FromMeta, Debug)]
struct JNISigAttribute(String);

#[derive(Debug)]
struct JNIMethodList(Vec<JNIMethodAttribute>);

#[derive(Debug)]
struct JNIMethodAttribute {
    name: syn::Ident,
    sig: syn::LitStr,
    ty: JNIMethodType,
}

#[derive(Debug)]
enum JNIMethodType {
    Void,
    To(syn::Type),
    From(syn::Type),
    FromTo(syn::Type, syn::Type),
}

impl ToTokens for JNIMethodAttribute {
    fn to_tokens(&self, stream: &mut proc_macro2::TokenStream) {
        let fn_name = &self.name;
        let fn_sig = &self.sig;
        stream.append_all(match &self.ty {
            JNIMethodType::Void => quote! {
                fn #fn_name (&self, jenv: &mut JNIEnv<'a>) -> Result<()> {
                    let _ = jenv.call_method(&self.jobject, stringify!(#fn_name), #fn_sig, &[])?;
                    Ok(())
                }
            },
            JNIMethodType::To(to) => quote! {
                fn #fn_name (&self, jenv: &mut JNIEnv<'a>) -> Result<#to> {
                    use carte::util::JObjectInto as _;
                    let result = jenv.call_method(&self.jobject, stringify!(#fn_name), #fn_sig, &[])?;
                    <#to>::intoj(jenv, result.into())
                }
            },
            JNIMethodType::From(from) => quote! {
                fn #fn_name (&self, jenv: &mut JNIEnv<'a>, from: #from) -> Result<()> {
                    use crate::util::JObjectFrom as _;
                    let jobj = &<#from>::fromj(jenv, from)?;
                    let _ = jenv.call_method(&self.jobject, stringify!(#fn_name), #fn_sig, &[
                        jobj.into()
                    ])?;
                    Ok(())
                }
            },
            JNIMethodType::FromTo(from, to) => quote! {
                fn #fn_name (&self, jenv: &mut JNIEnv<'a>, from: #from) -> Result<#to> {
                    use crate::util::{JObjectFrom as _, JObjectInto as _};
                    let jobj = &<#from>::fromj(jenv, from)?;
                    let result = jenv.call_method(&self.jobject, stringify!(#fn_name), #fn_sig, &[
                        jobj.into()
                    ])?;
                    <#to>::intoj(jenv, result.into())
                }
            },
        });
    }
}

impl FromMeta for JNIMethodList {
    fn from_list(items: &[darling::ast::NestedMeta]) -> darling::Result<Self> {
        Ok(Self(
            items
                .iter()
                .map(|i| {
                    rules!(i.into_token_stream() => {
                        ($name:ident($lit:literal)) => {
                            let sig = syn::parse2(lit.to_token_stream()).unwrap();
                            JNIMethodAttribute { name,
                                sig,
                                ty: JNIMethodType::Void
                            }
                        }
                        ($name:ident($lit:literal, $from:ty -> ())) => {
                            let sig = syn::parse2(lit.to_token_stream()).unwrap();
                            JNIMethodAttribute { name,
                                sig,
                                ty: JNIMethodType::From(from)
                            }
                        }
                        ($name:ident($lit:literal, () -> $to:ty)) => {
                            let sig = syn::parse2(lit.to_token_stream()).unwrap();
                            JNIMethodAttribute { name,
                                sig,
                                ty: JNIMethodType::To(to)
                            }
                        }
                        ($name:ident($lit:literal, $from:ty -> $to:ty)) => {
                            let sig = syn::parse2(lit.to_token_stream()).unwrap();
                            JNIMethodAttribute { name,
                                sig,
                                ty: JNIMethodType::FromTo(from, to)
                            }
                        }
                    })
                })
                .collect::<Vec<JNIMethodAttribute>>(),
        ))
    }
}

#[derive(FromDeriveInput, Debug)]
#[darling(attributes(jni_wrapper), forward_attrs)]
struct JNIWrapperDeriveOpts {
    ident: syn::Ident,
    generics: syn::Generics,
    // attrs: Vec<syn::Attribute>,
    sig: JNISigAttribute,
    methods: JNIMethodList,
}

#[proc_macro_derive(JNIWrapper, attributes(jni_wrapper))]
pub fn jni_wrapper_derive(tokens: TokenStream) -> TokenStream {
    _jni_wrapper_derive(tokens).unwrap()
}

fn _jni_wrapper_derive(tokens: TokenStream) -> Result<TokenStream> {
    let opts = JNIWrapperDeriveOpts::from_derive_input(&DeriveInput::from_input(tokens)?)
        .map_err(|e| anyhow!(e.to_string()))?;

    let JNIWrapperDeriveOpts {
        ident,
        generics: a_t,
        // attrs,
        sig: JNISigAttribute(sig),
        methods: JNIMethodList(methods),
    } = opts;

    let a = a_t
        .lifetimes()
        .next()
        .ok_or(anyhow!("Expected at least one lifetime parameter!"))?;

    Ok(quote! {
        impl #a_t #ident #a_t {
            fn new(jenv: &mut JNIEnv<#a>, jobject: JObject<#a>) -> Result<Self> {
                let expected_class = jenv.find_class(#sig)?;
                let object_class = jenv.get_object_class(&jobject)?;

                if !jenv.is_assignable_from(object_class, expected_class)? {
                    Err(anyhow!("argument jobject has incorrect inheritance"))
                } else {
                    Ok(Self { jobject })
                }
            }

            #(#methods)*
        }
    }
    .into())
}

#[proc_macro]
pub fn jni_wrapper(tokens: TokenStream) -> TokenStream {
    _jni_wrapper(tokens).unwrap()
}

fn _jni_wrapper(tokens: TokenStream) -> Result<TokenStream> {
    rules!(tokens.into() => {
        (name: $name:ident, sig: $sig:literal $(, methods = [])?) => {
            Ok(quote! {
                #[derive(JNIWrapper)]
                #[jni_wrapper(
                    sig = #sig,
                    methods()
                )]
                struct #name <'a> {
                    jobject: JObject<'a>,
                }
            }.into())
        }
        (name: $name:ident, sig: $sig:literal, methods: $methods:tt) => {
            let group = if let proc_macro2::TokenTree::Group(group) = methods { Ok(group) } else { Err(anyhow!("wrong tt"))}?;
            let inner = group.stream();
            Ok(quote! {
                #[derive(JNIWrapper)]
                #[jni_wrapper(
                    sig = #sig,
                    methods(#inner)
                )]
                struct #name <'a> {
                    jobject: JObject<'a>,
                }
            }.into())
        }
    })
}
