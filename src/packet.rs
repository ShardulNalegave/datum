
// ===== Imports =====
use bytes::Bytes;

use crate::header::Header;
// ===================

pub struct Packet {
  pub header: Header,
}

impl From<Bytes> for Packet {
  fn from(_value: Bytes) -> Self {
    todo!()
  }
}

impl Into<Bytes> for Packet {
  fn into(self) -> Bytes {
    todo!()
  }
}