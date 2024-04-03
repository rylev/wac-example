mod bindings;

use bindings::Guest;

struct Component;

impl Guest for Component {
    /// Say hello!
    fn hello() -> String {
        "Hello".to_string()
    }
}
