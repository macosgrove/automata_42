pub const WHITE: u32 = 0xffffff;
pub const BLACK: u32 = 0x000000;

use std::cmp;
use hsl::HSL;

//Specify HSL colour in a u32 as 0x0hhhssll
//The HSL package expects HSL as h = 0-360 degrees, s = 0...1, l = 0...1
//In this package we wrap the value of H so it can range from 0x0000 to 0xffff
//s and l are adjusted so that 0x0 and 0x1 are both 0.0, 0x80 is 0.5 and 0xff is 1.0
pub fn hsl_to_rgb(hsl: u32) -> u32 {
  let (h, s, l) = hsl_to_h_s_l_f64(hsl);
  let hsl = HSL {h: h, s: s, l: l};
  let (r, g, b) = hsl.to_rgb();
  r_g_b_to_rgb(r, g, b)
}

pub fn hsl_to_h_s_l_f64(hsl:u32) -> (f64, f64, f64) {
  let h = (hsl & 0xffff0000) >> 16;
  let s = cmp::max((hsl & 0x0000ff00) >> 8, 1);
  let l = cmp::max(hsl & 0x000000ff, 1);
  let adj_s = (s-1) as f64/254.0;
  let adj_l = (l-1) as f64/254.0;
  return ((h % 360) as f64, adj_s, adj_l);
}

pub fn r_g_b_to_rgb(r:u8, g:u8, b:u8) -> u32 {
 return  ((r as u32) << 16) + ((g as u32) << 8) + (b as u32)
}

pub fn h_s_l_to_hsl(h:u16, s:u8, l:u8) -> u32 {
  return ((h as u32) << 16) + ((s as u32) << 8) + (l as u32)
}

pub fn hsl_to_h_s_l(hsl: u32) -> (u16, u8, u8) {
  let h = ((hsl & 0xffff0000) >> 16) as u16;
  let s = ((hsl & 0x0000ff00) >> 8) as u8;
  let l = (hsl & 0x000000ff) as u8;
  return (h, s, l)
}

#[cfg(test)]
mod tests {
  use rstest::*;
  use crate::colours::*;

use super::hsl_to_h_s_l_f64;

  #[rstest]
  #[case(0x00a800ff, 0xffffff)] //anything with full lum and 0 sat is white
  #[case(0x00123400, 0x000000)] //anything with 0 lum is black
  #[case(0x0000ff80, 0xff0000)] //red has hue 0 full sat and 50% lum
  #[case(0x0168ff80, 0xff0000)] //red can also be represented as hue 360=0x168 full sat and 50% lum
  #[case(0x2a30ff80, 0xff0000)] //hue keeps wrapping so all multiples of 360 are red
  #[case(0x00f0ff80, 0x0000ff)] //blue has hue 240 degrees = 0xf0
  #[case(0x0078ff80, 0x00ff00)] //green has hue 120 degrees = 0x78
  fn test_hsl_to_rgb(#[case] hsl:u32, #[case] expected_rgb:u32 ) {
    assert_eq!(hsl_to_rgb(hsl), expected_rgb, "Expected {:#08x} but got {:#08x}", expected_rgb, hsl_to_rgb(hsl))
  }

  #[rstest]
  #[case(0x00000000, 0.0, 0.0, 0.0)]
  #[case(0x00000101, 0.0, 0.0, 0.0)] // 0x00 and 0x01 are both treated as 0 so that 0x80 can be 0.5 and 0xff can be 1.0
  #[case(0x01680000, 0.0, 0.0, 0.0)] // hue wraps back around to red at 360 = 0x168
  #[case(0x01698080, 1.0, 0.5, 0.5)]
  #[case(0x00b4ffff, 180.0, 1.0, 1.0)] // sat and lum max out at 255 => 1
  #[case(0xffffffff, 15.0, 1.0, 1.0)] // hue keeps wrapping up to #ffff
  fn test_hsl_to_h_s_l_f64(#[case] hsl:u32, #[case] exp_h:f64, #[case] exp_s:f64, #[case] exp_l:f64) {
    let(h, s, l) = hsl_to_h_s_l_f64(hsl);
    assert!((h - exp_h).abs() < 0.0001);
    assert!((s - exp_s).abs() < 0.0001);
    assert!((l - exp_l).abs() < 0.0001);
  }

  #[rstest]
  #[case(0x00, 0x00, 0x00, 0x00000000)]
  #[case(0xff, 0x00, 0x00, 0x00ff0000)]
  #[case(0xff, 0xbb, 0x00, 0x00ffbb00)]
  #[case(0x00, 0xbb, 0x11, 0x0000bb11)]
  fn test_r_g_b_to_rgb(#[case] r: u8, #[case] g: u8, #[case] b: u8, #[case] exp_rgb: u32) {
    assert_eq!(r_g_b_to_rgb(r, g, b), exp_rgb);
  }

  #[rstest]
  #[case(0x00000000, 0, 0, 0,)]
  #[case(0x00000101, 0, 0x1, 0x1)]
  #[case(0x01680000, 0x168, 0, 0)]
  #[case(0x01698080, 0x169, 0x80, 0x80)]
  #[case(0x00b4ffff, 0x00b4, 0xff, 0xff)] // sat and lum max out at 255 => 1
  #[case(0xffffffff, 0xffff, 0xff, 0xff)] // hue keeps wrapping up to #ffff
  fn test_hsl_to_h_s_l(#[case] hsl:u32, #[case] exp_h:u16, #[case] exp_s:u8, #[case] exp_l:u8) {
    let(h, s, l) = hsl_to_h_s_l(hsl);
    assert_eq!(h, exp_h);
    assert_eq!(s, exp_s);
    assert_eq!(l, exp_l);
  }

  #[rstest]
  #[case(0x00, 0x00, 0x00, 0x00000000)]
  #[case(0xff, 0x00, 0x00, 0x00ff0000)]
  #[case(0xff, 0xbb, 0x00, 0x00ffbb00)]
  #[case(0x00, 0xbb, 0x11, 0x0000bb11)]
  #[case(0x1234, 0xbb, 0x11, 0x1234bb11)]
  fn test_h_s_l_to_hsl(#[case] h: u16, #[case] s: u8, #[case] l: u8, #[case] exp_hsl: u32) {
    assert_eq!(h_s_l_to_hsl(h, s, l), exp_hsl);
  }

}
