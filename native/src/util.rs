use std::marker::PhantomData;

use jni::{JNIEnv, objects::JObject};

pub trait JObjectInto<'a, T> {
    fn intoj(jenv: &mut JNIEnv<'a>, jobject: JObject<'a>) -> T;
}
pub trait JObjectFrom<'a, T> {
    fn fromj(jenv: &mut JNIEnv<'a>, t: T) -> JObject<'a>;
}

pub struct JObjectCasters<T> {
    phantom: PhantomData<T>,
}

impl<'a> JObjectFrom<'a, &str> for JObjectCasters<&str> {
    fn fromj(jenv: &mut JNIEnv<'a>, t: &str) -> JObject<'a> {
        jenv.new_string(t).unwrap().into()
    }
}

pub mod macros {

    #[macro_export]
    macro_rules! jni_wrapper {
        (name: $name:ident, sig: $sig:literal) => {
            #[derive(JNIWrapper)]
            #[jni_wrapper(sig = $sig, methods())]
            struct $name<'a> {
                jobject: JObject<'a>,
            }
        };
    }
}
