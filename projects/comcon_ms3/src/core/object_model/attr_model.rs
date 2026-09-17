//#[derive(Debug)]
pub struct AttrModel {
    id: u32,
    pub value: u32,
}

impl AttrModel {
    pub fn new(id: u32) -> Self {
        Self { id, value: 22 }
    }
}
