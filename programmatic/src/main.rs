use wac_graph::{CompositionGraph, EncodeOptions};
use wac_types::Package;

fn main() {
    let mut graph = CompositionGraph::new();

    // Register the package dependencies into the graph
    let package = Package::from_file("hello", None, "../deps/example/hello.wasm").unwrap();
    let hello = graph.register_package(package).unwrap();
    let package = Package::from_file("greeter", None, "../deps/example/greeter.wasm").unwrap();
    let greeter = graph.register_package(package).unwrap();

    // Instantiate the hello instance which does not have any arguments
    let hello_instance = graph.add_instantiation_node(hello, ()).unwrap();

    // Instantiate the greeter instance which has a single argument "hello" which is exported by the hello instance
    let greeter_instance = graph.add_instantiation_node(greeter, ()).unwrap();
    let hello_export = graph.add_alias_node(hello_instance, "hello", ()).unwrap();
    graph
        .add_argument_edge(hello_export, greeter_instance, "hello")
        .unwrap();

    // Export the "greet" from the greeter instance
    let greet_export = graph.add_alias_node(greeter_instance, "greet", ()).unwrap();
    graph.export_node(greet_export, "greet").unwrap();

    // Encode the graph into a WASM binary
    let encoding = graph.encode(EncodeOptions::default()).unwrap();
    std::fs::write("composition.wasm", encoding).unwrap();
}
