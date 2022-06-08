# MyStudio IDE

An IDE built from scratch in Rust as a learning experience.

### Prerequisites

1. [Rust](https://rust-lang.org) (min v1.56)
2. GTK3+
3. gtk-rs (0.15+)

### Get Started

1. Clone this repository.

2. `cargo run`

> The project will compile and you should see the IDE.

### Project Structure

This project uses [Cargo Workspaces](https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html). It contains two projects:

1. __libmystudio__: A library which consists model definitions, filesystem management and caching layer for `GtkNotebook`.

2. __mystudio-ide__: The binary application for this project. It depends on `libmystudio` and builds an executable.


## Contributing

PRs are appreciated for bug fixes. In the case of feature requests, create an issue on GitHub so we can discuss it before you spend a lot of time on it :) 
