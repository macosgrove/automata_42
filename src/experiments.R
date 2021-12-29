require("grid")

# Open a graphics window on Mac
# quartz()


# Various fills:

hue <<- matrix(50, nrow = height, ncol = width) # red
hue <<- matrix((0:(width - 1)) * 100 / (width - 1), nrow = height, ncol = width, byrow = TRUE) # vertical rainbow
sat <<- matrix(0, nrow = height, ncol = width) # no colour
sat <<- matrix(100, nrow = height, ncol = width) # full colour
lum <<- matrix(runif(width*height)*100, nrow = height, ncol = width) # random greys
lum <<- matrix((0:(height - 1)) * 100 / (height - 1), nrow = height, ncol = width) # black top, white bottom
lum <<- matrix(((height - 1):0) * 100 / (height - 1), nrow = height, ncol = width) # white top, black bottom



library(tcltk)

mywait <- function() {
  tt <- tktoplevel()
  tkpack( tkbutton(tt, text='Continue', command={function()tkdestroy(tt)}),
          side='bottom')
  tkbind(tt,'<KeyPress-space>', function(){ print("You pressed space") } )
  tkbind(tt,'<KeyPress-a>', function(){ print("You pressed a") } )
  
  tkwait.window(tt)
}

loop <- function() {
  tt <- tktoplevel()
  key <- NA
  tkbind(tt,'<Key>', function(k) { key <<- k } )
  quit <- FALSE
  while (!quit) {
    print(".")
    if(!is.na(key)) {
      print(paste("Key detected: ", key))
      if (key == 20) {
        quit <- TRUE
      }
      key <- NA
    }
    Sys.sleep(1)
  }
  tkdestroy(tt)
}