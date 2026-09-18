#[derive(Debug)]
pub struct AttrModel {
    pub id: u32,
    pub value: u32,
}

impl AttrModel {
    pub fn new(id: u32, initial_value: u32) -> Self {
        Self {
            id,
            value: initial_value,
        }
    }
}
