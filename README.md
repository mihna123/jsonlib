# jsonlib

__Jsonlib__ is a json _parsing_ tool written in __Rust!__ 🦀  

### Usage
The parser returns a `Result` :
```rs 
Result<Value, Box<dyn Error>>
```


#### Value type

A strong recursive type that represents any possible json value

```rs
pub enum Value {
  Object(HashMap<String, Value>),
  Array(Vec<Value>),
  String(String),
  Number(f64),
  Bool(bool),  
  Null,
}

```

To use the value extract it using match case:  
```rs
let res = jsonlib::parse(some_data).unwrap();
if let Value::Number(num) = &res {
  //Do stuff with the num value
}
```  
Or you can extract the value with a method for each type:
```rs
let num = jsonlib::parse(some_data).unwrap().get_num().unwrap();
//Do stuff with num
```
Available methods for Value are:
```rs
pub fn get_num(self) -> Result<f64, Box<dyn Error>>
pub fn get_obj(self) -> Result<HashMap<String, Value>, Box<dyn Error>>
pub fn get_arr(self) -> Result<Vec<Value>, Box<dyn Error>>
pub fn get_str(self) -> Result<String, Box<dyn Error>>
pub fn get_bool(self) -> Result<bool, Box<dyn Error>>
pub fn is_null(&self) -> Result<bool, Box<dyn Error>>
```

To use the parser include the library and the _Value_ type into scope and you are good to go :  
```rs
use jsonlib;
use jsonlib::value::Value;

let json_data = "{\"parse\":\"me\"}";
let result = jsonlib::parse(json_data).expect("Should parse no problem");

if let Value::Object(obj) = &result {
    assert_eq!(obj["parse"], Value::String("me".to_string()));
}

```



### Current development

As of right now there is a working parser, it is a work in progress and needs to be tested more... 

