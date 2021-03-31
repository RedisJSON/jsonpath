use serde_json::Value;
use serde_json::map::Map;

pub enum JsonValueType<'a, J : 'a + JsonValue> {
    Null(&'a J),
    Bool(&'a J),
    Number(&'a J),
    String(&'a J),
    Array(&'a J),
    Object(&'a J),
}

// pub trait Arr<J : JsonValue > {
// }

// pub trait Map<J : JsonValue > {
// }

pub trait JsonValue : std::fmt::Debug + std::default::Default{
    fn getType(&self) -> JsonValueType<Self>;
    fn is_array(&self) -> bool;
    // fn get<I: Index>(&self, index: I) -> Option<&JsonValue> {
    fn as_object(&self) -> Option<&Map<String, Value>>;
    fn as_object_mut(&mut self) -> Option<&mut Map<String, Value>>;
    fn as_array_mut(&mut self) -> Option<&mut Vec<Value>>;
    fn as_array(&mut self) -> Option<&Vec<Value>>;
    fn as_bool(&self) -> Option<bool>;
}

impl JsonValue for Value {
    fn getType(&self) -> JsonValueType<Value>{
        match self {
            Null => JsonValueType::Null(self),
            Bool => JsonValueType::Bool(self),
            Number => JsonValueType::Number(self),
            String => JsonValueType::String(self),
            Value::Array(v) => JsonValueType::Array(self),
            Value::Object(o) => JsonValueType::Object(self),
        }
    }

    fn is_array(&self) -> bool {
        self.is_array()
    }

    fn as_object(&self) -> Option<&Map<String, Value>> {
        self.as_object()
    }

    fn as_object_mut(&mut self) -> Option<&mut Map<String, Value>> {
        self.as_object_mut()
    }

    fn as_array_mut(&mut self) -> Option<&mut Vec<Value>> {
        self.as_array_mut()
    }

    fn as_array(&mut self) -> Option<&Vec<Value>> {
        self.as_array()
    }

    fn as_bool(&self) -> Option<bool> {
        self.as_bool()
    }

}

// impl Map<Value> for map::Map<String, Value> {
// }

// impl Arr<Value> for Vec<Value> {
// }