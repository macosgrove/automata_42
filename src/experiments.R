# This experiment shows that even without writing to the universe_raster
# or modifying the hue/sat/lum rasters, memory usage increases and time to display
# slows. This is a grid.raster issue.

# Start: 
# 349 MB
# user  system elapsed 
# 0.799   0.047   0.846 
# End:
# 438 MB
# user  system elapsed 
# 0.998   0.047   1.044   
mem_experiment <- function () {
  print(mem_used())
  for (t in 1:30) {
    time <- system.time({
    # for (x in 1:height) {
    #   for (y in 1:width) {
    #     universe_raster[x, y] <- hsl100_to_rgb(hue[x, y], sat[x, y], lum[x, y])
    #   }
    # }
    grid.raster(universe_raster, interpolate = FALSE)
    print(mem_used())
    print(time)
    })
  }
}
