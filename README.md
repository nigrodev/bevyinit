# bevyinit

**bevyinit** makes it easy to create a [Bevy Engine](https://bevyengine.org/) project with ready-to-use templates and useful options

> If you're looking for the place where the online templates are located, take a look at [bevyinit_data](https://github.com/nigrodev/bevyinit_data).

## Usage
Just install it and run `bevyinit` in your terminal

![example](repo/example.gif)

## Installation

#### [cargo-binstall](https://crates.io/crates/cargo-binstall)

```bash
cargo binstall bevyinit
```

#### crates.io

```bash
cargo install bevyinit
```

#### Github Releases

You can check out the
[releases](https://github.com/nigrodev/bevyinit/releases), but you have to add it to the PATH manually.

## FAQ

### Why use this?
Because **bevyinit** is a more practical and faster way to create projects using Bevy with [recommended configuration options](https://bevyengine.org/learn/book/getting-started/setup/). It currently has few templates, but it's very easy to create new ones, just take a look at [Extras](#extras)

### Does it work with Linux or Mac?
I've only tested it with Windows and Ubuntu WSL. It's meant to work with any OS that has cargo in the PATH.

If an error occurs, please open an Issue on GitHub.

## Planned
- Add support for extra dependencies besides Bevy in templates
- Find out a way to open the folder with Visual Studio Code
- More options...?

## Extras
It's very easy to create new templates. These are the ones available:
| Templates | 
| --- |
| [Minimal Bevy App](templates/minimal.ron) |
| [Hello World Example](templates/hello_world.ron) |