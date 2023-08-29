use std::collections::HashMap;
use std::convert::From;

#[derive(Debug)]
pub struct QueryString<'buf> {
    data: HashMap<&'buf str, Value<'buf>>,
}

#[derive(Debug)]
pub enum Value<'buf> {
    SingleValue(&'buf str),
    MultipleValue(Vec<&'buf str>),
}

impl<'buf> QueryString<'buf> {
    pub fn get(&self, key: &str) -> Option<&Value> {
        self.data.get(key)
    }
}

impl<'buf> From<&'buf str> for QueryString<'buf> {
    fn from(s: &'buf str) -> Self {
        let mut data: HashMap<&str, Value<'buf>> = HashMap::new();

        for sub in s.split("&") {
            let mut key = sub;
            let mut value = "";

            if let Some(i) = sub.find("=") {
                key = &sub[..i];
                value = &sub[i+1..];
            }

            data.entry(key)
            .and_modify(|exisiting: &mut Value| 
                match exisiting {
                    Value::SingleValue(prev) => { 
                        *exisiting = Value::MultipleValue(vec![prev, value]);
                    },
                    Value::MultipleValue(vec) => vec.push(value),
                }
            )
            .or_insert(Value::SingleValue(value));
        }

        QueryString {data}

    }
}

