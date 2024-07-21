// last 71 page
mod to_do;

use crate::to_do::traits::delete::Delete;
use crate::to_do::traits::edit::Edit;
use crate::to_do::traits::get::Get;
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
}
