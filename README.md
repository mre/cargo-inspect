# cargo-inspect

![Logo](./assets/logo.svg)

[![docs](https://docs.rs/cargo-inspect/badge.svg)](https://docs.rs/cargo-inspect/)
[![Build Status](https://travis-ci.org/mre/cargo-inspect.svg?branch=master)](https://travis-ci.org/mre/cargo-inspect)

## Thanks All!

ℹ️ This crate was **superceded by [cargo-expand](https://github.com/dtolnay/cargo-expand)**, which added support for all the features that were missing
when we started to work on cargo-inspect. Thanks all for your feedback and support.

## What is Rust doing behind the scenes?

> There are only two ways to live your life.  
> One is as though nothing is a miracle. The other is as though everything is a
> miracle. -- Albert Einstein

## Installation

You need Rust nightly and `rustfmt` to get started.  
You can install those via [rustup]:

```
rustup install nightly
rustup component add rustfmt
```

All set? Let's get cracking!

```
cargo install cargo-inspect
```

## Usage

Call it on any Rust file:

```
cargo inspect main.rs
```

If you don't specify a file, the current crate will be analyzed instead.

```
cargo inspect
```

Depending on the size of the crate, this might take a while.  
Please be patient.

It can also compare two file outputs! Try this:

```
cargo inspect --diff examples/range.rs,examples/range_inclusive.rs --plain
```

## Configuration 

```
USAGE:
    cargo inspect [FLAGS] [OPTIONS] [INPUT_FILE]

FLAGS:
    -h, --help
            Prints help information

        --list-themes
            Should we list all pretty printer themes?

        --plain
            Don't highlight output

    -V, --version
            Prints version information

    -v, --verbose
            Print the original code as a comment above the desugared code


OPTIONS:
        --theme <THEME>
            Specify a theme override for the pretty printer

        --diff <files>
            Diff input files

        --format <format>
            Override for the format that gets outputted when the `unpretty` mode is set to `flowgraph` [default: svg]

        --unpretty <unpretty>
            rustc "unpretty" parameters

            *Note*: For `--unpretty=flowgraph=[symbol]` you need to have `dot` on your PATH. [default: hir]

ARGS:
    <INPUT_FILE>
            Input file
```

## Background

Rust allows for a lot of syntactic sugar, that makes it a pleasure to write. It
is sometimes hard, however, to look behind the curtain and see what the compiler
is really doing with our code.

To quote @tshepang, "It is good to know what these conveniences are, to avoid
being mystified by what's going on under the hood... the less magical thinking
we have of the world, the better."

* lifetime elisions
* type inference
* syntactic sugar
* implicit dereferencing
* type coercions
* hidden code (e.g. the prelude)

I was always interested in how programming languages work in the background, how
my code was unrolled to make the compiler backend easier to maintain.

The goal is to make the compiler more approachable for mere mortals.  
Mystery! Exploration! Discovery! 

Read more on the background of `cargo-inspect` [on my blog](https://matthias-endler.de/2018/cargo-inspect/).

## Code Examples

### `If-let` gets desugared into `match`

Consider the following code snippet:

```rust
fn main() {
    if let Some(x) = Some(1) {
        // Do something with x
    }
}
```

When you compile it, the first thing Rust does is desugar it. To see what the
code looks like after this step, run

```
cargo inspect examples/if_let.rs
```

This produces the following output:

![Please run the command to reproduce the desugared output](assets/if-let.png)

You can see that the `if let` was desugared into a `match` statement.

To change the colorscheme, try `cargo-inspect --list-themes`, e.g.

```
cargo inspect examples/if_let.rs --theme GitHub
```

![Please run the command to reproduce the desugared output](assets/if-let-white.png)

Oh, and if you have [`graphviz`](https://graphviz.gitlab.io/download/) installed, you can also print a pretty flowgraph from your code:

```
cargo inspect --unpretty=flowgraph=main examples/if_let.rs
```

![Please run the command to reproduce the desugared output](assets/if-let-flowgraph.svg)

### More examples

Please find more examples in the `examples` folder. You can also contribute
more.

## The Magic Sauce

The best things in the world are assembled from simple building blocks. This
tool stands on the shoulders of giants. To work its magic, it runs the following
commands:

1. `rustc -Zinspect=hir`, for retrieving the HIR.
2. `rustfmt`, for formatting the output.
3. [`prettyprint`](https://github.com/mre/prettyprint), for syntax-highlighting,
   which is just a wrapper around the awesome
   [syntect](https://github.com/trishume/syntect/blob/master/examples/syncat.rs)
   and [bat](https://github.com/sharkdp/bat/) crates.

## Contributing

This is a young project, which has downsides and upsides.

* Everything is in flux and things can break at any time. 😫
* There's plenty of opportunity to shape and form the project. 😊

Thus, become a contributor today!

## Known issues

As of now, this is a very fragile tool. If it fails, it ~~might~~ will produce
horrible output. You have been warned. That said, it won't eat your code, of
course. :blush:

## License

Licensed under either of

* Apache License, Version 2.0, (LICENSE-APACHE or
  http://www.apache.org/licenses/LICENSE-2.0)
* MIT license (LICENSE-MIT or http://opensource.org/licenses/MIT)

at your option.

[rustup]: https://rustup.rs/


## Credits

Magnifying glass designed by [Rawpixel.com]( https://www.freepik.com/free-vector/illustration-of-a-magnifying-glass_2945064.htm) 
