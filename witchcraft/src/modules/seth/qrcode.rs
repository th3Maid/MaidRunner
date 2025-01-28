use crate::core::core::*;

use image::Luma;
use qrcode::QrCode;

/// Generates a QR code based on the provided command line arguments.
///
/// This function expects a slice of strings (`argsv`) where the first element is typically the program name
/// and the second element (`argsv[1]`) should be the data to encode in the QR code.
///
/// If the data is successfully processed, the function generates a QR code and saves it as an image file in the current directory.
///
/// # Arguments
///
/// * `argsv` - A slice of `String` references representing the command line arguments.
///             The first element is usually the program name, and the second element is the data to be encoded.
///
/// # Returns
///
/// * `i32` - Returns `0` if successful, or `1` if there was an error (e.g., missing argument or failure in generating the QR code).
///
/// # Example
///
/// ```
/// let args = vec!["qrcode".to_string(), "\"Hello, QR!\"".to_string()];
/// gen_qrcode_from_argsv(&args);
/// ```
pub fn gen_qrcode_from_argsv(argsv: &[String]) -> i32 {
    let data = search_value("data", argsv);
    let code = match QrCode::new(data.as_bytes()) {
        Ok(val) => val,
        Err(err) => {
            raise(&err.to_string(), "done");
            return 1;
        }
    };

    let now = datetime_now();
    let image = code.render::<Luma<u8>>().build();
    let home = get_os_env("HOME");
    lazy_exec(format!("mkdir {}/Downloads/", &home));
    image
        .save(format!("{}/Downloads/qrcode-{now}.png", home))
        .unwrap();

    let string = code.render().light_color(' ').dark_color('#').build();
    raise(&string, "");
    0
}
