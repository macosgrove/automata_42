require("grid")
require("tcltk")
require("numbers")
require("pryr")

source("~/src/automata_42/src/colours.R")

setup <- function(w = 50, h = 20) {
  quartz()
  width <<- w
  height <<- h
  hue <<- matrix(40, nrow = height, ncol = width) # green
  sat <<- matrix((0:(width - 1)) * 100 / (width - 1), nrow = height, ncol = width, byrow = TRUE)
  lum <<- matrix((0:(height - 1)) * 100 / (height - 1), nrow = height, ncol = width)
  universe_raster <<- matrix(0, height, width)
}

mainloop <- function() {
  tt <- tktoplevel()
  key <- NA
  tkbind(tt,'<Key>', function(k) { key <<- k } )
  quit <- FALSE
  while (!quit) {
    for (x in 1:height) {
      for (y in 1:width) {
        universe_raster[x, y] <- hsl100_to_rgb(hue[x, y], sat[x, y], lum[x, y])
        hue[x,y] <- mod(hue[x, y] + 10, 101)
      }
    }
    grid.raster(universe_raster, interpolate = FALSE)
    print(mem_used())
    if(!is.na(key)) {
      print(paste("Key detected: ", key))
      if (key == 20) {
        quit <- TRUE
      }
      key <- NA
    }
  }
  tkdestroy(tt)
}

teardown <- function() {
  graphics.off()
}

teardown()
setup()
mainloop()
