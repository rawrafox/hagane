extern crate metal;

use std::io::Read;

use metal::*;
use metal::eve_rust::wbg;

fn main() {
  let args: Vec<String> = std::env::args().collect();

  let mut f = std::fs::File::open(&args[1]).unwrap();
  let mut data = Vec::new();
  f.read_to_end(&mut data).unwrap();

  let asset = wbg::import(&data[..]);

  let result = asset.export_asset_to_url_error(&NSURLID::new_with_string(&NSStringID::from_str(&args[2][..]))).unwrap();

  std::process::exit(if result { 0 } else { 1 });
}