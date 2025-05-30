# Seastar
Seastar is a fast, extensible build system for C, and soon, C++ and Rust as well.
I believe that it should be easy to make, prototype, and iterate upon designs.
However, C is still one of our most widely used languages, but it makes it hard to create
programs easily, especially for beginners. Instead, Seastar aims to be more like
Rust's tooling with `cargo`, but supporting seamless compilation across more languages.

## Running
Seastar is very simple to build and run. Assuming you have Cargo and Rust installed,
clone the repository, `cd` into the `example` folder, and run `cargo run -- build`
to run Seastar and build the example project, or `cargo run -- run` to run the example
project. Check `example/Seastar.toml` to make sure that you have the compiler
installed and correctly set in that file.

## Roadmap
Seastar is still in a very early state, and thus I wouldn't recommend using it
currently for anything serious. Below, however, you can see my roadmap, and if
you want to get updates, you can watch this repository.

* [X] Sort of working: Being able to compile and link a simple project with multiple files and include headers.
* [X] Incremental builds: The entire program shouldn't be recompiled every time a single file is changed.
* [ ] Custom compiler flags: The programmer should be able to customize the compiler flags through `Seastar.toml` without needing to change the build/run commands.
* [ ] Parallel builds: Compiling in parallel is faster and more efficient.
* [ ] C++ support: Seastar should be able to compile and link C++ without changing options or difficult configuration.
* [ ] Rust support: Seastar should be able to compile and link Rust without changing options or difficult configuration.
* [ ] Easy template generation: We should be able to create templates with a single command, e.g. `seastar init --lang c`
* [ ] *Unified package manager: a difficult goal, but it should be easy to install packages for C, C++, and Rust neatly and natively to Seastar.*

If you find this project cool and want to see more , please consider leaving a star!
It supports the development of this project and is really helpful.