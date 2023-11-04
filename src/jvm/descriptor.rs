use super::types::{
    boolean::Boolean, byte::Byte, char::Char, double::Double, float::Float, int::Int, long::Long,
    reference::Reference, short::Short, ReturnTypes, Types, Value,
};

pub fn parse_method_descriptor(descriptor: &str) -> Option<(Vec<Types>, ReturnTypes)> {
    let mut args = Vec::new();
    let mut i = 0;
    let mut parsed_args = false;
    while i < descriptor.len() {
        let char = descriptor.chars().nth(i).unwrap();
        if i == 0 && char != '(' {
            panic!("Invalid method descriptor: {}", descriptor);
        } else if i == 0 && char == '(' {
            i += 1;
            continue;
        }
        if char == ')' {
            parsed_args = true;
            i += 1;
            continue;
        }
        if !parsed_args {
            let (arg, len) = get_next_type_from_descriptor(&descriptor[i..], false);
            match arg {
                ReturnTypes::Type(t) => args.push(t),
                _ => panic!("Invalid method descriptor, expected Type: {}", descriptor),
            }
            i += len;
        } else {
            let (return_type, len) = get_next_type_from_descriptor(&descriptor[i..], true);
            assert_eq!(len + i, descriptor.len());
            return Some((args, return_type));
        }
    }
    None
}

fn get_next_type_from_descriptor(descriptor: &str, is_return_type: bool) -> (ReturnTypes, usize) {
    match descriptor.chars().next().unwrap() {
        'B' => (ReturnTypes::Type(Types::Byte(Byte::new())), 1),
        'C' => (ReturnTypes::Type(Types::Char(Char::new())), 1),
        'D' => (ReturnTypes::Type(Types::Double(Double::new())), 1),
        'F' => (ReturnTypes::Type(Types::Float(Float::new())), 1),
        'I' => (ReturnTypes::Type(Types::Int(Int::new())), 1),
        'J' => (ReturnTypes::Type(Types::Long(Long::new())), 1),
        'S' => (ReturnTypes::Type(Types::Short(Short::new())), 1),
        'Z' => (ReturnTypes::Type(Types::Boolean(Boolean::new())), 1),
        'L' => {
            let mut j = 1;
            while let Some(c) = descriptor.chars().nth(j) {
                if c == ';' {
                    break;
                }
                j += 1;
            }
            (ReturnTypes::Type(Types::Reference(Reference::new())), j + 1)
        }
        '[' => {
            let (return_type, len) =
                get_next_type_from_descriptor(&descriptor[1..], is_return_type);
            (return_type, len + 1)
        }
        'V' if is_return_type => (ReturnTypes::Void, 1),
        _ => panic!("Invalid method descriptor type: {}", descriptor),
    }
}

pub fn parse_field_descriptor(descriptor: &str) -> Option<Types> {
    match descriptor {
        "B" => Some(Types::Byte(Byte::new())),
        "C" => Some(Types::Char(Char::new())),
        "D" => Some(Types::Double(Double::new())),
        "F" => Some(Types::Float(Float::new())),
        "I" => Some(Types::Int(Int::new())),
        "J" => Some(Types::Long(Long::new())),
        "S" => Some(Types::Short(Short::new())),
        "Z" => Some(Types::Boolean(Boolean::new())),
        descriptor if descriptor.starts_with('L') => Some(Types::Reference(Reference::new())),
        descriptor if descriptor.starts_with('[') => Some(Types::Reference(Reference::new())),
        _ => None,
    }
}

pub fn field_descriptor_is_primitive(descriptor: &str) -> bool {
    matches!(descriptor, "B" | "C" | "D" | "F" | "I" | "J" | "S" | "Z")
}

pub fn field_descriptor_is_object(descriptor: &str) -> bool {
    matches!(descriptor, descriptor if descriptor.starts_with('L'))
}

pub fn field_descriptor_is_array(descriptor: &str) -> bool {
    matches!(descriptor, descriptor if descriptor.starts_with('['))
}
