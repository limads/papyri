pub mod model;

#[cfg(feature="gdk4")]
#[cfg(feature="gdk-pixbuf")]
#[cfg(feature="cairo-rs")]
pub mod render;

#[cfg(feature="gdk4")]
#[cfg(feature="gdk-pixbuf")]
#[cfg(feature="cairo-rs")]
pub mod ffi {

    use std::os::raw::c_char;

    const INVALID_UTF8 : i64 = 1;

    const INSUFFICIENT_LEN : i64 = 2;

    const INVALID_DEFINITION : i64 = 3;

    #[no_mangle]
    extern "C" fn error(code : i64, out : *mut u8, out_len : &mut usize) -> i64 {
        let mut out = unsafe { std::slice::from_raw_parts_mut(out, *out_len) };
        let msg = match code {
            INVALID_UTF8 => "Invalid UTF-8",
            INSUFFICIENT_LEN => "Insufficient length",
            _ => "Unknown error"
        };
        if let Some(s) = out.get_mut(..msg.len()) {
            s.copy_from_slice(msg.as_bytes());
            *out_len = s.len();
            0
        } else {
            -1
        }
    }

    #[no_mangle]
    extern "C" fn svg(model : *const u8, model_len : usize, out : *mut u8, out_len : &mut usize) -> i64 {
        let model = unsafe { std::slice::from_raw_parts(model, model_len) };
        match std::str::from_utf8(model) {
            Ok(s) => {
                match crate::render::Panel::new_from_json(s) {
                    Ok(mut pl) => {
                        match pl.svg() {
                            Ok(svg) => {
                                let out = unsafe { std::slice::from_raw_parts_mut(out, *out_len) };
                                match out.get_mut(..svg.len()) {
                                    Some(s) => {
                                        s.copy_from_slice(svg.as_bytes());
                                        *out_len = s.len();
                                        0
                                    },
                                    None => {
                                        INSUFFICIENT_LEN
                                    }
                                }
                            },
                            Err(_) => {
                                INVALID_DEFINITION
                            }
                        }
                    },
                    Err(_) => {
                        INVALID_DEFINITION
                    }
                }
            },
            Err(_) => {
                INVALID_UTF8
            }
        }
    }

}



