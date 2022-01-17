initialise <- function(width, height) {
  hue <- raster(matrix(40, ncol = width, nrow = height)) 
  sat <- raster(matrix(as.integer((0:(width - 1)) * 255 / (width - 1)), ncol = width, nrow = height, byrow=TRUE))
  lum <- raster(matrix(as.integer((0:(height - 1)) * 255 / (height - 1)), ncol = width, nrow = height))
  universe <- brick(hue, sat, lum)
  names(universe) <- c("hue", "sat", "lum")
  crs(universe) <- "+proj=longlat" # Remove warnings
  extent(universe) <- c(xmin=0, xmax=1, ymin=0, ymax=height/width)
  universe
}

evolve <- function(universe) {
  calc(universe, rotate_sat)
}

rotate_hue <- function(hsl) {
  c(mod(hsl["hue"] + 2, 255), hsl["sat"], hsl["lum"])
}

rotate_sat <- function(hsl) {
  c(hsl["hue"], mod(hsl["sat"] + 2, 255), hsl["lum"])
}