use serialize::base64::{self, ToBase64};
use serialize::hex::FromHex;

pub fn hex_to_base64(hex: String) -> String {
    let hex_array: Vec<u8> = hex.from_hex().expect("failed to make into hex");
    hex_array.to_base64(base64::STANDARD)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hex_to_b64_test() {
        let expected_b64 = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t".to_string();
        let converted_hex = hex_to_base64("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d".to_string());
        assert_eq!(converted_hex, expected_b64)
    }
}

