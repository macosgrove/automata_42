require("grid")
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
  return(rgb(r, g, b))
}

# specify h, s and l in the range 0-100
hsl100_to_rgb <- function(h, s, l) {
  hsl_to_rgb(h * 360 / 100, s / 100, l / 100)
}

setup <- function(w=100, h=50) {
  quartz()
  width <<- w
  height <<- h
  hue <<- matrix((0:(width - 1)) * 100 / (width - 1), nrow = height, ncol = width, byrow = TRUE)
  sat <<- matrix(100, nrow = height, ncol = width)
  lum <<- matrix((0:(height - 1)) * 100 / (height - 1), nrow = height, ncol = width)
}

mainloop <- function() {
  universe_raster <<- matrix(0, height, width)
  for (x in 1:height) {
    for (y in 1:width) {
      universe_raster[x, y] <- hsl100_to_rgb(hue[x, y], sat[x, y], lum[x, y])
    }
  }
  grid.raster(universe_raster)
}

teardown <- function() {
  dev.off()
}

setup()
mainloop()
teardown()
