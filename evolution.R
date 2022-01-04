evolve <- function(universe) {
  calc(universe, rotate_hue)
}

rotate_hue <- function(hsl) {
  c(mod(hsl["hue"] + 2, 100), hsl["sat"], hsl["lum"])
}