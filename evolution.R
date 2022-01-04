initialise <- function(width, height) {
  hue <- raster(matrix(40, ncol = width, nrow = height)) 
  sat <- raster(matrix((0:(width - 1)) * 100 / (width - 1), ncol = width, nrow = height, byrow=TRUE))
  lum <- raster(matrix((0:(height - 1)) * 100 / (height - 1), ncol = width, nrow = height))
  universe <- brick(hue, sat, lum)
  names(universe) <- c("hue", "sat", "lum")
  crs(universe) <- "+proj=longlat" # Remove warnings
  extent(universe) <- c(xmin=0, xmax=1, ymin=0, ymax=height/width)
  universe
}

evolve <- function(universe) {
  calc(universe, rotate_hue)
}

rotate_hue <- function(hsl) {
  c(mod(hsl["hue"] + 2, 100), hsl["sat"], hsl["lum"])
}