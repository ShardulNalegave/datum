
mod packet;
mod header;

// ===== Imports =====
use anyhow::Result;
// ===================

#[tokio::main]
async fn main() -> Result<()> {
  println!("Hello, world!");
  Ok(())
}
