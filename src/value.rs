use std::clone::Clone;
use std::collections::HashMap;
use std::error::Error;

#[derive(PartialEq, Debug)]
pub enum Value {
    Object(HashMap<String, Value>),
    Array(Vec<Value>),
    String(String),
    Number(f64),
    Bool(bool),
    Null,
}

impl Clone for Value {
    fn clone(&self) -> Self {
        match self {
            Value::Object(obj) => Value::Object(obj.clone()),
            Value::Array(arr) => Value::Array(arr.clone()),
            Value::Number(num) => Value::Number(num.clone()),
            Value::String(string) => Value::String(string.clone()),
            Value::Null => Value::Null,
            Value::Bool(boo) => Value::Bool(boo.clone()),
        }
    }
}

impl Value {
    pub fn get_obj(self) -> Result<HashMap<String, Value>, Box<dyn Error>> {
        if let Value::Object(obj) = self {
            return Ok(obj);
        }
        Err("The value you are trying to extract is not an object!")?
    }

    pub fn get_arr(self) -> Result<Vec<Value>, Box<dyn Error>> {
        if let Value::Array(vec) = self {
            return Ok(vec);
        }
        Err("The value you are trying to extract is not an Array!")?
    }

    pub fn get_str(self) -> Result<String, Box<dyn Error>> {
        if let Value::String(str) = self {
            return Ok(str);
        }
        Err("The value you are trying to extract is not a String!")?
    }

    pub fn get_num(self) -> Result<f64, Box<dyn Error>> {
        if let Value::Number(num) = self {
            return Ok(num);
        }
        Err("The value you are trying to extract is not a Number!")?
    }

    pub fn get_bool(self) -> Result<bool, Box<dyn Error>> {
        if let Value::Bool(boo) = self {
            return Ok(boo);
        }
        Err("The value you are trying to extract is not a Bool!")?
    }

    pub fn is_null(&self) -> bool {
        if let Value::Null = self {
            return true;
        }
        false
    }
}
