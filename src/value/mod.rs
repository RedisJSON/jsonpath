use serde_json::{Number};
use std::slice::Iter;

pub enum DocValueType<V: DocValue> {
    Null,
    Bool(bool),
    Number(Number),
    String(String),
    Array(V::A),
    Object(V::M),
}

pub trait DocArr<V: DocValue> : IntoIterator<Item = V> {
    fn iter(&self) -> Iter<'_, V>;
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
    fn get(&self, index: usize) -> Option<&Self>;
    fn as_object(&self) -> Option<&Self::M>;
    fn as_object_mut(&mut self) -> Option<&mut Self::M>;
    fn as_array_mut(&mut self) -> Option<&mut Self::A>;
    fn as_array(&mut self) -> Option<&Self::A>;
    fn as_bool(&self) -> Option<bool>;
}