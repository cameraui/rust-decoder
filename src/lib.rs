use fast_image_resize::CropBox;
use fast_image_resize::{
  images::{Image, ImageRef},
  PixelType, ResizeAlg, ResizeOptions, Resizer,
};
use image::{ImageBuffer, RgbImage};
use napi::bindgen_prelude::*;
use napi::{Error, Result};
use napi_derive::napi;
use std::path::Path;
use yuvutils_rs::{
  bgra_to_rgb, rgb_to_rgba, rgba_to_rgb, yuv420_to_rgb, yuv420_to_rgba, yuv_nv12_to_rgb,
  yuv_nv12_to_rgba, YuvBiPlanarImage, YuvConversionMode, YuvPlanarImage, YuvRange,
  YuvStandardMatrix,
};

#[napi]
pub fn convert_yuv_to_grayscale(frame: Buffer, width: u32, height: u32) -> Result<Uint8Array> {
  let y_size = (width * height) as usize;
  let u_size = ((width / 2) * (height / 2)) as usize;
  let v_size = u_size;
  let expected_size = y_size + u_size + v_size;

  let yuv_data = frame.as_ref();
  if yuv_data.len() < expected_size {
    return Err(Error::from_reason(
      "YUV buffer is smaller than expected for the given width and height.",
    ));
  }

  let result = _internal_convert_yuv_to_grayscale_internal(yuv_data, width, height);

  Ok(Uint8Array::from(result))
}

#[napi]
pub fn convert_yuv_to_rgb(frame: Buffer, width: u32, height: u32) -> Result<Uint8Array> {
  let y_size = (width * height) as usize;
  let u_size = ((width / 2) * (height / 2)) as usize;
  let v_size = u_size;
  let expected_size = y_size + u_size + v_size;

  let yuv_data = frame.as_ref();
  if yuv_data.len() < expected_size {
    return Err(Error::from_reason(
      "YUV buffer is smaller than expected for the given width and height.",
    ));
  }

  let result = _internal_convert_yuv_to_rgb_internal(yuv_data, width, height)?;

  Ok(Uint8Array::from(result))
}

#[napi]
pub fn convert_yuv_to_rgba(frame: Buffer, width: u32, height: u32) -> Result<Uint8Array> {
  let y_size = (width * height) as usize;
  let u_size = ((width / 2) * (height / 2)) as usize;
  let v_size = u_size;
  let expected_size = y_size + u_size + v_size;

  let yuv_data = frame.as_ref();
  if yuv_data.len() < expected_size {
    return Err(Error::from_reason(
      "YUV buffer is smaller than expected for the given width and height.",
    ));
  }

  let result = _internal_convert_yuv_to_rgba_internal(yuv_data, width, height)?;

  Ok(Uint8Array::from(result))
}

#[napi]
pub fn convert_nv12_to_grayscale(frame: Buffer, width: u32, height: u32) -> Result<Uint8Array> {
  let y_size = (width * height) as usize;
  let uv_size = (width * height / 2) as usize;
  let expected_size = y_size + uv_size;

  let nv12_data = frame.as_ref();
  if nv12_data.len() < expected_size {
    return Err(Error::from_reason(
      "NV12 buffer is smaller than expected for the given width and height.",
    ));
  }

  let result = _internal_convert_nv12_to_grayscale_internal(nv12_data, width, height);

  Ok(Uint8Array::from(result))
}

#[napi]
pub fn convert_nv12_to_rgb(frame: Buffer, width: u32, height: u32) -> Result<Uint8Array> {
  let y_size = (width * height) as usize;
  let uv_size = (width * height / 2) as usize;
  let expected_size = y_size + uv_size;

  let nv12_data = frame.as_ref();
  if nv12_data.len() < expected_size {
    return Err(Error::from_reason(
      "NV12 buffer is smaller than expected for the given width and height.",
    ));
  }

  let result = _internal_convert_nv12_to_rgb_internal(nv12_data, width, height)?;

  Ok(Uint8Array::from(result))
}

#[napi]
pub fn convert_nv12_to_rgba(frame: Buffer, width: u32, height: u32) -> Result<Uint8Array> {
  let y_size = (width * height) as usize;
  let uv_size = (width * height / 2) as usize;
  let expected_size = y_size + uv_size;

  let nv12_data = frame.as_ref();
  if nv12_data.len() < expected_size {
    return Err(Error::from_reason(
      "NV12 buffer is smaller than expected for the given width and height.",
    ));
  }

  let result = _internal_convert_nv12_to_rgba_internal(nv12_data, width, height)?;

  Ok(Uint8Array::from(result))
}

#[napi]
pub fn convert_rgba_to_rgb(rgba_data: Uint8Array, width: u32, height: u32) -> Result<Uint8Array> {
  let rgba_slice = rgba_data.as_ref();
  let expected_size = (width * height * 4) as usize;

  if rgba_slice.len() != expected_size {
    return Err(Error::from_reason(format!(
      "RGBA buffer size mismatch. Expected {} bytes, got {}",
      expected_size,
      rgba_slice.len()
    )));
  }

  let rgba_stride = width * 4;
  let rgb_stride = width * 3;
  let mut rgb_data = vec![0u8; (rgb_stride * height) as usize];

  rgba_to_rgb(
    rgba_slice,
    rgba_stride,
    &mut rgb_data,
    rgb_stride,
    width,
    height,
  )
  .map_err(|e| Error::from_reason(format!("RGBA to RGB conversion failed: {}", e)))?;

  Ok(Uint8Array::from(rgb_data))
}

#[napi]
pub fn convert_bgra_to_rgb(bgra_data: Uint8Array, width: u32, height: u32) -> Result<Uint8Array> {
  let bgra_slice = bgra_data.as_ref();
  let expected_size = (width * height * 4) as usize;

  if bgra_slice.len() != expected_size {
    return Err(Error::from_reason(format!(
      "BGRA buffer size mismatch. Expected {} bytes, got {}",
      expected_size,
      bgra_slice.len()
    )));
  }

  let bgra_stride = width * 4;
  let rgb_stride = width * 3;
  let mut rgb_data = vec![0u8; (rgb_stride * height) as usize];

  bgra_to_rgb(
    bgra_slice,
    bgra_stride,
    &mut rgb_data,
    rgb_stride,
    width,
    height,
  )
  .map_err(|e| Error::from_reason(format!("BGRA to RGB conversion failed: {}", e)))?;

  Ok(Uint8Array::from(rgb_data))
}

#[napi]
pub fn convert_rgb_to_rgba(rgb_data: Uint8Array, width: u32, height: u32) -> Result<Uint8Array> {
  let rgb_slice = rgb_data.as_ref();
  let expected_size = (width * height * 3) as usize;

  if rgb_slice.len() != expected_size {
    return Err(Error::from_reason(format!(
      "RGB buffer size mismatch. Expected {} bytes, got {}",
      expected_size,
      rgb_slice.len()
    )));
  }

  let rgb_stride = width * 3;
  let rgba_stride = width * 4;
  let mut rgba_data = vec![0u8; (rgba_stride * height) as usize];

  rgb_to_rgba(
    rgb_slice,
    rgb_stride,
    &mut rgba_data,
    rgba_stride,
    width,
    height,
  )
  .map_err(|e| Error::from_reason(format!("RGB to RGBA conversion failed: {}", e)))?;

  Ok(Uint8Array::from(rgba_data))
}

#[napi]
pub fn resize_image(
  frame: Uint8Array,
  input_width: u32,
  input_height: u32,
  channels: u32,
  output_width: u32,
  output_height: u32,
) -> Result<Uint8Array> {
  let pixel_type = match channels {
    1 => PixelType::U8,
    2 => PixelType::U8x2,
    3 => PixelType::U8x3,
    4 => PixelType::U8x4,
    _ => return Err(Error::from_reason("Unsupported channel count")),
  };

  let resize_width_opt = if output_width > 0 {
    Some(output_width)
  } else {
    None
  };
  let resize_height_opt = if output_height > 0 {
    Some(output_height)
  } else {
    None
  };

  // If no resize needed, return original
  if resize_width_opt.is_none() || resize_height_opt.is_none() {
    return Ok(frame);
  }

  let result = _internal_apply_crop_resize(
    frame.as_ref(),
    input_width,
    input_height,
    pixel_type,
    None,
    None,
    None,
    None,
    Some(output_width),
    Some(output_height),
  )?;

  Ok(Uint8Array::from(result))
}

#[napi]
pub fn crop_image(
  frame: Uint8Array,
  input_width: u32,
  input_height: u32,
  channels: u32,
  crop_left: u32,
  crop_top: u32,
  crop_width: u32,
  crop_height: u32,
) -> Result<Uint8Array> {
  let pixel_type = match channels {
    1 => PixelType::U8,
    2 => PixelType::U8x2,
    3 => PixelType::U8x3,
    4 => PixelType::U8x4,
    _ => return Err(Error::from_reason("Unsupported channel count")),
  };

  let crop_left_opt = if crop_left > 0 { Some(crop_left) } else { None };
  let crop_top_opt = if crop_top > 0 { Some(crop_top) } else { None };
  let crop_width_opt = if crop_width > 0 {
    Some(crop_width)
  } else {
    None
  };
  let crop_height_opt = if crop_height > 0 {
    Some(crop_height)
  } else {
    None
  };

  // If no crop needed, return original
  if crop_left_opt.is_none()
    && crop_top_opt.is_none()
    && crop_width_opt.is_none()
    && crop_height_opt.is_none()
  {
    return Ok(frame);
  }

  let result = _internal_apply_crop_resize(
    frame.as_ref(),
    input_width,
    input_height,
    pixel_type,
    Some(crop_left),
    Some(crop_top),
    Some(crop_width),
    Some(crop_height),
    None,
    None,
  )?;

  Ok(Uint8Array::from(result))
}

#[napi]
pub fn resize_and_crop(
  input_frame: Uint8Array,
  input_width: u32,
  input_height: u32,
  channels: u32,
  crop_left: u32,
  crop_top: u32,
  crop_width: u32,
  crop_height: u32,
  output_width: u32,
  output_height: u32,
) -> Result<Uint8Array> {
  let pixel_type = match channels {
    1 => PixelType::U8,
    2 => PixelType::U8x2,
    3 => PixelType::U8x3,
    4 => PixelType::U8x4,
    _ => return Err(Error::from_reason("Unsupported channel count")),
  };

  let crop_left_opt = if crop_left > 0 { Some(crop_left) } else { None };
  let crop_top_opt = if crop_top > 0 { Some(crop_top) } else { None };
  let crop_width_opt = if crop_width > 0 {
    Some(crop_width)
  } else {
    None
  };
  let crop_height_opt = if crop_height > 0 {
    Some(crop_height)
  } else {
    None
  };

  let resize_width_opt = if output_width > 0 {
    Some(output_width)
  } else {
    None
  };
  let resize_height_opt = if output_height > 0 {
    Some(output_height)
  } else {
    None
  };

  let has_crop = crop_left_opt.is_some()
    || crop_top_opt.is_some()
    || crop_width_opt.is_some()
    || crop_height_opt.is_some();
  let has_resize = resize_width_opt.is_some() && resize_height_opt.is_some();

  // If no operations needed, return original
  if !has_crop && !has_resize {
    return Ok(input_frame);
  }

  let result = _internal_apply_crop_resize(
    input_frame.as_ref(),
    input_width,
    input_height,
    pixel_type,
    Some(crop_left),
    Some(crop_top),
    Some(crop_width),
    Some(crop_height),
    Some(output_width),
    Some(output_height),
  )?;

  Ok(Uint8Array::from(result))
}

#[napi]
pub fn process_image(
  input_frame: Uint8Array,
  input_width: u32,
  input_height: u32,
  input_format: u32,
  output_format: u32,
  crop_top: u32,
  crop_left: u32,
  crop_width: u32,
  crop_height: u32,
  resize_width: u32,
  resize_height: u32,
) -> Result<Uint8Array> {
  // Input formats: 1 = YUV420P (I420), 12 = NV12
  // Output formats: 1 = GRAY, 3 = RGB, 4 = RGBA
  if output_format != 1 && output_format != 3 && output_format != 4 {
    return Err(Error::from_reason(
      "Invalid output format. Must be GRAY=1, RGB=3, or RGBA=4.",
    ));
  }

  if input_format != 1 && input_format != 12 {
    return Err(Error::from_reason(
      "Invalid input format. Must be YUV420P=1 or NV12=12.",
    ));
  }

  let input_data = input_frame.as_ref();
  let y_size = (input_width * input_height) as usize;

  let expected_size = match input_format {
    1 => {
      // YUV420P
      let u_size = ((input_width / 2) * (input_height / 2)) as usize;
      let v_size = u_size;
      y_size + u_size + v_size
    }
    12 => {
      // NV12
      let uv_size = (input_width * input_height / 2) as usize;
      y_size + uv_size
    }
    _ => unreachable!(),
  };

  if input_data.len() < expected_size {
    return Err(Error::from_reason(
      "Input buffer is smaller than expected for the given width and height.",
    ));
  }

  // Convert crop/resize u32 to Option<u32>
  let crop_left_opt = if crop_left > 0 { Some(crop_left) } else { None };
  let crop_top_opt = if crop_top > 0 { Some(crop_top) } else { None };
  let crop_width_opt = if crop_width > 0 {
    Some(crop_width)
  } else {
    None
  };
  let crop_height_opt = if crop_height > 0 {
    Some(crop_height)
  } else {
    None
  };
  let resize_width_opt = if resize_width > 0 {
    Some(resize_width)
  } else {
    None
  };
  let resize_height_opt = if resize_height > 0 {
    Some(resize_height)
  } else {
    None
  };

  // Convert based on input and output format
  // For grayscale, we can avoid allocation and just reference the Y plane
  // For RGB/RGBA, we need to allocate
  let grayscale_buffer: &[u8];
  let rgb_buffer: Vec<u8>;

  let current_image: &[u8] = match (input_format, output_format) {
    (1, 1) => {
      // YUV420P -> GRAY (zero-copy: just extract Y plane)
      grayscale_buffer =
        _internal_convert_yuv_to_grayscale_internal(input_data, input_width, input_height);
      grayscale_buffer
    }
    (12, 1) => {
      // NV12 -> GRAY (zero-copy: just extract Y plane)
      grayscale_buffer =
        _internal_convert_nv12_to_grayscale_internal(input_data, input_width, input_height);
      grayscale_buffer
    }
    (1, 3) => {
      // YUV420P -> RGB
      rgb_buffer = _internal_convert_yuv_to_rgb_internal(input_data, input_width, input_height)?;
      &rgb_buffer
    }
    (1, 4) => {
      // YUV420P -> RGBA
      rgb_buffer = _internal_convert_yuv_to_rgba_internal(input_data, input_width, input_height)?;
      &rgb_buffer
    }
    (12, 3) => {
      // NV12 -> RGB
      rgb_buffer = _internal_convert_nv12_to_rgb_internal(input_data, input_width, input_height)?;
      &rgb_buffer
    }
    (12, 4) => {
      // NV12 -> RGBA
      rgb_buffer = _internal_convert_nv12_to_rgba_internal(input_data, input_width, input_height)?;
      &rgb_buffer
    }
    _ => return Err(Error::from_reason("Unsupported format combination.")),
  };

  // Apply crop/resize if needed
  let has_crop = crop_left_opt.is_some()
    || crop_top_opt.is_some()
    || crop_width_opt.is_some()
    || crop_height_opt.is_some();
  let has_resize = resize_width_opt.is_some() && resize_height_opt.is_some();

  // If no operations needed, return current_image directly
  if !has_crop && !has_resize {
    return Ok(Uint8Array::from(current_image));
  }

  let pixel_type = match output_format {
    1 => PixelType::U8,
    3 => PixelType::U8x3,
    4 => PixelType::U8x4,
    _ => return Err(Error::from_reason("Unsupported channel count")),
  };

  let result = _internal_apply_crop_resize(
    current_image,
    input_width,
    input_height,
    pixel_type,
    crop_left_opt,
    crop_top_opt,
    crop_width_opt,
    crop_height_opt,
    resize_width_opt,
    resize_height_opt,
  )?;

  Ok(Uint8Array::from(result))
}

#[napi]
pub fn process_image_debug(
  input_frame: Uint8Array,
  input_width: u32,
  input_height: u32,
  input_format: u32,
  output_format: u32,
  crop_top: u32,
  crop_left: u32,
  crop_width: u32,
  crop_height: u32,
  resize_width: u32,
  resize_height: u32,
  output_dir: String,
  prefix: String,
) -> Result<Uint8Array> {
  // First process the image normally
  let result = process_image(
    input_frame,
    input_width,
    input_height,
    input_format,
    output_format,
    crop_top,
    crop_left,
    crop_width,
    crop_height,
    resize_width,
    resize_height,
  )?;

  // Determine output dimensions
  let (out_width, out_height) = if resize_width > 0 && resize_height > 0 {
    (resize_width, resize_height)
  } else if crop_width > 0 && crop_height > 0 {
    (crop_width, crop_height)
  } else {
    (input_width, input_height)
  };

  // Save the result based on output format
  let result_data = result.as_ref();
  match output_format {
    1 => {
      // GRAY - save as grayscale PNG
      save_grayscale_image(
        result_data,
        out_width,
        out_height,
        &output_dir,
        &format!("{}_output.png", prefix),
      )?;
    }
    3 => {
      // RGB - save as RGB PNG
      save_rgb_image(
        result_data,
        out_width,
        out_height,
        &output_dir,
        &format!("{}_output.png", prefix),
      )?;
    }
    4 => {
      // RGBA - save as RGBA PNG
      save_rgba_image(
        result_data,
        out_width,
        out_height,
        &output_dir,
        &format!("{}_output.png", prefix),
      )?;
    }
    _ => return Err(Error::from_reason("Invalid output format")),
  }

  Ok(result)
}

// Internal

#[inline(always)]
fn _internal_convert_yuv_to_rgb_internal(
  yuv_data: &[u8],
  width: u32,
  height: u32,
) -> Result<Vec<u8>> {
  let yuv_range = YuvRange::Full;
  let yuv_matrix = YuvStandardMatrix::Bt2020;

  let y_size = (width * height) as usize;
  let u_size = ((width / 2) * (height / 2)) as usize;
  let v_size = u_size;
  let expected_size = y_size + u_size + v_size;

  let y_plane = &yuv_data[0..y_size];
  let u_plane = &yuv_data[y_size..y_size + u_size];
  let v_plane = &yuv_data[y_size + u_size..expected_size];

  let y_stride = width;
  let u_stride = width / 2;
  let v_stride = width / 2;

  let planar_image = YuvPlanarImage {
    y_plane,
    u_plane,
    v_plane,
    y_stride,
    u_stride,
    v_stride,
    width,
    height,
  };

  let rgb_stride = width * 3;
  let mut rgb_frame = vec![0u8; (rgb_stride * height) as usize];

  yuv420_to_rgb(
    &planar_image,
    &mut rgb_frame,
    rgb_stride,
    yuv_range,
    yuv_matrix,
  )
  .map_err(|e| Error::from_reason(format!("YUV to RGB conversion failed: {}", e)))?;

  Ok(rgb_frame)
}

#[inline(always)]
fn _internal_convert_yuv_to_rgba_internal(
  yuv_data: &[u8],
  width: u32,
  height: u32,
) -> Result<Vec<u8>> {
  let yuv_range = YuvRange::Full;
  let yuv_matrix = YuvStandardMatrix::Bt2020;

  let y_size = (width * height) as usize;
  let u_size = ((width / 2) * (height / 2)) as usize;
  let v_size = u_size;
  let expected_size = y_size + u_size + v_size;

  let y_plane = &yuv_data[0..y_size];
  let u_plane = &yuv_data[y_size..y_size + u_size];
  let v_plane = &yuv_data[y_size + u_size..expected_size];

  let y_stride = width;
  let u_stride = width / 2;
  let v_stride = width / 2;

  let planar_image = YuvPlanarImage {
    y_plane,
    u_plane,
    v_plane,
    y_stride,
    u_stride,
    v_stride,
    width,
    height,
  };

  let rgba_stride = width * 4;
  let mut rgba_frame = vec![0u8; (rgba_stride * height) as usize];

  yuv420_to_rgba(
    &planar_image,
    &mut rgba_frame,
    rgba_stride,
    yuv_range,
    yuv_matrix,
  )
  .map_err(|e| Error::from_reason(format!("YUV to RGBA conversion failed: {}", e)))?;

  Ok(rgba_frame)
}

#[inline(always)]
fn _internal_convert_yuv_to_grayscale_internal(yuv_data: &[u8], width: u32, height: u32) -> &[u8] {
  let y_size = (width * height) as usize;
  &yuv_data[0..y_size]
}

#[inline(always)]
fn _internal_convert_nv12_to_grayscale_internal(
  nv12_data: &[u8],
  width: u32,
  height: u32,
) -> &[u8] {
  let y_size = (width * height) as usize;
  &nv12_data[0..y_size]
}

#[inline(always)]
fn _internal_convert_nv12_to_rgb_internal(
  nv12_data: &[u8],
  width: u32,
  height: u32,
) -> Result<Vec<u8>> {
  let yuv_range = YuvRange::Full;
  let yuv_matrix = YuvStandardMatrix::Bt2020;

  let y_size = (width * height) as usize;
  let uv_size = (width * height / 2) as usize;

  let y_plane = &nv12_data[0..y_size];
  let uv_plane = &nv12_data[y_size..y_size + uv_size];

  let y_stride = width;
  let uv_stride = width;

  let biplanar_image = YuvBiPlanarImage {
    y_plane,
    y_stride,
    uv_plane,
    uv_stride,
    width,
    height,
  };

  let rgb_stride = width * 3;
  let mut rgb_frame = vec![0u8; (rgb_stride * height) as usize];

  yuv_nv12_to_rgb(
    &biplanar_image,
    &mut rgb_frame,
    rgb_stride,
    yuv_range,
    yuv_matrix,
    YuvConversionMode::default(),
  )
  .map_err(|e| Error::from_reason(format!("NV12 to RGB conversion failed: {}", e)))?;

  Ok(rgb_frame)
}

#[inline(always)]
fn _internal_convert_nv12_to_rgba_internal(
  nv12_data: &[u8],
  width: u32,
  height: u32,
) -> Result<Vec<u8>> {
  let yuv_range = YuvRange::Full;
  let yuv_matrix = YuvStandardMatrix::Bt2020;

  let y_size = (width * height) as usize;
  let uv_size = (width * height / 2) as usize;

  let y_plane = &nv12_data[0..y_size];
  let uv_plane = &nv12_data[y_size..y_size + uv_size];

  let y_stride = width;
  let uv_stride = width;

  let biplanar_image = YuvBiPlanarImage {
    y_plane,
    y_stride,
    uv_plane,
    uv_stride,
    width,
    height,
  };

  let rgba_stride = width * 4;
  let mut rgba_frame = vec![0u8; (rgba_stride * height) as usize];

  yuv_nv12_to_rgba(
    &biplanar_image,
    &mut rgba_frame,
    rgba_stride,
    yuv_range,
    yuv_matrix,
    YuvConversionMode::default(),
  )
  .map_err(|e| Error::from_reason(format!("NV12 to RGBA conversion failed: {}", e)))?;

  Ok(rgba_frame)
}

#[inline(always)]
fn _internal_apply_crop_resize(
  src_buffer: &[u8],
  src_width: u32,
  src_height: u32,
  pixel_type: PixelType,
  crop_left: Option<u32>,
  crop_top: Option<u32>,
  crop_width: Option<u32>,
  crop_height: Option<u32>,
  resize_width: Option<u32>,
  resize_height: Option<u32>,
) -> Result<Vec<u8>> {
  let src_image = ImageRef::new(src_width, src_height, src_buffer, pixel_type)
    .map_err(|e| Error::from_reason(format!("Failed to create source image: {}", e)))?;

  // Calculate final dimensions
  let (final_width, final_height) = if let (Some(w), Some(h)) = (resize_width, resize_height) {
    (w, h)
  } else {
    (
      crop_width.unwrap_or(src_width),
      crop_height.unwrap_or(src_height),
    )
  };

  let mut dst_image = Image::new(final_width, final_height, pixel_type);
  let mut resizer = Resizer::new();

  // Build resize options
  let has_crop =
    crop_left.is_some() || crop_top.is_some() || crop_width.is_some() || crop_height.is_some();
  let has_resize = resize_width.is_some() && resize_height.is_some();

  let options = if has_crop {
    Some(ResizeOptions {
      algorithm: ResizeAlg::Nearest,
      mul_div_alpha: true,
      cropping: fast_image_resize::SrcCropping::Crop(CropBox {
        left: crop_left.unwrap_or(0) as f64,
        top: crop_top.unwrap_or(0) as f64,
        width: crop_width.unwrap_or(src_width) as f64,
        height: crop_height.unwrap_or(src_height) as f64,
      }),
    })
  } else if has_resize {
    Some(ResizeOptions {
      algorithm: ResizeAlg::Nearest,
      mul_div_alpha: true,
      cropping: fast_image_resize::SrcCropping::None,
    })
  } else {
    None
  };

  resizer
    .resize(&src_image, &mut dst_image, options.as_ref())
    .map_err(|e| Error::from_reason(format!("Failed to crop/resize: {}", e)))?;

  Ok(dst_image.into_vec())
}

// Debugging

fn save_grayscale_image(
  data: &[u8],
  width: u32,
  height: u32,
  output_dir: &str,
  filename: &str,
) -> Result<()> {
  let img = image::GrayImage::from_raw(width, height, data.to_vec())
    .ok_or_else(|| Error::from_reason("Failed to create grayscale image"))?;

  let path = Path::new(output_dir).join(filename);
  img
    .save(&path)
    .map_err(|e| Error::from_reason(format!("Failed to save grayscale image: {}", e)))?;

  Ok(())
}

fn save_rgb_image(
  data: &[u8],
  width: u32,
  height: u32,
  output_dir: &str,
  filename: &str,
) -> Result<()> {
  let img: RgbImage = ImageBuffer::from_raw(width, height, data.to_vec())
    .ok_or_else(|| Error::from_reason("Failed to create RGB image"))?;

  let path = Path::new(output_dir).join(filename);
  img
    .save(&path)
    .map_err(|e| Error::from_reason(format!("Failed to save RGB image: {}", e)))?;

  Ok(())
}

fn save_rgba_image(
  data: &[u8],
  width: u32,
  height: u32,
  output_dir: &str,
  filename: &str,
) -> Result<()> {
  let img = image::RgbaImage::from_raw(width, height, data.to_vec())
    .ok_or_else(|| Error::from_reason("Failed to create RGBA image"))?;

  let path = Path::new(output_dir).join(filename);
  img
    .save(&path)
    .map_err(|e| Error::from_reason(format!("Failed to save RGBA image: {}", e)))?;

  Ok(())
}
