use serde_json::{Value, Number};
use serde_json::map::Map;

pub enum DocValueType<'a, V : 'a + DocValue> {
    Null,
    Bool(&'a bool),
    Number(&'a Number),
    String(&'a String),
    Array(&'a V::A),
    Object(&'a V::M),
}

pub trait DocArr<V: DocValue> : IntoIterator<Item = V> {
}

pub trait DocMap<V: DocValue> : IntoIterator<Item = (String,V)> {
    fn get(&self, key: &str) -> Option<&V>;
    fn contains_key(&self, key: &str) -> bool;
}

pub trait DocValue : std::fmt::Debug + std::default::Default{
    type M: DocMap<Self>;
    type A: DocArr<Self>;

    fn get_type(&self) -> DocValueType<Self>;
    fn is_array(&self) -> bool;
    // fn get<I: Index>(&self, index: I) -> Option<&DocValue> {
    fn as_object(&self) -> Option<&Self::M>;
    fn as_object_mut(&mut self) -> Option<&mut Self::M>;
    fn as_array_mut(&mut self) -> Option<&mut Self::A>;
    fn as_array(&mut self) -> Option<&Self::A>;
    fn as_bool(&self) -> Option<bool>;
}


impl DocMap<Value> for Map<String, Value> {
    fn get(&self, key: &str) -> Option<&Value>{
        Map::get(self, key)
    }
    fn contains_key(&self, key: &str) -> bool{
        Map::contains_key(self, key)
    }
}

impl DocArr<Value> for Vec<Value> {
}

impl DocValue for Value {

    type M = Map<String, Value>;
    type A = Vec<Value>;
    
    fn get_type(&self) -> DocValueType<Value>{
        match self {
            Value::Null => DocValueType::Null,
            Value::Bool(b) => DocValueType::Bool(b),
            Value::Number(n) => DocValueType::Number(n),
            Value::String(s) => DocValueType::String(s),
            Value::Array(v) => DocValueType::Array(v),
            Value::Object(o) => DocValueType::Object(o),
        }
    }

    fn is_array(&self) -> bool {
        Value::is_array(self)
    }

    fn as_object(&self) -> Option<&Self::M> {
        Value::as_object(self)
    }

    fn as_object_mut(&mut self) -> Option<&mut Self::M> {
        Value::as_object_mut(self)
    }

    fn as_array_mut(&mut self) -> Option<&mut Self::A> {
        Value::as_array_mut(self)
    }

    fn as_array(&mut self) -> Option<&Self::A> {
        Value::as_array(self)
    }

    fn as_bool(&self) -> Option<bool> {
        Value::as_bool(self)
    }
}