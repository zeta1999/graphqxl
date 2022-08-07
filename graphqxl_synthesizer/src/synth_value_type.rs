use crate::synths::Synth;
use graphqxl_parser::{ValueBasicType, ValueType};

pub(crate) struct ValueTypeSynth(pub(crate) ValueType);

impl Synth for ValueTypeSynth {
    fn synth(&self, _indent_lvl: usize, _multiline: bool) -> String {
        match &self.0 {
            ValueType::Simple(simple) => {
                let suffix = if !simple.nullable { "!" } else { "" };
                match &simple.content {
                    ValueBasicType::Int => "Int".to_string() + suffix,
                    ValueBasicType::Float => "Float".to_string() + suffix,
                    ValueBasicType::String => "String".to_string() + suffix,
                    ValueBasicType::Boolean => "Boolean".to_string() + suffix,
                    ValueBasicType::Object(name) => name.clone() + suffix,
                }
            }
            ValueType::Array(array) => {
                let suffix = if !array.nullable { "!" } else { "" };
                format!(
                    "[{}]{}",
                    ValueTypeSynth(ValueType::Simple(array.value.clone())).synth_zero(),
                    suffix
                )
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use graphqxl_parser::{ValueTypeArray, ValueTypeSimple};

    #[test]
    fn test_nullable_int() {
        let synth = ValueTypeSynth(ValueType::Simple(ValueTypeSimple {
            content: ValueBasicType::Int,
            nullable: true,
        }));
        assert_eq!(synth.synth_zero(), "Int");
    }

    #[test]
    fn test_non_nullable_int() {
        let synth = ValueTypeSynth(ValueType::Simple(ValueTypeSimple {
            content: ValueBasicType::Int,
            nullable: false,
        }));
        assert_eq!(synth.synth_zero(), "Int!");
    }

    #[test]
    fn test_array_int() {
        let synth = ValueTypeSynth(ValueType::Array(ValueTypeArray {
            value: ValueTypeSimple {
                content: ValueBasicType::Int,
                nullable: true,
            },
            nullable: true,
        }));
        assert_eq!(synth.synth_zero(), "[Int]");
    }

    #[test]
    fn test_non_nullable_array_nullable_int() {
        let synth = ValueTypeSynth(ValueType::Array(ValueTypeArray {
            value: ValueTypeSimple {
                content: ValueBasicType::Int,
                nullable: true,
            },
            nullable: false,
        }));
        assert_eq!(synth.synth_zero(), "[Int]!");
    }

    #[test]
    fn test_non_nullable_array_non_nullable_int() {
        let synth = ValueTypeSynth(ValueType::Array(ValueTypeArray {
            value: ValueTypeSimple {
                content: ValueBasicType::Int,
                nullable: false,
            },
            nullable: false,
        }));
        assert_eq!(synth.synth_zero(), "[Int!]!");
    }

    #[test]
    fn test_non_nullable_array_non_nullable_string() {
        let synth = ValueTypeSynth(ValueType::Array(ValueTypeArray {
            value: ValueTypeSimple {
                content: ValueBasicType::String,
                nullable: false,
            },
            nullable: false,
        }));
        assert_eq!(synth.synth_zero(), "[String!]!");
    }

    #[test]
    fn test_non_nullable_array_non_nullable_object() {
        let synth = ValueTypeSynth(ValueType::Array(ValueTypeArray {
            value: ValueTypeSimple {
                content: ValueBasicType::Object("MyObject".to_string()),
                nullable: false,
            },
            nullable: false,
        }));
        assert_eq!(synth.synth_zero(), "[MyObject!]!");
    }
}
