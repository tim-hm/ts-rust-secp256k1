
## Setup

```shell
npm install
cargo update
```
## Issue

All values are identical except for the signature. Both produce signatures with 64 elements.

## Rust output

- `$ cargo run`

```text
data        [1]
private key [159, 134, 208, 129, 136, 76, 125, 101, 154, 47, 234, 160, 197, 90, 208, 21, 163, 191, 79, 27, 43, 11, 130, 44, 209, 93, 108, 21, 176, 240, 10, 8]
public key  [2, 95, 129, 149, 109, 88, 38, 186, 215, 211, 13, 174, 210, 181, 200, 201, 142, 114, 4, 108, 30, 200, 50, 61, 163, 54, 68, 84, 118, 24, 63, 183, 202]
digest      [75, 245, 18, 47, 52, 69, 84, 197, 59, 222, 46, 187, 140, 210, 183, 227, 209, 96, 10, 214, 49, 195, 133, 165, 215, 204, 226, 60, 119, 133, 69, 154]
signature   [92, 7, 89, 85, 210, 43, 67, 5, 188, 43, 186, 151, 244, 29, 95, 102, 103, 180, 245, 158, 254, 76, 134, 136, 172, 54, 237, 170, 47, 212, 6, 243, 29, 180, 186, 169, 92, 112, 234, 33, 231, 228, 14, 154, 192, 125, 206, 74, 144, 160, 219, 87, 201, 100, 210, 73, 198, 236, 210, 139, 5, 131, 239, 96]
```

## Typescript output

- `$ npm run dev`

```text
data        [1]
private key [159, 134, 208, 129, 136, 76, 125, 101, 154, 47, 234, 160, 197, 90, 208, 21, 163, 191, 79, 27, 43, 11, 130, 44, 209, 93, 108, 21, 176, 240, 10, 8]
public key  [2, 95, 129, 149, 109, 88, 38, 186, 215, 211, 13, 174, 210, 181, 200, 201, 142, 114, 4, 108, 30, 200, 50, 61, 163, 54, 68, 84, 118, 24, 63, 183, 202]
digest      [75, 245, 18, 47, 52, 69, 84, 197, 59, 222, 46, 187, 140, 210, 183, 227, 209, 96, 10, 214, 49, 195, 133, 165, 215, 204, 226, 60, 119, 133, 69, 154]
signature   [22, 156, 180, 160, 31, 25, 179, 99, 9, 52, 138, 203, 25, 173, 189, 253, 2, 163, 23, 137, 45, 20, 202, 173, 171, 82, 198, 145, 245, 209, 138, 14, 52, 77, 242, 21, 52, 57, 9, 196, 178, 206, 66, 105, 97, 116, 240, 199, 105, 179, 90, 222, 145, 26, 16, 71, 246, 49, 247, 208, 22, 19, 8, 84]
```
