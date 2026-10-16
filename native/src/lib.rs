use jni::objects::{JByteArray, JClass};
use jni::sys::jstring;
use jni::JNIEnv;

pub mod image;

#[no_mangle]
pub extern "system" fn Java_org_example_nohomework_MainActivity_greeting(
    mut env: JNIEnv,
    _: JClass,
) -> jstring {
    let output = env
        .new_string("Hello from Rust!")
        .expect("Could not create Java string");
    output.into_inner()
}

#[no_mangle]
pub extern "system" fn Java_org_example_nohomework_MainActivity_invertColors(
    mut env: JNIEnv,
    _: JClass,
    width: i32,
    height: i32,
    input_bytes: JByteArray,
) -> JByteArray {
    let len = match env.get_array_length(input_bytes) {
        Ok(l) => l as usize,
        Err(_) => return JByteArray::null(),
    };

    let bytes: Vec<u8> = match env.get_byte_array_region(input_bytes, 0, len as i32) {
        Ok(b) => b.into_iter().map(|x| x as u8).collect(),
        Err(_) => return JByteArray::null(),
    };

    match image::invert_colors(&mut bytes, width, height) {
        Ok(()) => match env.byte_array_from_slice(&bytes) {
            Ok(a) => a,
            Err(_) => JByteArray::null(),
        },
        Err(e) => {
            let _ = env.throw_new("java/lang/IllegalArgumentException", e.to_string());
            JByteArray::null()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::image::*;

    #[test]
    fn lib_delegates_to_image_invert_colors() {
        let mut buffer = vec![10, 20, 30, 200];
        assert!(invert_colors(&mut buffer, 1, 1).is_ok());
        assert_eq!(buffer, [245, 235, 225, 200]);
    }

    #[test]
    fn lib_validates_dimensions_before_processing() {
        assert_eq!(
            invert_colors(&mut [], -1, 5),
            Err(ImageError::NegativeDimension)
        );
        assert_eq!(
            invert_colors(&mut [], 0, 5),
            Err(ImageError::ZeroDimension)
        );
    }
}
