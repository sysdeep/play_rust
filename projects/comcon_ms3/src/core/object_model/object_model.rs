use super::attr_model::AttrModel;

pub struct ObjectModel {
    sys_id: String,

    attr_state: AttrModel,
    attr_dir: AttrModel,
}

impl ObjectModel {
    pub fn new(sys_id: String) -> Self {
        let attr_state = AttrModel::new(1);
        Self {
            sys_id,
            attr_state,
            attr_dir: AttrModel::new(2),
        }
    }

    pub fn print(&self) {
        println!("from obj: {}", self.sys_id)
    }

    pub fn get_attr_value(&self, attr_id: u32) -> u32 {
        match attr_id {
            1 => self.attr_state.value,
            2 => self.attr_dir.value,
            _ => 0,
        }
    }
}
