# Experiment with calc

one <<- raster(matrix(1:12, nrow=3, ncol=4))

two <<- one * 2

s <<- stack(one, two)
names(s) <- c("one", "two")

my_func_1 <- function(x) {
  c(x["one"] + x["two"], x["one"] - x["two"], x["one"] * x["two"], x["one"]/x["two"])
}

my_func_2 <- function(x) {
  c(x[1] + x[2], x[1] - x[2], x[1] * x[2], x[1]/x[2])
}

my_func_3 <- function(x) {
  x[2] - x[1]/2
}

result_1 <<- calc(s, my_func_1)
result_2 <<- calc(s, my_func_2)
result_3 <<- calc(s, my_func_3)

print("Three results")
print(result_1)
print(result_2)
print(result_3)
print("")
print("Result 1 values")
print(values(result_1[[1]]))
print(values(result_1))
print("Result 2 as matrix")
print(as.matrix(result_2))
print("result 3 values")
print(values(result_3))
print("result 3 as matrix")
print(as.matrix(result_3))




# the function takes a vector - the set of cells through the layers of the stack
# Within the function, the values of the stack can be addressed by name or index.
# If it returns a vector, the result is a RasterBrick composed of the returned vectors.
# If it returns a single value, the result is a RasterLayer of all returned values.
# You can use s[[n]] to return a layer from a stack or brick, and as.matrix to return its values.
# 
# 
# [1] "Three results"
# class      : RasterBrick 
# dimensions : 3, 4, 12, 4  (nrow, ncol, ncell, nlayers)
# resolution : 0.25, 0.3333333  (x, y)
# extent     : 0, 1, 0, 1  (xmin, xmax, ymin, ymax)
# crs        : NA 
# source     : memory
# names      : one.1, one.2, one.3, one.4 
# min values :   3.0, -12.0,   2.0,   0.5 
# max values :  36.0,  -1.0, 288.0,   0.5 
# 
# class      : RasterBrick 
# dimensions : 3, 4, 12, 4  (nrow, ncol, ncell, nlayers)
# resolution : 0.25, 0.3333333  (x, y)
# extent     : 0, 1, 0, 1  (xmin, xmax, ymin, ymax)
# crs        : NA 
# source     : memory
# names      : one.1, one.2, one.3, one.4 
# min values :   3.0, -12.0,   2.0,   0.5 
# max values :  36.0,  -1.0, 288.0,   0.5 
# 
# class      : RasterLayer 
# dimensions : 3, 4, 12  (nrow, ncol, ncell)
# resolution : 0.25, 0.3333333  (x, y)
# extent     : 0, 1, 0, 1  (xmin, xmax, ymin, ymax)
# crs        : NA 
# source     : memory
# names      : layer 
# values     : 1.5, 18  (min, max)
# 
# [1] ""
# [1] "Result 1 values"
# [1]  3 12 21 30  6 15 24 33  9 18 27 36
# one.1 one.2 one.3 one.4
# [1,]     3    -1     2   0.5
# [2,]    12    -4    32   0.5
# [3,]    21    -7    98   0.5
# [4,]    30   -10   200   0.5
# [5,]     6    -2     8   0.5
# [6,]    15    -5    50   0.5
# [7,]    24    -8   128   0.5
# [8,]    33   -11   242   0.5
# [9,]     9    -3    18   0.5
# [10,]    18    -6    72   0.5
# [11,]    27    -9   162   0.5
# [12,]    36   -12   288   0.5
# [1] "Result 2 as matrix"
# one.1 one.2 one.3 one.4
# [1,]     3    -1     2   0.5
# [2,]    12    -4    32   0.5
# [3,]    21    -7    98   0.5
# [4,]    30   -10   200   0.5
# [5,]     6    -2     8   0.5
# [6,]    15    -5    50   0.5
# [7,]    24    -8   128   0.5
# [8,]    33   -11   242   0.5
# [9,]     9    -3    18   0.5
# [10,]    18    -6    72   0.5
# [11,]    27    -9   162   0.5
# [12,]    36   -12   288   0.5
# [1] "result 3 values"
# [1]  1.5  6.0 10.5 15.0  3.0  7.5 12.0 16.5  4.5  9.0 13.5 18.0
# [1] "result 3 as matrix"
# [,1] [,2] [,3] [,4]
# [1,]  1.5  6.0 10.5 15.0
# [2,]  3.0  7.5 12.0 16.5
# [3,]  4.5  9.0 13.5 18.0