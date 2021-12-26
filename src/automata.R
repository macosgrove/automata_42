requireNamespace("grid")
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
      if (t < 0) { t <- t + 1.0 }
      if (t > 1) { t <- t - 1.0 }
      if (t < 1/6) { return(p + (q - p) * 6.0 * t) }
      if (t < 1/2) { return(q) }
      if (t < 2/3) { return(p + ((q - p) * ((2/3) - t) * 6)) }
      return(p)
    }
    q <- ifelse(l < 0.5, l * (1.0 + s), l + s - (l*s))
    p <- 2.0 * l - q
    r <- hue_to_rgb(p, q, h + 1/3)
    g <- hue_to_rgb(p, q, h)
    b <- hue_to_rgb(p, q, h - 1/3)
  }
  return(rgb(r,g,b))
}

# specify h, s and l in the range 0-100
hsl100_to_rgb <- function(hsl) {
  hsl_to_rgb(hsl[1]*360/100, hsl[2]/100, hsl[3]/100)
}

setup <- function(){
  quartz()
  height <- 10
  width <- 10
  a42.viewport <<- grid::viewport(x=0.5, y=0.5, width=1, height=1, 
                                  default.units="npc", xscale=c(1,10), yscale=c(1,10))
  grid::pushViewport(a42.viewport)
  init <<- c(45, 50, 50, 0)
  universe <<- matrix(init, nrow = height, ncol = width)
}

mainloop <- function(){
  a42.params <- grid::gpar(fill="#fac0ff")
  grid::grid.rect(x=1, y=1, width=2, height=3, gp=a42.params, vp=a42.viewport)
  # universe_raster <- apply(universe, c(1,2), hsl100_to_rgb)
  # grid.raster(universe_raster)
}

teardown <- function(){
  dev.off()
}

setup()
mainloop()
teardown()
