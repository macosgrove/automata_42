# Experiment with raster image

mem_experiment_plotrgb_raster <- function(width = 1000, height = 1000) {
  r <- raster(matrix(0, ncol = width, nrow = height))
  g <- raster(matrix((0:(width - 1)) * 255 / (width - 1), ncol = width, nrow = height))
  b <- raster(matrix((0:(height - 1)) * 255 / (height - 1), ncol = width, nrow = height, byrow = TRUE))
  s <- stack(r, g, b)
  crs(s) <- "+proj=longlat"
  print(mem_used())
  for (t in 1:100) {
    time <- system.time({
      plotRGB(s, r = 1, g = 2, b = 3)
    })
    print(mem_used())
    print(time)
  }
}

mem_experiment_grid_raster <- function(width = 1000, height = 1000) {
  hue <- (matrix(40, ncol = width, nrow = height)) 
  sat <- (matrix((0:(width - 1)) * 100 / (width - 1), ncol = width, nrow = height))
  lum <- (matrix((0:(height - 1)) * 100 / (height - 1), ncol = width, nrow = height))
  universe_raster <- (matrix(0, ncol = width, nrow = height)) 
  for (x in 1:height) {
    for (y in 1:width) {
      universe_raster[x, y] <- hsl100_to_rgb(hue[x, y], sat[x, y], lum[x, y])
    }
  }
  print(mem_used())
  for (t in 1:100) {
    time <- system.time({
      grid.raster(universe_raster, interpolate = FALSE)
    })
    print(mem_used())
    print(time)
  }
}


# > mem_experiment_plotrgb_raster()
# 313 MB
# 317 MB
# user  system elapsed 
# 0.282   0.054   0.337 
# 321 MB
# user  system elapsed 
# 0.253   0.044   0.298 
# 325 MB
# user  system elapsed 
# 0.254   0.042   0.296 
# 329 MB
# user  system elapsed 
# 0.248   0.042   0.289 
# 333 MB
# user  system elapsed 
# 0.251   0.042   0.292 
# 338 MB
# user  system elapsed 
# 0.244   0.042   0.286 
# 338 MB
# user  system elapsed 
# 0.242   0.041   0.283 
# 342 MB
# user  system elapsed 
# 0.250   0.042   0.291 
# 346 MB
# user  system elapsed 
# 0.263   0.062   0.324 
# 350 MB
# user  system elapsed 
# 0.262   0.057   0.319 

# > mem_experiment_grid_raster()
# 358 MB
# 366 MB
# user  system elapsed 
# 0.077   0.011   0.087 
# 374 MB
# user  system elapsed 
# 0.080   0.008   0.087 
# 382 MB
# user  system elapsed 
# 0.083   0.004   0.088 
# 390 MB
# user  system elapsed 
# 0.090   0.006   0.096 
# 398 MB
# user  system elapsed 
# 0.092   0.005   0.098 
# 406 MB
# user  system elapsed 
# 0.096   0.004   0.100 
# 414 MB
# user  system elapsed 
# 0.101   0.004   0.104 
# 422 MB
# user  system elapsed 
# 0.109   0.005   0.115 
# 430 MB
# user  system elapsed 
# 0.112   0.006   0.118 
# 438 MB
# user  system elapsed 
# 0.118   0.005   0.123 
