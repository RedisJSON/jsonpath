
use serde_json::{Number, Value, Map};
use value::{DocArr, DocMap, DocValue, DocValueType};
use std::slice::Iter;

#[derive(Debug)]
pub struct JsonArr<'a> {
    vec : &'a Vec<Value>,
}

pub struct JsonArrIterator {
    //TODO impl state
}

impl Iterator for JsonArrIterator {
    type Item = DocValueType<Value>;
    fn next(&mut self) -> Option<Self::Item> {
        None //TODO impl
    }
}

impl <'a> IntoIterator for JsonArr<'a> {
    type Item = DocValueType<Value>;
    type IntoIter = JsonArrIterator;
    fn into_iter(self) -> Self::IntoIter {
        JsonArrIterator{}
    }
}

impl <'a> DocArr<Value> for JsonArr<'a> {
    fn iter(&self) -> Iter<'_, DocValueType<Value>>{
        Iter::new(&[]) // TODO
    }

    fn len(&self) -> usize {
        0 // TODO
    }
    fn remove(&self, index: usize) -> DocValueType<Value> {
        DocValueType::Null // Todo
    }
    fn get_mut(&self, index: usize) -> &mut DocValueType<Value> {
        &mut DocValueType::Null // Todo
    }

    fn get(&self, index: usize) -> &DocValueType<Value> {
        &mut DocValueType::Null // Todo
    }
}

impl <'a> core::ops::Index<usize> for JsonArr<'a> {
    type Output = &'a Value;
    
    fn index(&self, index: usize) -> &'a Value {
        &self.vec[index] // TODO 
    }
}

#[derive(Debug)]
pub struct JsonMap<'a> {
    map : &'a Map<String, Value>,
}

pub struct JsonMapIterator {
    //TODO impl state
}

impl Iterator for JsonMapIterator {
    type Item = (String, DocValueType<Value>);
    fn next(&mut self) -> Option<Self::Item> {
        None //TODO impl
    }
}

impl <'a> IntoIterator for JsonMap<'a> {
    type Item=(String, DocValueType<Value>);
    type IntoIter = JsonMapIterator;
    fn into_iter(self) -> Self::IntoIter {
        JsonMapIterator{}
    }
}

impl <'a> DocMap<Value> for JsonMap<'a> {
    fn get(&self, key: &str) -> Option<&'a DocValueType<Value>>{
        None //TODO
    }
    fn contains_key(&self, key: &str) -> bool{
        true // TODO 
    }
}


impl DocValue for Value {

    type M<'a> = JsonMap<'a>;
    type A<'a> = JsonArr<'a>;
    
    // fn get_type<'a>(&'a self) -> DocValueType<Value>{
    //     match self {
    //         Value::Null => DocValueType::Null,
    //         Value::Bool(b) => DocValueType::Bool(b.clone()),
    //         Value::Number(n) => DocValueType::Number(n.clone()),
    //         Value::String(s) => DocValueType::String(s.clone()),
    //         Value::Array(v) => DocValueType::Array(JsonArr{vec: &v}),
    //         Value::Object(o) => DocValueType::Object(JsonMap{map: &o}),
    //     }
    // }

    // fn is_array(&self) -> bool {
    //     Value::is_array(self)
    // }

    // fn get(&self, index: usize) -> Option<&Self>{
    //     None // TODO
    // }

    // fn as_object(&self) -> Option<&Self::M> {
    //     Value::as_object(self)
    // }

    // fn as_object_mut(&mut self) -> Option<&mut Self::M> {
    //     Value::as_object_mut(self)
    // }

    // fn as_array_mut(&mut self) -> Option<&mut Self::A> {
    //     Value::as_array_mut(self)
    // }

    // fn as_array(&mut self) -> Option<&Self::A> {
    //     Value::as_array(self)
    // }

    // fn as_bool(&self) -> Option<bool> {
    //     Value::as_bool(self)
    // }
}