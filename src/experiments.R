# Experiments with RasterBrick

width <- 6
height <- 4
one <- raster(matrix(1:height, ncol = width, nrow = height))
two <- raster(matrix(1:width, ncol = width, nrow = height, byrow=TRUE))
three <- raster(matrix(1:(height*width), ncol = width, nrow = height, byrow=TRUE))

data_from_stack <- stack(one, two, three)
values(data_from_stack)
extent(data_from_stack)

# Use crop to extract a subregion. It always returns a brick
# To crop by row and column numbers you can create an extent like this (for Raster x, row 5 to 10, column 7 to 12) crop(x, extent(x, 5, 10, 7, 12))
ext <- extent(data_from_stack, 1, 3, 2, 3)
sub_region <- crop(data_from_stack, ext)
values(sub_region)
extent(sub_region)
class(sub_region)

# Try adjacent to extract a subregion
adj <- adjacent(data_from_stack, cells=c(1, width*height), directions=8, pairs=FALSE) 
as.matrix(adj)
class(adj)