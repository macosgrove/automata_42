require("grid")

# Open a graphics window on Mac
quartz()


# Various fills:

hue <<- matrix(50, nrow = height, ncol = width) # red
hue <<- matrix((0:(width - 1)) * 100 / (width - 1), nrow = height, ncol = width, byrow = TRUE) # vertical rainbow
sat <<- matrix(0, nrow = height, ncol = width) # no colour
sat <<- matrix(100, nrow = height, ncol = width) # full colour
lum <<- matrix(runif(width*height)*100, nrow = height, ncol = width) # random greys
lum <<- matrix((0:(height - 1)) * 100 / (height - 1), nrow = height, ncol = width) # black top, white bottom
lum <<- matrix(((height - 1):0) * 100 / (height - 1), nrow = height, ncol = width) # white top, black bottom
