// last 71 page
mod state;
mod to_do;

use std::env;

use crate::to_do::traits::delete::Delete;
use crate::to_do::traits::edit::Edit;
use crate::to_do::traits::get::Get;
use serde_json::json;
use serde_json::Map;
use serde_json::Value;
use to_do::enums::TaskStatus;
// use to_do::structs::done::Done;
// use to_do::structs::pending::Pending;
use to_do::to_do_factory;
use to_do::ItemTypes;

fn main() {
    // println!("Hello, world!");

    // let done = Done::new("shopping");
    // println!("{}", done.super_struct.title);
    // println!("{}", done.super_struct.status.stringify());
    //
    // let pending = Pending::new("laundry");
    // println!("{}", pending.super_struct.title);
    // println!("{}", pending.super_struct.status.stringify());
    //
    //

    // ----
    let to_do_item = to_do_factory("washing", TaskStatus::DONE);

    match to_do_item {
        ItemTypes::Done(item) => {
            item.get(&item.super_struct.title);
            item.delete(&item.super_struct.title);
            // println!("{}", item.super_struct.status.stringify());
            // println!("{}", item.super_struct.title);
        }
        ItemTypes::Pending(item) => {
            item.get(&item.super_struct.title);
            item.set_to_done(&item.super_struct.title);
            // println!("{}", item.super_struct.status.stringify());
            // println!("{}", item.super_struct.title);
        }
    }

    // read write example ---------------------------------
    let args: Vec<String> = env::args().collect();
    let status: &String = &args[1];
    let title: &String = &args[2];

    let mut state: Map<String, Value> = state::read_file("/tmp/state.json");
    println!("Before operation: {:?}", state);

    state.insert(title.to_string(), json!(status));
    println!("After operation: {:?}", state);

    state::write_to_file("/tmp/state.json", &mut state);
}
