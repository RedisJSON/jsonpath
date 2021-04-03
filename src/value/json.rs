
pub struct JsonArr<'a> {
    vec : &'a Vec<Value>,
}

pub struct JsonArrIterator {
    //TODO impl state
}

impl Iterator for JsonArrIterator {
    type Item = Value;
    fn next(&mut self) -> Option<Self::Item> {
        None //TODO impl
    }
}

impl <'a> IntoIterator for JsonArr<'a> {
    type Item = Value;
    type IntoIter = JsonArrIterator;
    fn into_iter(self) -> Self::IntoIter {
        JsonArrIterator{}
    }
}

impl <'a> DocArr<Value> for JsonArr<'a> {
}

pub struct JsonMap<'a> {
    map : &'a Map<String, Value>,
}

pub struct JsonMapIterator {
    //TODO impl state
}

impl Iterator for JsonMapIterator {
    type Item = (String,Value);
    fn next(&mut self) -> Option<Self::Item> {
        None //TODO impl
    }
}

impl <'a> IntoIterator for JsonMap<'a> {
    type Item=(String, Value);
    type IntoIter = JsonMapIterator;
    fn into_iter(self) -> Self::IntoIter {
        JsonMapIterator{}
    }
}

impl <'a> DocMap<Value> for JsonMap<'a> {
    fn get(&self, key: &str) -> Option<&'a Value>{
        None //TODO
    }
    fn contains_key(&self, key: &str) -> bool{
        true // TODO 
    }
}


impl DocValue for Value {

    type M = JsonMap;
    type A = JsonArr;
    
    fn get_type(&self) -> DocValueType<Value>{
        match self {
            Value::Null => DocValueType::Null,
            Value::Bool(b) => DocValueType::Bool(b.clone()),
            Value::Number(n) => DocValueType::Number(n.clone()),
            Value::String(s) => DocValueType::String(s.clone()),
            Value::Array(v) => DocValueType::Array(v),
            Value::Object(o) => DocValueType::Object(Map{map:o}),
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