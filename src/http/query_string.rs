use std::collections::HashMap;

#[derive(Debug)]
pub struct QueryString<'buf> {
    data: HashMap<&'buf str, Value<'buf>>,
}

#[derive(Debug, PartialEq)]
pub enum Value<'buf> {
    Single(&'buf str),
    Multiple(Vec<&'buf str>),
}

impl<'buf> QueryString<'buf> {
    pub fn get(&self, key: &str) -> Option<&Value> {
        self.data.get(key)
    }
}

// a=1&b=2&c&d=&e===&d=7&d=abc
impl<'buf> From<&'buf str> for QueryString<'buf> {
    fn from(s: &'buf str) -> Self {
        let mut data = HashMap::new();

        for sub_str in s.split('&') {
            let mut key = sub_str;
            let mut val = "";
            if let Some(i) = sub_str.find('=') {
                key = &sub_str[..i];
                val = &sub_str[i + 1..];
            }

            data.entry(key)
                .and_modify(|existing: &mut Value| match existing {
                    Value::Single(prev_val) => {
                        *existing = Value::Multiple(vec![prev_val, val]);
                    }
                    Value::Multiple(vec) => vec.push(val),
                })
                .or_insert(Value::Single(val));
        }

        QueryString { data }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn from_single() {
        let path = "a=1&b=2&c&d=&e===&d=7&d=abc";
        let query_string = Some(QueryString::from(path));
        let map = query_string.unwrap().data;
        assert_eq!(map.get(&"a").unwrap(), &Value::Single("1"));
        assert_eq!(map.get(&"c").unwrap(), &Value::Single(""));
        assert_eq!(map.get(&"b").unwrap(), &Value::Single("2"));
        assert_eq!(map.get(&"e").unwrap(), &Value::Single("=="));

    }

    #[test]
    fn from_multiple() {
        let path = "a=1&b=2&c&d=&e===&d=7&d=abc";
        let query_string = Some(QueryString::from(path));
        let map = query_string.unwrap().data;
        let vector = vec!["", "7", "abc"];
        assert_eq!(map.get(&"d").unwrap(), &Value::Multiple(vector));
    }
}
