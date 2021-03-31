use std::collections::HashSet;
use value::{JsonValue, JsonValueType};

pub(super) struct ValueWalker;

impl<'a> ValueWalker {
    pub fn all_with_num<T: JsonValue>(vec: &[&'a T], tmp: &mut Vec<&'a T>, index: f64) {
        Self::walk(vec, tmp, &|v| if v.is_array() {
            if let Some(item) = v.get(index as usize) {
                Some(vec![item])
            } else {
                None
            }
        } else {
            None
        });
    }

    pub fn all_with_str<T: JsonValue>(vec: &[&'a T], tmp: &mut Vec<&'a T>, key: &str, is_filter: bool) {
        if is_filter {
            Self::walk(vec, tmp, &|v| match v.getType() {
                JsonValueType::Object(map) if map.contains_key(key) => Some(vec![v]),
                _ => None,
            });
        } else {
            Self::walk(vec, tmp, &|v| match v.getType() {
                JsonValueType::Object(map) => match map.get(key) {
                    Some(v) => Some(vec![v]),
                    _ => None,
                },
                _ => None,
            });
        }
    }

    pub fn all<T: JsonValue>(vec: &[&'a T], tmp: &mut Vec<&'a T>) {
        Self::walk(vec, tmp, &|v| match v.getType() {
            JsonValueType::Array(vec) => {
                let vec = vec.as_array().unwrap();
                Some(vec.iter().collect())
            },
            JsonValueType::Object(map) => {
                let map = map.as_object().unwrap();
                let mut tmp = Vec::new();
                for (_, v) in map {
                    tmp.push(v);
                }
                Some(tmp)
            }
            _ => None,
        });
    }

    fn walk<F, T: JsonValue>(vec: &[&'a T], tmp: &mut Vec<&'a T>, fun: &F) where F: Fn(&T) -> Option<Vec<&T>> {
        for v in vec {
            Self::_walk::<F,T>(v, tmp, fun);
        }
    }

    fn _walk<F, T: JsonValue>(v: &'a T, tmp: &mut Vec<&'a T>, fun: &F) where F: Fn(&T) -> Option<Vec<&T>> {
        if let Some(mut ret) = fun(v) {
            tmp.append(&mut ret);
        }

        match v.getType() {
            JsonValueType::Array(vec) => {
                let vec = vec.as_array().unwrap();
                for v in vec {
                    Self::_walk(v, tmp, fun);
                }
            }
            JsonValueType::Object(map) => {
                let map = map.as_object().unwrap();
                for (_, v) in map {
                    Self::_walk(&v, tmp, fun);
                }
            }
            _ => {}
        }
    }

    pub fn walk_dedup<T: JsonValue>(v: &'a T,
                      tmp: &mut Vec<&'a T>,
                      key: &str,
                      visited: &mut HashSet<*const T>, ) {
        match v.getType() {
            JsonValueType::Object(map) => {
                let map = map.as_object().unwrap();
                if map.contains_key(key) {
                    let ptr = v as *const T;
                    if !visited.contains(&ptr) {
                        visited.insert(ptr);
                        tmp.push(v)
                    }
                }
            }
            JsonValueType::Array(vec) => {
                let vec = vec.as_array().unwrap();
                for v in vec {
                    Self::walk_dedup(v, tmp, key, visited);
                }
            }
            _ => {}
        }
    }
}

