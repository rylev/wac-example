mod bindings;

use bindings::Guest;

struct Component;

impl Guest for Component {
    /// Say hello!
    fn greet() -> String {
        format!("{}, World!", bindings::hello())
    }
}
