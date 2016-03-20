// Set 1
use serialize::base64::{self, ToBase64};
use serialize::hex::{FromHex, ToHex};

// Challenge 1
pub fn hex_to_base64(hex: String) -> String {
    let hex_array: Vec<u8> = hex.from_hex().expect("failed to make into hex");
    hex_array.to_base64(base64::STANDARD)
}

// Challenge 2
pub fn xor(l: String, r: String) -> String {
    let l_hex: Vec<u8> = l.from_hex().expect("failed to make into hex");
    let r_hex: Vec<u8> = r.from_hex().expect("failed to make into hex");
    let xored_bytes: Vec<u8> = l_hex.iter().zip(r_hex.iter()).map(|(l_byte, r_byte)| l_byte ^ r_byte).collect::<Vec<_>>();
    xored_bytes.to_hex()
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

    #[test]
    fn xor_test() {
        let l = "1c0111001f010100061a024b53535009181c".to_string();
        let r = "686974207468652062756c6c277320657965".to_string();
        let exp = "746865206b696420646f6e277420706c6179".to_string();
        assert_eq!(xor(l, r), exp)
    }
}

