# Wac Example

This repo contains two different but equal ways for composing two example components together
* using the `wac` CLI
* using the `wac-graph` library crate

The `hello` component exports a function `hello` which returns a string. This will be plugged into the `hello` import of the `greeter` component. The resulting component will therefore only have the exported `greet` function originally from the `greeter` component.

*Note*: the `hello` and `greeter` components are built with `cargo component --target=wasm32-unknown-unknown` so the definitions of these component's worlds found in their corresponding `wit` directories are complete meaning that they do not implicitly import any wasi interfaces.

## CLI

`wac` can be used as a CLI tool. Running the following command should produce a new component that is the composition of the `hello` and `greeter` components.

```
wac encode composition.wac -d example:greeting=world.wit -o composed.wasm  
```

*Note*: the `-d example:greeting=world.wit` is an argument for specifying that the `example:greeting` wit dependency can be found in the `world.wit` file.

## Programmatic Graph API

You can also build the composition using the programmatic API used by the `programmatic` example binary. This can be done simply by running the following inside the `programmatic` directory:

```
cargo run
```

## Repo Structure

The repo is composed of the following parts:
* `build`: directory which contains the source code for the two components being composed.
* `deps`: which contains the build artifacts for the two components being composed (the source for which can be found in `build`)
* `composition.wac`: the composition file for performing composition using the `wac` CLI tool
* `programmatic`: a Rust binary that uses the programmatic API for composing the two components together in the same way that `composition.wac` does.
