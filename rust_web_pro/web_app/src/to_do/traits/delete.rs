use serde_json::{Map, Value};

use crate::state::write_to_file;

pub trait Delete {
    fn delete(&self, title: &String, state: &mut Map<String, Value>) {
        state.remove(title);
        // TODO: file_name
        write_to_file("/tmp/state.json", state);

        println!("\n\n{} is being deleted\n\n", title);
    }
}
