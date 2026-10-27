use wasm_bindgen::prelude::wasm_bindgen;

/// Write a varuint32 to a buffer
/// Takes ownership and returns the buffer so wasm-bindgen doesn't goobener
#[wasm_bindgen(js_name = "write_var_uint32_to_buffer")]
pub fn write_var_uint32_to_buffer_w(mut buffer: Vec<u8>, value: u32) -> Vec<u8> {
	write_var_uint32_to_buffer(&mut buffer, value);
	buffer
}

/// Write a varuint32 to a buffer
pub fn write_var_uint32_to_buffer(buffer: &mut Vec<u8>, value: u32) {
	if value == 0 {
		buffer.push(0);
		return;
	}

	let mut v = value;
	let mut bytes: Vec<u8> = Vec::new();

	// Extract groups LSB first, setting continuation bit on all but the last
	loop {
		let byte = (v & 0x7f) | 0x80;
		bytes.push(byte as u8);
		v >>= 7;
		if v == 0 {
			// Last byte: clear continuation bit
			*bytes.last_mut().unwrap() &= 0x7f;
			break;
		}
	}

	// bytes are in LSB first order, which is the correct order for varuint32
	buffer.extend_from_slice(&bytes);
}
