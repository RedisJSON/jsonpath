use std::collections::HashSet;
use value::{DocValue, DocValueType, DocMap, DocArr};

pub(super) struct ValueWalker;

impl<'a> ValueWalker {
    pub fn all_with_num<T: DocValue>(vec: &[&'a DocValueType<T>], tmp: &mut Vec<&'a DocValueType<T>>, index: f64) {
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

    pub fn all_with_str<T: DocValue>(vec: &[&'a DocValueType<T>], tmp: &mut Vec<&'a DocValueType<T>>, key: &str, is_filter: bool) {
        if is_filter {
            Self::walk(vec, tmp, &|v| match v {
                DocValueType::Object(map) if map.contains_key(key) => Some(vec![v]),
                _ => None,
            });
        } else {
            Self::walk(vec, tmp, &|v| match v {
                DocValueType::Object(map) => match map.get(key) {
                    Some(v) => Some(vec![v]),
                    _ => None,
                },
                _ => None,
            });
        }
    }

    pub fn all<T: DocValue>(vec: &[&'a DocValueType<T>], tmp: &mut Vec<&'a DocValueType<T>>) {
        Self::walk(vec, tmp, &|v| match v {
            DocValueType::Array(vec) => {
                Some(vec.iter().collect())
            },
            DocValueType::Object(map) => {
                let mut tmp = Vec::new();
                for (_, v) in map {
                    tmp.push(&v);
                }
                Some(tmp)
            }
            _ => None,
        });
    }

    fn walk<F, T: DocValue>(vec: &[&'a DocValueType<T>], tmp: &mut Vec<&'a DocValueType<T>>, fun: &F) where F: Fn(&DocValueType<T>) -> Option<Vec<&DocValueType<T>>> {
        for v in vec {
            Self::_walk::<F,T>(v, tmp, fun);
        }
    }

    fn _walk<F, T: DocValue>(v: &'a DocValueType<T>, tmp: &mut Vec<&'a DocValueType<T>>, fun: &F) where F: Fn(&DocValueType<T>) -> Option<Vec<&DocValueType<T>>> {
        if let Some(mut ret) = fun(v) {
            tmp.append(&mut ret);
        }

        match v {
            DocValueType::Array(vec) => {
                for v in vec {
                    Self::_walk(&v, tmp, fun);
                }
            }
            DocValueType::Object(map) => {
                for (_, v) in map {
                    Self::_walk(&v, tmp, fun);
                }
            }
            _ => {}
        }
    }

    pub fn walk_dedup<T: DocValue>(v: &'a DocValueType<T>,
                      tmp: &mut Vec<&'a DocValueType<T>>,
                      key: &str,
                      visited: &mut HashSet<*const DocValueType<T>>, ) {
        match v {
            DocValueType::Object(map) => {
                if map.contains_key(key) {
                    let ptr = v as *const DocValueType<T>;
                    if !visited.contains(&ptr) {
                        visited.insert(ptr);
                        tmp.push(v)
                    }
                }
            }
            DocValueType::Array(vec) => {
                for v in vec {
                    Self::walk_dedup(&v, tmp, key, visited);
                }
            }
            _ => {}
        }
    }
}

