use super::attr_model::AttrModel;

pub struct ObjectModel {
    sys_id: String,

    attr_state: AttrModel,
    attr_dir: AttrModel,
    // attrs_map: HashMap<u32, AttrModel>,
}

impl ObjectModel {
    pub fn new(sys_id: String) -> Self {
        let attr_state = AttrModel::new(1, 11);
        // let map = HashMap::new();
        //     map.insert(attr_state.id, attr_state)
        Self {
            sys_id,
            attr_state,
            attr_dir: AttrModel::new(2, 22),
        }
    }

    pub fn print(&self) {
        println!("from obj: {}", self.sys_id)
    }

    pub fn get_attr_value(&self, attr_id: u32) -> u32 {
        let attrs = self.attrs();

        for &a in attrs.iter() {
            if a.id == attr_id {
                return a.value;
            }
        }

        return 0;

        // match attr_id {
        //     1 => self.attr_state.value,
        //     2 => self.attr_dir.value,
        //     _ => 0,
        // }
    }

    fn attrs(&self) -> [&AttrModel; 2] {
        return [&self.attr_state, &self.attr_dir];
    }
}
