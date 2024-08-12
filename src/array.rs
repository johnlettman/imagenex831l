use std::io::Cursor;

#[derive(Debug, Clone)]
#[cfg_attr(
    target_family = "wasm",
    derive(tsify::Tsify, serde::Serialize, serde::Deserialize),
    tsify(into_wasm_abi, from_wasm_abi)
)]
pub struct Array {
    buffer: Vec<u8>,
    cursor: Cursor<Vec<u8>>,
}

impl Array {
    pub fn new(buffer: Vec<u8>) -> Self {
        let cursor = Cursor::new(buffer.clone());
        Self { buffer, cursor }
    }
}
