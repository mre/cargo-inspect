cargo inspect
cargo desugar
cargo uglify
cargo lower

> "There are only two ways to live your life. One is as though nothing is a miracle. The other is as though everything is a miracle." - Albert Einstein

## Installation

You need Rust nightly and rustfmt to get started.  
Use [rustup] to get one:

```
rustup install nightly
rustup component add rustfmt-preview
```

All set? Let's get cracking!

```
cargo install cargo-inspect
```

## Usage

Call it on any Rust file:

```
cargo inspect src/lib.rs
```

## Background

What is Rust doing behind the scenes?

Rust allows for a lot of syntactic sugar, that makes it a pleasure to write.
It is sometimes hard, however, to look behind the curtain and see what the compiler is really doing with our code.

To quote @tshepang, 
"It is good to know what these conveniences are, to avoid being mystified by what's going on under the hood... the less magical thinking we have of the world, the better."

* lifetime elisions
* type inference
* syntactic sugar
* implicit dereferencing
* type coercions
* hidden code (e.g. the prelude)

I was always interested in how programming languages work in the background,
how my code was unrolled to the compiler backend easier to maintain.

The goal is to make the compiler more approachable for mere-mortals.
Mystery! Exploration! Discovery! 


## Code Examples

https://www.youtube.com/watch?v=aGJTXdXQN2o&feature=youtu.be&t=1886

## The Magic Sauce

The best things in the world are assembled from simple building blocks.
This tool stands on the shoulders of giants.
To work its magic, it runs the following commands:

1. `rustc -Zinspect=hir`, for retrieving the HIR.
2. rustfmt, for formatting the output.
3. [syntect](https://github.com/trishume/syntect/blob/master/examples/syncat.rs), for syntax-highlighting.


## Contributing

This is a young project, which has downsides and upsides.

* Everything is in flux and things can break at any time. 😫
* There's plenty of opportunity to shape and form the project. 😊

Thus, become a contributor today!

## Known issues

As of now, this is a very fragile tool.
If it fails, it --might-- will produce horrible error messages. You have been warned.


[rustup]: https://rustup.rs/

## License

Licensed under either of

* Apache License, Version 2.0, (LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license (LICENSE-MIT or http://opensource.org/licenses/MIT)

at your option.