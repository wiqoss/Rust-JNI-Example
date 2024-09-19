extern crate jni;

use jni::objects::{JClass, JObject, JString};
use jni::JNIEnv;

#[no_mangle]
pub extern "system" fn Java_org_wiqs_Main_run(
    mut env: JNIEnv,
    _class: JClass,
    person: JObject,
) {
    let name_obj = env.call_method(&person, "getName", "()Ljava/lang/String;", &[])
        .expect("Couldn't call getName")
        .l()
        .expect("Invalid return for getName");

    let age = env.call_method(&person, "getAge", "()I", &[])
        .expect("Couldn't call getAge")
        .i()
        .expect("Invalid return for getAge");

    let name: String = env.get_string(&JString::from(name_obj))
        .expect("Couldn't get Java string!")
        .into();

    println!("Hello, {}! You are {} years old", name, age)
}
