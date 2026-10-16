// last 84 page
mod processes;
mod state;
mod to_do;

use std::env;

use processes::process_input;
use serde_json::Map;
use serde_json::Value;
use to_do::enums::TaskStatus;
use to_do::to_do_factory;

fn main() {
    let args: Vec<String> = env::args().collect();
    let command: &String = &args[1];
    let title: &String = &args[2];

    let state: Map<String, Value> = state::read_file("/tmp/state.json");
    let status: String;

    match &state.get(*&title) {
        Some(result) => status = result.to_string().replace('\"', ""),
        None => status = "pending".to_owned(),
    }

    let item = to_do_factory(title, TaskStatus::from_string(status.to_uppercase()));
    process_input(item, command.to_string(), &state);

    // match to_do_item {
    //     ItemTypes::Done(item) => {
    //         item.get(&item.super_struct.title);
    //         item.delete(&item.super_struct.title);
    //         // println!("{}", item.super_struct.status.stringify());
    //         // println!("{}", item.super_struct.title);
    //     }
    //     ItemTypes::Pending(item) => {
    //         item.get(&item.super_struct.title);
    //         item.set_to_done(&item.super_struct.title);
    //         // println!("{}", item.super_struct.status.stringify());
    //         // println!("{}", item.super_struct.title);
    //     }
    // }

    // // read write example ---------------------------------

    // println!("Before operation: {:?}", state);

    // state.insert(title.to_string(), json!(status));
    // println!("After operation: {:?}", state);

    // state::write_to_file("/tmp/state.json", &mut state);
}
