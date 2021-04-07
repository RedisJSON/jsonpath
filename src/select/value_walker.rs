use std::collections::HashSet;
use value::JsonValue;

pub(super) struct ValueWalker;

impl<'a> ValueWalker {
    pub fn all_with_num(vec: &[&'a JsonValue], tmp: &mut Vec<&'a JsonValue>, index: f64) {
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

    pub fn all_with_str(vec: &[&'a JsonValue], tmp: &mut Vec<&'a JsonValue>, key: &str, is_filter: bool) {
        if is_filter {
            Self::walk(vec, tmp, &|v| match v.get_type() {
                JsonValue::Object(map) if map.contains_key(key) => Some(vec![v]),
                _ => None,
            });
        } else {
            Self::walk(vec, tmp, &|v| match v.get_type() {
                JsonValue::Object(map) => match map.get(key) {
                    Some(v) => Some(vec![v]),
                    _ => None,
                },
                _ => None,
            });
        }
    }

    pub fn all(vec: &[&'a JsonValue], tmp: &mut Vec<&'a JsonValue>) {
        Self::walk(vec, tmp, &|v| match v.get_type() {
            JsonValue::Array(vec) => {
                Some(vec.iter().collect())
            },
            JsonValue::Object(map) => {
                let mut tmp = Vec::new();
                for (_, v) in map {
                    tmp.push(v);
                }
                Some(tmp)
            }
            _ => None,
        });
    }

    fn walk<F>(vec: &[&'a JsonValue], tmp: &mut Vec<&'a JsonValue>, fun: &F) where F: Fn(&JsonValue) -> Option<Vec<&JsonValue>> {
        for v in vec {
            Self::_walk::<F>(v, tmp, fun);
        }
    }

    fn _walk<F>(v: &'a JsonValue, tmp: &mut Vec<&'a JsonValue>, fun: &F) where F: Fn(&JsonValue) -> Option<Vec<&JsonValue>> {
        if let Some(mut ret) = fun(v) {
            tmp.append(&mut ret);
        }

        match v.get_type() {
            JsonValue::Array(vec) => {
                for v in vec {
                    Self::_walk(v, tmp, fun);
                }
            }
            JsonValue::Object(map) => {
                for (_, v) in map {
                    Self::_walk(&v, tmp, fun);
                }
            }
            _ => {}
        }
    }

    pub fn walk_dedup(v: &'a JsonValue,
                      tmp: &mut Vec<&'a JsonValue>,
                      key: &str,
                      visited: &mut HashSet<*const JsonValue>, ) {
        match v.get_type() {
            JsonValue::Object(map) => {
                if map.contains_key(key) {
                    let ptr = v as *const JsonValue;
                    if !visited.contains(&ptr) {
                        visited.insert(ptr);
                        tmp.push(v)
                    }
                }
            }
            JsonValue::Array(vec) => {
                for v in vec {
                    Self::walk_dedup(v, tmp, key, visited);
                }
            }
            _ => {}
        }
    }
}

