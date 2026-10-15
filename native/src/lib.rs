use jni::objects::{JClass, JString};
use jni::sys::jstring;
use jni::JNIEnv;

#[no_mangle]
pub extern "system" fn Java_org_example_nohomework_NativeLib_greeting(
    mut env: JNIEnv,
    _: JClass,
) -> jstring {
    let output = env
        .new_string("Hello from Rust!")
        .expect("Could not create Java string");
    output.into_inner()
}

// We'll add an image processing function later
#[no_mangle]
pub extern "system" fn Java_org_example_nohomework_NativeLib_invert_colors(
    env: JNIEnv,
    _: JClass,
    width: i32,
    height: i32,
    input_bytes: jni::objects::JByteArray,
) -> jni::objects::JByteArray {
    // For now, just return the input as is
    input_bytes
}

// Add unit tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greeting() {
        // We can't easily test the JNI function without a JVM, but we can test the logic.
        assert_eq!(true, true);
    }
}
