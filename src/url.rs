use std::io::Write;

use hmac::{Hmac, Mac};
use sha1::Sha1;

type HmacSha1 = Hmac<Sha1>;

/// ```
/// use gapdecoder::compute_url;
/// let path = "wGcDNN8L-2COcm9toX5BTp6HPxpMPPPuxrMU-ZL-W-nDHW8I_L4R5vlBJ6ITtlmONQ";
/// let token = "KwCgJ1QIfgprHn0a93x7Q-HhJ04";
/// assert_eq!(
///     compute_url(path, token, 0, 0, 7),
///     "https://lh3.googleusercontent.com/wGcDNN8L-2COcm9toX5BTp6HPxpMPPPuxrMU-ZL-W-nDHW8I_L4R5vlBJ6ITtlmONQ=x0-y0-z7-tHeJ3xylnSyyHPGwMZimI4EV3JP8"
/// );
pub fn compute_url<T, U>(path: T, token: U, x: u32, y: u32, z: u32) -> String
    where T: AsRef<str>,
          U: AsRef<str> {
    let mut url = format!(
        "https://lh3.googleusercontent.com/{}=x{}-y{}-z{}-t",
        path.as_ref(), x, y, z
    );

    let mut sign_path: Vec<u8> = Vec::new();
    sign_path.extend_from_slice(path.as_ref().as_bytes());
    write!(sign_path, "=x{}-y{}-z{}-t", x, y, z).unwrap();
    sign_path.extend_from_slice(token.as_ref().as_bytes());

    let digest = mac_digest(&sign_path);
    base64::encode_config_buf(&digest, base64::URL_SAFE_NO_PAD, &mut url);
    url
}

fn mac_digest(b: &[u8]) -> Vec<u8> {
    let key = &[123, 43, 78, 35, 222, 44, 197, 197];
    let mut mac = HmacSha1::new_varkey(key).expect("HMac keys can have any length");
    mac.input(b);
    mac.result().code().to_vec()
}