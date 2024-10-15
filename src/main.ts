import {secp256k1} from "@noble/curves/secp256k1";
import {sha256} from "@noble/hashes/sha256";

function main() {
  const data = Uint8Array.from([0x1]);
  // [1]
  console.log("data [", data.map(v => v).join(", "), "]");

  const privateKey = sha256("test");
  // [159, 134, 208, 129, 136, 76, 125, 101, 154, 47, 234, 160, 197, 90, 208, 21, 163, 191, 79, 27, 43, 11, 130, 44, 209, 93, 108, 21, 176, 240, 10, 8]
  console.log("private key [", privateKey.map(v => v).join(", "), "]");

  const digest = sha256(data);
  // [75, 245, 18, 47, 52, 69, 84, 197, 59, 222, 46, 187, 140, 210, 183, 227, 209, 96, 10, 214, 49, 195, 133, 165, 215, 204, 226, 60, 119, 133, 69, 154]
  console.log("digest [", digest.map(v => v).join(", "), "]");

  const signature = secp256k1.sign(digest, privateKey).toCompactRawBytes();
  // [22, 156, 180, 160, 31, 25, 179, 99, 9, 52, 138, 203, 25, 173, 189, 253, 2, 163, 23, 137, 45, 20, 202, 173, 171, 82, 198, 145, 245, 209, 138, 14, 52, 77, 242, 21, 52, 57, 9, 196, 178, 206, 66, 105, 97, 116, 240, 199, 105, 179, 90, 222, 145, 26, 16, 71, 246, 49, 247, 208, 22, 19, 8, 84]
  console.log("signature [", signature.map(v => v).join(", "), "]");
}

main()
