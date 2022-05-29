pub const fn from_u8_rgb(r: u8, g: u8, b: u8) -> u32 {
  let (r, g, b) = (r as u32, g as u32, b as u32);
  (r << 16) | (g << 8) | b
}
pub const BLACK: u32 = 0;
pub const WHITE: u32 = 0xffffff;
pub const AZURE_BLUE: u32 = from_u8_rgb(0, 127, 255);

