use k256::ecdsa::signature::digest::Digest;
use k256::ecdsa::signature::Signer;
use k256::ecdsa::{Signature, SigningKey};
use sha2::Sha256;

fn main() {
    let data = [1u8];
    // [1]
    println!("data {data:?}");

    let seed = Sha256::digest("test");
    let signing_key = SigningKey::from_bytes(&seed).unwrap();
    let signing_key_vec = signing_key.to_bytes();
    // [159, 134, 208, 129, 136, 76, 125, 101, 154, 47, 234, 160, 197, 90, 208, 21, 163, 191, 79, 27, 43, 11, 130, 44, 209, 93, 108, 21, 176, 240, 10, 8]
    println!("private key {signing_key_vec:?}");

    // [2, 95, 129, 149, 109, 88, 38, 186, 215, 211, 13, 174, 210, 181, 200, 201, 142, 114, 4, 108, 30, 200, 50, 61, 163, 54, 68, 84, 118, 24, 63, 183, 202]
    let public_key = signing_key.verifying_key().to_sec1_bytes();
    println!("public key {public_key:?}");

    let digest = Sha256::digest(data);
    // [75, 245, 18, 47, 52, 69, 84, 197, 59, 222, 46, 187, 140, 210, 183, 227, 209, 96, 10, 214, 49, 195, 133, 165, 215, 204, 226, 60, 119, 133, 69, 154]
    println!("digest {digest:?}");

    let signature: Signature = signing_key.sign(&digest);
    // [92, 7, 89, 85, 210, 43, 67, 5, 188, 43, 186, 151, 244, 29, 95, 102, 103, 180, 245, 158, 254, 76, 134, 136, 172, 54, 237, 170, 47, 212, 6, 243, 29, 180, 186, 169, 92, 112, 234, 33, 231, 228, 14, 154, 192, 125, 206, 74, 144, 160, 219, 87, 201, 100, 210, 73, 198, 236, 210, 139, 5, 131, 239, 96]
    println!("signature {:?}", signature.to_vec());
}
