# bachelor_thesis

My thesis about algorithms finding large bipartite subgraphs written in the Rust language.

A structure representing undirected graphs, a crawler to scan a websites network and the algorithm introduced in "Finding bipartite subgraphs efficiently" by Dhruv Mubayi and Gyorgy Turan were implemented.

## Installment

Make sure to install Rust environment together with Cargo tool according to the instruction at [Rust official page](https://www.rust-lang.org/tools/install).

The whole implementation was written in Rust 1.62.0 and is guaranteed to work with this version or a newer one.

After installment please run
``` cargo build --lib ```
which will build a file named
``` liblabisu.rlib ```
that can be later referenced in other projects in Rust by adding a dependency to the file in the new project ``` Cargo.toml ``` file.

## Documentation

The documentation is not attached as it can be easily generated using the Cargo tool.
One needs to simply run the ``` cargo doc ``` command to build a documentation in HTML that can be open in any browser.

Moreover each function or method that can be used by other projects is well documented in the source code itself.
