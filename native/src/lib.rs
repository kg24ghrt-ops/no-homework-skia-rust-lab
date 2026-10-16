use jni::objects::{JByteArray, JClass, JObject};
use jni::sys::jstring;
use jni::JNIEnv;

pub mod image;

#[cfg(feature = "skia")]
pub mod skia;

#[no_mangle]
pub extern "system" fn Java_org_example_nohomework_MainActivity_greeting(
    env: JNIEnv,
    _: JClass,
) -> jstring {
    let output = env
        .new_string("Hello from Rust!")
        .expect("Could not create Java string");
    output.into_raw()
}

#[no_mangle]
pub extern "system" fn Java_org_example_nohomework_MainActivity_invertColors<'a>(
    mut env: JNIEnv<'a>,
    _: JClass<'a>,
    width: i32,
    height: i32,
    input_bytes: JByteArray<'a>,
) -> JByteArray<'a> {
    let len = match env.get_array_length(&input_bytes) {
        Ok(l) => l as usize,
        Err(_) => return null_byte_array(),
    };

    let mut buf: Vec<jni::sys::jbyte> = vec![0; len];
    match env.get_byte_array_region(&input_bytes, 0, &mut buf) {
        Ok(()) => {}
        Err(_) => return null_byte_array(),
    }

    let mut bytes: Vec<u8> = buf.into_iter().map(|x| x as u8).collect();

    match image::invert_colors(&mut bytes, width, height) {
        Ok(()) => match env.byte_array_from_slice(&bytes) {
            Ok(a) => a,
            Err(_) => null_byte_array(),
        },
        Err(e) => {
            let _ = env.throw_new("java/lang/IllegalArgumentException", e.to_string());
            null_byte_array()
        }
    }
}

#[cfg(feature = "skia")]
#[no_mangle]
pub extern "system" fn Java_org_example_nohomework_MainActivity_generateNotebookPaper<'a>(
    mut env: JNIEnv<'a>,
    _: JClass<'a>,
    width: i32,
    height: i32,
    line_spacing: f32,
    margin_width: f32,
) -> JByteArray<'a> {
    match skia::generate_notebook_paper_png(width, height, line_spacing, margin_width) {
        Ok(png) => match env.byte_array_from_slice(&png) {
            Ok(a) => a,
            Err(_) => null_byte_array(),
        },
        Err(e) => {
            let _ = env.throw_new("java/lang/IllegalArgumentException", e.to_string());
            null_byte_array()
        }
    }
}

fn null_byte_array() -> JByteArray<'static> {
    unsafe { JByteArray::from_raw(JObject::null().as_raw()) }
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
        assert_eq!(invert_colors(&mut [], 0, 5), Err(ImageError::ZeroDimension));
    }
}
