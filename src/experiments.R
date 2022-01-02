# Experiment with raster image
require("raster")
require("tcltk")
require("pryr")
require("grid")

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
# 213 MB
# 223 MB
# user  system elapsed 
# 0.616   0.098   0.710 
# 227 MB
# user  system elapsed 
# 0.356   0.067   0.420 
# 231 MB
# user  system elapsed 
# 0.276   0.035   0.311 
# ...(94 more, then)
# 287 MB
# user  system elapsed 
# 0.252   0.044   0.295 
# 287 MB
# user  system elapsed 
# 0.259   0.045   0.303 
# 287 MB
# user  system elapsed 
# 0.251   0.043   0.294 

# > mem_experiment_grid_raster()
# 234 MB
# 242 MB
# user  system elapsed 
# 0.057   0.011   0.067 
# 250 MB
# user  system elapsed 
# 0.060   0.009   0.071 
# 258 MB
# user  system elapsed 
# 0.065   0.006   0.072 
# ...(94 more, then)
# 1.02 GB
# user  system elapsed 
# 1.103   0.126   1.230 
# 1.03 GB
# user  system elapsed 
# 1.091   0.125   1.216 
# 1.03 GB
# user  system elapsed 
# 1.117   0.129   1.245 
