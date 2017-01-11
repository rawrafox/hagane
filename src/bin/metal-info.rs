extern crate metal;

use metal::{NSObject, NSArray, NSString, MTLDevice, MTLDeviceID};

fn print_device_info(device: MTLDeviceID, default_device: bool) {
  println!("\tDevice name: {}{}", device.name().as_str(), if default_device { " (default)" } else { "" });
  println!("\tHeadless: {}", device.is_headless());
  println!("\tLow power: {}", device.is_low_power());
  println!("\tDepth 24 / Stencil 8: {}", device.is_depth24_stencil8_pixel_format_supported());
  println!("\tTexture sample counts: {:?}", device.texture_sample_counts());
  // println!("\tRecommended maximum working set size: {:?}", device.recommended_max_working_set_size()); 10.12+ only :C

  let max_tg_size = device.max_threads_per_threadgroup();

  println!("\tMax threadgroup size: {} x {} x {}", max_tg_size.width, max_tg_size.height, max_tg_size.depth);

  println!("\tiOS GPUFamily1 v1: {}", device.supports_feature_set(metal::MTLFeatureSet_iOS_GPUFamily1_v1));
  println!("\tiOS GPUFamily1 v2: {}", device.supports_feature_set(metal::MTLFeatureSet_iOS_GPUFamily1_v2));
  println!("\tiOS GPUFamily1 v3: {}", device.supports_feature_set(metal::MTLFeatureSet_iOS_GPUFamily1_v3));

  println!("\tiOS GPUFamily2 v1: {}", device.supports_feature_set(metal::MTLFeatureSet_iOS_GPUFamily2_v1));
  println!("\tiOS GPUFamily2 v2: {}", device.supports_feature_set(metal::MTLFeatureSet_iOS_GPUFamily2_v2));
  println!("\tiOS GPUFamily2 v3: {}", device.supports_feature_set(metal::MTLFeatureSet_iOS_GPUFamily2_v3));

  println!("\tiOS GPUFamily3 v1: {}", device.supports_feature_set(metal::MTLFeatureSet_iOS_GPUFamily3_v1));
  println!("\tiOS GPUFamily3 v2: {}", device.supports_feature_set(metal::MTLFeatureSet_iOS_GPUFamily3_v2));

  println!("\tOSX GPUFamily1 v1: {}", device.supports_feature_set(metal::MTLFeatureSet_OSX_GPUFamily1_v1));
  println!("\tOSX GPUFamily1 v2: {}", device.supports_feature_set(metal::MTLFeatureSet_OSX_GPUFamily1_v2));
  println!("\tOSX ReadWriteTexture Tier 2: {}", device.supports_feature_set(metal::MTLFeatureSet_OSX_ReadWriteTextureTier2));

  println!("\ttvOS GPUFamily1 v1: {}", device.supports_feature_set(metal::MTLFeatureSet_tvOS_GPUFamily1_v1));
  println!("\ttvOS GPUFamily1 v2: {}", device.supports_feature_set(metal::MTLFeatureSet_tvOS_GPUFamily1_v2));
}

pub fn main() {
  let devices: Vec<MTLDeviceID> = metal::all_devices().to_vec();
  let default_device = metal::system_default_device();

  println!("Devices ({} found)", devices.len());
  for device in devices {
    print_device_info(device.clone(), device.is_equal(default_device.clone()));
  }
}
