use crate::Vector;

#[derive(Clone, Debug)]
pub struct Flag {
    pub name: String,
    pub usage: String,
    pub short_alias: Vector<String>,
    pub long_alias: Vector<String>,
    pub default_value: FlagValue,
    pub flag_type: FlagType,
}

#[derive(PartialOrd, PartialEq, Clone, Debug)]
pub enum FlagType {
    Bool,
    String,
    Int,
    Float,
}

impl FlagType {
    pub fn name<'a>(&self) -> &'a str {
        match self {
            FlagType::Bool => "Bool",
            FlagType::String => "String",
            FlagType::Int => "Int",
            FlagType::Float => "Float",
            //_ => "Unknown",
        }
    }
    pub fn type_default(&self) -> FlagValue {
        match self {
            FlagType::Bool => FlagValue::Bool(bool::default()),
            FlagType::String => FlagValue::String(String::default()),
            FlagType::Int => FlagValue::Int(isize::default()),
            FlagType::Float => FlagValue::Float(f64::default()),
        }
    }
    pub fn is_type_of(&self, val: &FlagValue) -> bool {
        Some(self) == val.get_type()
    }

    pub fn getValueFromString(&self, val: &str) -> FlagValue {
        match self {
            FlagType::Bool => match val {
                "true" => FlagValue::Bool(true),
                "false" => FlagValue::Bool(false),
                _ => FlagValue::None,
            },
            FlagType::String => FlagValue::String(String::from(val)),
            FlagType::Int => match val.parse::<isize>() {
                Ok(i) => FlagValue::Int(i),
                Err(_) => FlagValue::None,
            },
            FlagType::Float => match val.parse::<f64>() {
                Ok(f) => FlagValue::Float(f),
                Err(_) => FlagValue::None,
            },
        }
    }
}

impl Default for FlagType {
    fn default() -> Self {
        FlagType::String
    }
}

#[derive(PartialOrd, PartialEq, Clone, Debug)]
pub enum FlagValue {
    Bool(bool),
    String(String),
    Int(isize),
    Float(f64),
    None,
}

impl Default for FlagValue {
    fn default() -> Self {
        FlagValue::None
    }
}

impl FlagValue {
    pub fn get_type(&self) -> Option<&FlagType> {
        match self {
            FlagValue::Bool(_) => Some(&FlagType::Bool),
            FlagValue::String(_) => Some(&FlagType::String),
            FlagValue::Int(_) => Some(&FlagType::Int),
            FlagValue::Float(_) => Some(&FlagType::Float),
            FlagValue::None => None,
        }
    }
    pub fn is_type(&self, flag_type: &FlagType) -> bool {
        Some(flag_type) == self.get_type()
    }
}

impl Default for Flag {
    fn default() -> Flag {
        Flag {
            name: String::default(),
            usage: String::default(),
            short_alias: Vector::default(),
            long_alias: Vector::default(),
            flag_type: FlagType::default(),
            default_value: FlagValue::default(),
        }
    }
}

impl Flag {
    pub fn new(name: &str, usage: &str, flag_type: FlagType) -> Flag {
        let default_value: FlagValue = match flag_type {
            FlagType::Bool => FlagValue::Bool(bool::default()),
            FlagType::String => FlagValue::String(String::default()),
            FlagType::Int => FlagValue::Int(isize::default()),
            FlagType::Float => FlagValue::Float(f64::default()),
        };
        Flag {
            name: String::from(name),
            usage: String::from(usage),
            long_alias: Vector::default(),
            short_alias: Vector::default(),
            flag_type,
            default_value,
        }
    }

    pub fn build_new(
        name: String,
        usage: String,
        short_alias: Vector<String>,
        long_alias: Vector<String>,
        flag_type: FlagType,
        default_value: FlagValue,
    ) -> Flag {
        let calculated_default_value = if default_value.is_type(&flag_type) {
            default_value
        } else {
            let flag_type_str = flag_type.name();
            eprintln!("FlagType is {},but inputted default_value is not {}. default_value will be {}'s default.",flag_type_str,flag_type_str,flag_type_str);
            flag_type.type_default()
        };
        Flag {
            name,
            usage,
            short_alias,
            long_alias,
            flag_type,
            default_value: calculated_default_value,
        }
    }

    pub fn short<T: Into<String>>(mut self, a: T) -> Self {
        self.short_alias.push(a.into());
        self
    }
    pub fn alias<T: Into<String>>(mut self, a: T) -> Self {
        self.long_alias.push(a.into());
        self
    }

    pub fn default_value(mut self, default_value: FlagValue) -> Self {
        if self.flag_type.is_tyoe_of(&default_value) {
            self.default_value = default_value
        } else {
            println!(
                "not match flag_type: {}. default_value is not changed.",
                self.flag_type.name()
            );
        }
        self
    }

    pub fn usage<T: Into<String>>(mut self, usage: T) -> Self {
        self.usage = usage.into();
        self
    }

    pub fn is(&self, name: &str) -> bool {
        self.name == name
    }

    pub fn is_short(&self, alias: &str) -> bool {
        match &self.short_alias {
            Vector(None) => false,
            Vector(Some(short_alias)) => short_alias.iter().any(|s| s == alias),
        }
    }

    pub fn is_long(&self, alias: &str) -> bool {
        match &self.long_alias {
            Vector(None) => false,
            Vector(Some(long_alias)) => long_alias.iter().any(|s| s == alias),
        }
    }
}
