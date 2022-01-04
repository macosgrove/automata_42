require("raster")
require("tcltk")
require("numbers")
require("pryr")

source("~/src/automata_42/src/colours.R")
source("~/src/automata_42/evolution.R")

main <- function(width =10, height = 5) {
  quartz()
  tt <- tktoplevel()
  key <- NA
  tkbind(tt,'<Key>', function(k) { key <<- k } )
  quit <- FALSE
  hue <- raster(matrix(40, ncol = width, nrow = height)) 
  sat <- raster(matrix((0:(width - 1)) * 100 / (width - 1), ncol = width, nrow = height, byrow=TRUE))
  lum <- raster(matrix((0:(height - 1)) * 100 / (height - 1), ncol = width, nrow = height))
  universe <- brick(hue, sat, lum)
  names(universe) <- c("hue", "sat", "lum")
  crs(universe) <- "+proj=longlat" # Remove warnings
  extent(universe) <- c(xmin=0, xmax=1, ymin=0, ymax=height/width)
  print(universe)
  
  while (!quit) {
    time <- system.time({
      universe_rgb <<- calc(universe, hsl_stack_to_rgb)
      plotRGB(universe_rgb, r = 1, g = 2, b = 3)
      universe <- evolve(universe)
      names(universe) <- c("hue", "sat", "lum")
      print(universe)
      if(!is.na(key)) {
        print(paste("Key detected: ", key))
        if (key == 20) {
          quit <- TRUE
        }
        key <- NA
      }
    })
    print(mem_used())
    print(time)
  }
  
  tkdestroy(tt)
  graphics.off()
}

