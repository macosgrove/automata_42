# https://stackoverflow.com/questions/28562288/how-to-use-the-hsl-hue-saturation-lightness-cylindric-color-model
# specify h as whole input degrees (e.g 0-360)
# s = 0.0 - 1 (0 - 100%)
# l = 0.0 - 1, (0 - 100%)
# returns output from R's rgb() function
hsl_to_rgb <- function(h, s, l) {
  h <- h / 360
  r <- g <- b <- 0.0
  if (s == 0) {
    r <- g <- b <- l
  } else {
    hue_to_rgb <- function(p, q, t) {
      if (t < 0) {
        t <- t + 1.0
      }
      if (t > 1) {
        t <- t - 1.0
      }
      if (t < 1 / 6) {
        return(p + (q - p) * 6.0 * t)
      }
      if (t < 1 / 2) {
        return(q)
      }
      if (t < 2 / 3) {
        return(p + ((q - p) * ((2 / 3) - t) * 6))
      }
      return(p)
    }
    q <- ifelse(l < 0.5, l * (1.0 + s), l + s - (l * s))
    p <- 2.0 * l - q
    r <- hue_to_rgb(p, q, h + 1 / 3)
    g <- hue_to_rgb(p, q, h)
    b <- hue_to_rgb(p, q, h - 1 / 3)
  }
  c(round(r * 255), round(g * 255), round(b * 255))
}

# specify h, s and l in the range 0-100
hsl100_to_rgb <- function(h, s, l) {
  hsl_to_rgb(h * 360 / 100, s / 100, l / 100)
}

# function designed to be applied to stacks via "calc", 
#   eg calc(hsl_stack, hsl_stack_to_rgb) returns a RasterBrick with layers for rgb

hsl_stack_to_rgb <- function(hsl) {
  h = hsl[1] * 360 / 100
  s = hsl[2] / 100
  l = hsl[3] / 100
  hsl_to_rgb(h, s, l)
}
