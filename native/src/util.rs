use anyhow::Result;
use jni::{
    JNIEnv,
    objects::{JObject, JString},
};

use crate::{fromj, intoj};

#[allow(unused)]
pub trait JObjectInto<'a, T> {
    fn intoj(jenv: &mut JNIEnv<'a>, jobject: JObject<'a>) -> Result<T>;
}

#[allow(unused)]
pub trait JObjectFrom<'a, T> {
    fn fromj(jenv: &mut JNIEnv<'a>, t: T) -> Result<JObject<'a>>;
}

fromj!((jenv, t: &str) {
    Ok(jenv.new_string(t)?.into())
});

intoj!((jenv, jobj) -> String {
    let jstring: JString<'a> = jobj.into();
    let java_string = jenv.get_string(&jstring)?;
    Ok(java_string.into())
});

pub mod macros {

    #[macro_export]
    macro_rules! fromj {
        (($jenv:ident, $name:ident: $from:ty) $body:block) => {
            impl<'a> crate::util::JObjectFrom<'a, $from> for $from {
                fn fromj($jenv: &mut JNIEnv<'a>, $name: $from) -> Result<JObject<'a>> $body
            }
        };
    }

    #[macro_export]
    macro_rules! intoj {
        (($jenv:ident, $jobj:ident) -> $into:ty $body:block) => {
            impl<'a> crate::util::JObjectInto<'a, $into> for $into {
                fn intoj($jenv: &mut JNIEnv<'a>, $jobj: JObject<'a>) -> Result<$into> $body
            }
        };
    }
}
