use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use std::collections::HashMap;

#[proc_macro_attribute]
pub fn to_redis(_param: TokenStream, input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let mut ast = syn::parse_macro_input!(input as syn::ItemStruct);

    // Build the trait implementation
    impl_to_redis_args(&mut ast)
}

fn common(attr: &[syn::Attribute]) -> Vec<String> {
    let mut hm = [
        ("Serialize", false),
        ("Deserialize", false),
        ("Debug", false),
    ]
    .iter()
    .map(|(k, v)| (k.to_string(), *v))
    .collect::<HashMap<String, bool>>();
    for a in attr.iter() {
        if let syn::Meta::List(list) = &a.meta {
            if list.path.to_token_stream().to_string() == "derive" {
                list.tokens.to_string().split(",").for_each(|x| {
                    hm.entry(x.to_string()).and_modify(|v| *v = true);
                });
            }
        }
    }
    hm.iter()
        .filter(|(_, v)| !**v)
        .map(|(k, _)| k.to_string())
        .collect()
}

fn impl_to_redis_args(ast: &mut syn::ItemStruct) -> TokenStream {
    for x in common(&ast.attrs) {
        let t = syn::parse_str::<syn::Type>(&x).unwrap();
        let a: syn::Attribute = syn::parse_quote!(#[derive(#t)]);
        ast.attrs.push(a)
    }
    let name = &ast.ident;
    let gen = quote! {
        #ast
        impl redis::ToRedisArgs for #name {
            fn write_redis_args<W>(&self, out: &mut W)
            where
                W: ?Sized + redis::RedisWrite {
                // let res = &serde_json::to_string(self).unwrap();
                let json_str = serde_json::to_string(self).unwrap();
                let bs = json_str.as_bytes();
                out.write_arg(bs)
            }
        }
    };
    gen.into()
}

#[proc_macro_attribute]
pub fn from_redis(_param: TokenStream, input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let mut ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_from_redis_value(&mut ast)
}

fn impl_from_redis_value(ast: &mut syn::DeriveInput) -> TokenStream {
    for x in common(&ast.attrs) {
        let t = syn::parse_str::<syn::Type>(&x).unwrap();
        let a: syn::Attribute = syn::parse_quote!(#[derive(#t)]);
        ast.attrs.push(a)
    }
    let name = &ast.ident;
    let gen = quote! {
        #ast
        impl redis::FromRedisValue for #name {
            fn from_redis_value(v: &redis::Value) -> redis::RedisResult<Self> {
                match v {
                    redis::Value::Data(bs)=>{
                        let res = serde_json::from_slice::<Self>(bs).unwrap();
                        return Ok(res);
                    }
                    redis::Value::Nil=>{
                        return Err((redis::ErrorKind::ResponseError,"can not find key").into())
                    }
                    _=>{
                        panic!("redis value is not vec<u8>")
                    }
                }
            }
        }
    };
    gen.into()
}
