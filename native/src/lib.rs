use jni::{JNIEnv, objects::JObject};
use macros::hello;

fn main() {
    println!("hello from native!");
    hello!();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
fn Java_me_wawwior_oxid_jni_JNI_main(jni_env: JNIEnv<'static>, jobject: JObject<'static>) {
    let _ = jobject;
    let _ = jni_env;
    main();
}
