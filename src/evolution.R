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
  for(c in 2:(ncol(universe)-1)) {
    for(r in 2:(nrow(universe)-1)) {
      region = crop(universe, extent(universe, r-1, r+1, c-1, c+1))
      universe[r,c] <- rotate_hue(region)
    }
  }
  universe
}

rotate_hue <- function(region) {
  c(mod(region[2,2][1] + 2, 255), region[2,2][2], region[2,2][3])
}
