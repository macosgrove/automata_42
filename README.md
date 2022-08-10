# Automata 42

## What?

Automata 42 is an implementation of [cellular automata](https://en.wikipedia.org/wiki/Cellular_automaton).
This implementation has a two dimensional universe - a grid where each cell can be any colour - and sets of rules (evolution algorithms) that determine how each cell changes over time.

So far three algorithms have been written:

* Random - every cell changes to a random new colour at each generation
* Rainbow - cells are initialised to a rainbow and the hue rotates at each generation
* Conways - [Conway's Game of Life](https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life)

You can see brief videos of each of these algorithms in my [Cellular Automata YouTube playlist](https://www.youtube.com/playlist?list=PLrV7ju1PHCWzTkbu4PmWoJK8-MdH80Jcu)

Colour is represented as 32 bit integers (u32 in Rust) in the form #0hhhssll, where h is hue, s is saturation and l is luminance.
Hue ranges from 0..360. Values over 359 will be wrapped around. Sat and Lum range from 0..256, which then get mapped to 0...1.
See the unit tests in `./src/colours.rs` for tips about colour.

HSL is used rather than the more well known RGB for colour, so that it is easy for the evolution algorithms to do things like modifying the hue while keeping the saturation and luminance constant. See the Rainbow algorithm for an example of this.

## Why?

I've written Automata 42 for several reasons:

* My husband is a theoretical physicist who believes that underpinning our real universe is a cellular automaton at incredibly tiny scales. We wanted a tool where we could play with and visualise cellular automata together. The '42' in the name is after Douglas Adam's Hitchhikers Guide to the Universe, in which 42 is the answer to life, the universe and everything.
* I've always had an interest in computer graphics and cellular automata - in fact I have a tatoo of a glider from Conway's Game of Life.
* I was interested in learning Rust and wanted a side project with a bit of meat on it for exploring the language (I do Ruby in my day job).

## How?

To run or modify Automata 42 you will first need to [install Rust](https://www.rust-lang.org/tools/install)

### Running the Automata

```sh
cargo run
```

### Runing the tests

```sh
cargo test
```

### Switching to a new algorithm

All the algorithms are listed in `./src/evolution/mod.rs`
You can run a different algorithm by replacing the algorithm name at the top of `./src/main.rs` where indicated.

### Writing a new algorithm

To add an algorithm:

* add a file for the algorithm to the `evolution` folder
* implement three functions:
  * `whoami() -> &'static str`,
  * `init(x: usize, y: usize) -> u32`, and
  * `evolve(last_generation: &Generation, x:usize, y:usize) -> u32`.
* list the new algorithm in `./src/evolution/mod.rs`.

`init()` is called to initialise the universe. It takes the position of the cell and should return the colour you want to set the new cell to, in #0hhhssll format (see the introduction above). Check out `./src/colours.rs` for some handy colour functions.

`evolve()` is called at each generation to calculate the new state of all cells. The function takes the state of the universe in the last generation and the location of the current cell. Like `init()`, `evolve` returns a colour in #0hhhssll format.

## Extensions

Would you like to contribute? Here are some ideas for extending this project:

* Allow the algorithm to be selected at runtime from a drop down list
* Allow the user to start, pause, step backwards (within reason) and step forwards.
* Allow the user to draw the initial conditions
* More algorithms!
