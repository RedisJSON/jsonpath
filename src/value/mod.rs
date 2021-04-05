use serde_json::{Number};
use std::slice::Iter;

mod json;

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum DocValueType<V: DocValue> {
    Null,
    Bool(bool),
    Number(Number),
    String(String),
    Array(V::A),
    Object(V::M),
}

impl <V: DocValue> DocValueType<V> {

    // fn get_type<'a>(&'a self) -> DocValueType<V>{

    // }
    // fn from_type(DocValueType<Self>) -> Self{

    // }
    pub fn is_array(&self) -> bool{
        if let DocValueType::Array(_) = self {
            true
        } else {
            false
        }
    }
    pub fn get(&self, index: usize) -> Option<&DocValueType<V>>{
        None // TODO
    }
    // fn as_object(&self) -> Option<&V::M>{

    // }
    // fn as_object_mut(&mut self) -> Option<&mut V::M>{

    // }

    // fn as_array_mut(&mut self) -> Option<&mut V::A>{

    // }
    // fn as_array(&mut self) -> Option<&V::A>{

    // }
    // fn as_bool(&self) -> Option<bool>{

    // }

}

pub trait DocValue : std::fmt::Debug + std::default::Default{
    type M: DocMap<Self>;
    type A: DocArr<Self>;

    // fn is_array(&self) -> bool;
    // fn get(&self, index: usize) -> Option<&Self>;
    // fn as_object(&self) -> Option<&Self::M>;
    // fn as_object_mut(&mut self) -> Option<&mut Self::M>;
    // fn as_array_mut(&mut self) -> Option<&mut Self::A>;
    // fn as_array(&mut self) -> Option<&Self::A>;
    // fn as_bool(&self) -> Option<bool>;
}

pub trait DocArr<V: DocValue> : std::fmt::Debug + core::ops::Index<usize> + IntoIterator<Item = DocValueType<V>> {
    fn iter(&self) -> Iter<'_, DocValueType<V>>;
    fn len(&self) -> usize;
    fn remove(&self, index: usize) -> DocValueType<V>;
    fn get_mut(&self, index: usize) -> &mut DocValueType<V>;
    fn get(&self, index: usize) -> &DocValueType<V>;
}

pub trait DocMap<V: DocValue> : std::fmt::Debug + IntoIterator<Item = (String,DocValueType<V>)> {
    fn get(&self, key: &str) -> Option<&DocValueType<V>>;
    fn contains_key(&self, key: &str) -> bool;
}

