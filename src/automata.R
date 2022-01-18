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
  universe <- initialise(width, height)
  # print(universe)
  
  options(error = function() {
    sink(stderr())
    on.exit(sink(NULL))
    traceback(3, max.lines = 1L)
    if (!interactive()) {
      q(status = 1)
    }
  })
  
  times <- c()
  while (!quit) {
    time <- system.time({
      universe_rgb <- calc(universe, hsl_stack_to_rgb)
      plotRGB(universe_rgb, r = 1, g = 2, b = 3)
      universe <- evolve(universe)
      names(universe) <- c("hue", "sat", "lum")
      # print(universe)
      if(!is.na(key)) {
        print(paste("Key detected: ", key))
        if (key == 20) {
          quit <- TRUE
        }
        key <- NA
      }
    })
    # print(mem_used())
    times <- c(times, time)
  }
  print(mean(times))
  tkdestroy(tt)
  graphics.off()
}

