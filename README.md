# Wac Example

Running the following command should produce a new component that is the composition of the `hello` and `greeter` components.

```
wac encode composition.wac -d example:greeting=world.wit -o composed.wasm  
```

The `hello` component exports a function `hello` which returns a string. This will be plugged into the `hello` import of the `greeter` component. The resulting component will therefore only have the exported `greet` function originally from the `greeter` component.

*Note*: the `hello` and `greeter` components are built with `cargo component --target=wasm32-unknown-unknown` so the definitions of these component's worlds found in their corresponding `wit` directories are complete meaning that they do not implicitly import any wasi interfaces.
