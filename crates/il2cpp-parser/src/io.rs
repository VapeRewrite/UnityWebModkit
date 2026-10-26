use crate::error::{Error, OutOfBoundsErr, Result};
use alloc::string::String;
use alloc::vec::Vec;
use wasm_bindgen::prelude::wasm_bindgen;

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct BinaryStream {
	data: Vec<u8>,
	pos: u64,
}

impl BinaryStream {
	pub fn new(data: Vec<u8>) -> Self {
		Self { data, pos: 0 }
	}

	pub fn len(&self) -> u64 {
		self.data.len() as u64
	}

	pub fn position(&self) -> u64 {
		self.pos
	}

	pub fn set_position(&mut self, pos: u64) {
		self.pos = pos;
	}

	pub fn remaining_from(&self, offset: u64) -> u64 {
		self.len().saturating_sub(offset)
	}

	pub fn read_u8(&mut self) -> Result<u8> {
		if self.pos as usize >= self.data.len() {
			return Err(Error::OutOfBounds(OutOfBoundsErr {
				offset: self.pos,
				size: 1,
			}));
		}
		let v = self.data[self.pos as usize];
		self.pos += 1;
		Ok(v)
	}

	pub fn read_u16(&mut self) -> Result<u16> {
		let start = self.pos as usize;
		if start + 2 > self.data.len() {
			return Err(Error::OutOfBounds(OutOfBoundsErr {
				offset: self.pos,
				size: 2,
			}));
		}
		self.pos += 2;
		Ok(u16::from_le_bytes([self.data[start], self.data[start + 1]]))
	}

	pub fn read_i16(&mut self) -> Result<i16> {
		Ok(self.read_u16()? as i16)
	}

	pub fn read_u32(&mut self) -> Result<u32> {
		let start = self.pos as usize;
		if start + 4 > self.data.len() {
			return Err(Error::OutOfBounds(OutOfBoundsErr {
				offset: self.pos,
				size: 4,
			}));
		}
		self.pos += 4;
		Ok(u32::from_le_bytes([
			self.data[start],
			self.data[start + 1],
			self.data[start + 2],
			self.data[start + 3],
		]))
	}

	pub fn read_i32(&mut self) -> Result<i32> {
		Ok(self.read_u32()? as i32)
	}

	pub fn read_u64(&mut self) -> Result<u64> {
		let start = self.pos as usize;
		if start + 8 > self.data.len() {
			return Err(Error::OutOfBounds(OutOfBoundsErr {
				offset: self.pos,
				size: 8,
			}));
		}
		self.pos += 8;
		Ok(u64::from_le_bytes([
			self.data[start],
			self.data[start + 1],
			self.data[start + 2],
			self.data[start + 3],
			self.data[start + 4],
			self.data[start + 5],
			self.data[start + 6],
			self.data[start + 7],
		]))
	}

	pub fn peek_string_to_null_at(&self, offset: u64) -> Result<String> {
		let mut i = offset as usize;
		if i >= self.data.len() {
			return Err(Error::OutOfBounds(OutOfBoundsErr { offset, size: 1 }));
		}
		let start = i;
		while i < self.data.len() && self.data[i] != 0 {
			i += 1;
		}
		Ok(String::from_utf8_lossy(&self.data[start..i]).into_owned())
	}

	pub fn peek_bytes_at(&self, offset: u64, len: usize) -> Result<&[u8]> {
		let s = offset as usize;
		if s + len > self.data.len() {
			return Err(Error::OutOfBounds(OutOfBoundsErr { offset, size: len }));
		}
		Ok(&self.data[s..s + len])
	}

	pub fn read_i32_array(&mut self, offset: u64, count: usize) -> Result<Vec<i32>> {
		if count == 0 {
			return Ok(Vec::new());
		}
		let old = self.pos;
		self.pos = offset;
		let mut result = Vec::with_capacity(count);
		for _ in 0..count {
			result.push(self.read_i32()?);
		}
		self.pos = old;
		Ok(result)
	}
}
