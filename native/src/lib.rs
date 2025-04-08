use anyhow::{Result, anyhow};
use jni::{JNIEnv, objects::JObject};
use macros::JNIWrapper;
mod util;

fn main<'local>(jenv: &mut JNIEnv<'local>, callback: JNICallback<'local>) -> Result<()> {
    tracing::info!("rust loaded!");

    callback.info(jenv, "Hello :3")?;

    Ok(())
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "system" fn Java_me_wawwior_oxid_jni_JNI_main<'local>(
    jenv: JNIEnv<'local>,
    _jobject: JObject<'local>,
    callback: JObject<'local>,
) {
    tracing_subscriber::fmt().init();

    let _ = with_result(jenv, callback).inspect_err(|err| tracing::error!("{}", err));
}

fn with_result<'local>(mut jenv: JNIEnv<'local>, callback: JObject<'local>) -> Result<()> {
    let callback = JNICallback::new(&mut jenv, callback)?;

    main(&mut jenv, callback)
}

jni_wrapper! {
    name: JNIBinding,
    sig:  "me/wawwior/oxid/jni/JNI"
}

#[derive(JNIWrapper)]
#[jni_wrapper(
    sig = "me/wawwior/oxid/jni/JNICallback",
    methods(info("(Ljava/lang/String;)V", &str -> ()))
)]
struct JNICallback<'a> {
    jobject: JObject<'a>,
}
