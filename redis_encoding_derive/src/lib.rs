use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(ToRedisArgs)]
pub fn to_redis_args_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_to_redis_args(&ast)
}

fn impl_to_redis_args(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
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

#[proc_macro_derive(FromRedisValue)]
pub fn from_redis_value_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_from_redis_value(&ast)
}

fn impl_from_redis_value(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl redis::FromRedisValue for #name {
            fn from_redis_value(v: &redis::Value) -> redis::RedisResult<Self> {
                // match Nil
                if let redis::Value::Data(bs) = v {
                    let res = serde_json::from_slice::<Self>(bs).unwrap();
                    return Ok(res);
                }
                if let redis::Value::Nil = v {
                    return Err((redis::ErrorKind::ResponseError,"can not find key").into())
                }
                panic!("redis value is not vec<u8>")
            }
        }
    };
    gen.into()
}
