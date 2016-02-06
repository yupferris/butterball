// TODO: This whole module should really be a part of the IL

#[derive(Debug, Clone)]
pub enum Value {
    Unit,
    Integer(i32),
    Float(f32),
    Bool(bool),
    String(String)
}

impl Value {
    pub fn default(value_type: &ValueType) -> Value {
        match value_type {
            &ValueType::Unit => Value::Unit,
            &ValueType::Integer => Value::Integer(0),
            &ValueType::Float => Value::Float(0.0),
            &ValueType::Bool => Value::Bool(false),
            &ValueType::String => Value::String(String::new())
        }
    }

    pub fn cast_to_unit(&self) -> Value {
        match self {
            &Value::Unit => Value::Unit,
            _ => panic!("Value cannot be cast to a unit: {:?}", self)
        }
    }

    pub fn as_integer(&self) -> i32 {
        match self {
            &Value::Integer(value) => value,
            _ => panic!("Value was not an integer: {:?}", self)
        }
    }

    pub fn cast_to_integer(&self) -> Value {
        Value::Integer(match self {
            &Value::Integer(value) => value,
            &Value::Float(value) => value as i32,
            &Value::Bool(value) => if value { 1 } else { 0 },
            _ => panic!("Value cannot be cast to an integer: {:?}", self)
        })
    }

    pub fn as_float(&self) -> f32 {
        match self {
            &Value::Float(value) => value,
            _ => panic!("Value was not a float: {:?}", self)
        }
    }

    pub fn cast_to_float(&self) -> Value {
        Value::Float(match self {
            &Value::Integer(value) => value as f32,
            &Value::Float(value) => value,
            _ => panic!("Value cannot be cast to a float: {:?}", self)
        })
    }

    pub fn as_bool(&self) -> bool {
        match self {
            &Value::Bool(value) => value,
            _ => panic!("Value was not a bool: {:?}", self)
        }
    }

    pub fn cast_to_bool(&self) -> Value {
        Value::Bool(match self {
            &Value::Bool(value) => value,
            _ => panic!("Value cannot be cast to an integer: {:?}", self)
        })
    }

    pub fn as_string(&self) -> String {
        match self {
            &Value::String(ref value) => value.clone(),
            _ => panic!("Value was not a string: {:?}", self)
        }
    }

    pub fn cast_to_string(&self) -> Value {
        Value::String(match self {
            &Value::String(ref value) => value.clone(),
            _ => panic!("Value cannot be cast to a string: {:?}", self)
        })
    }

    pub fn cast_to(&self, value_type: &ValueType) -> Value {
        match value_type {
            &ValueType::Unit => self.cast_to_unit(),
            &ValueType::Integer => self.cast_to_integer(),
            &ValueType::Float => self.cast_to_float(),
            &ValueType::Bool => self.cast_to_bool(),
            &ValueType::String => self.cast_to_string()
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ValueType {
    Unit,
    Integer,
    Float,
    Bool,
    String
}
