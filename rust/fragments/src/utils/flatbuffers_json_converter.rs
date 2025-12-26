use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq)]
pub enum FlatbuffersValue {
    Null,
    Bool(bool),
    Number(f64),
    String(String),
    Array(Vec<FlatbuffersValue>),
    Object(BTreeMap<String, FlatbuffersValue>),
}

pub trait FlatbuffersObject {
    fn fields(&self) -> Vec<(String, FlatbuffersValue)>;
}

pub fn get_object(obj: &dyn FlatbuffersObject) -> FlatbuffersValue {
    let mut map = BTreeMap::new();
    for (name, value) in obj.fields() {
        map.insert(name, value);
    }
    FlatbuffersValue::Object(map)
}
