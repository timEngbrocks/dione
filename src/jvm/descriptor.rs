use super::types::{Types, byte::Byte, Value, char::Char, double::Double, float::Float, int::Int, long::Long, short::Short, boolean::Boolean, reference::Reference, ReturnTypes};

pub fn parse_method_descriptor(descriptor: &str) -> Option<(Vec<Types>, ReturnTypes)> {
	let mut args = Vec::new();
	let mut i = 0;
	let mut parsed_args = false;
	while i < descriptor.len() {
		let char = descriptor.chars().nth(i).unwrap();
		if i == 0 && char != '(' {
			panic!("Invalid method descriptor: {}", descriptor);
		}
		if char == ')' {
			parsed_args = true;
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
	match descriptor.chars().nth(0).unwrap() {
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
		},
		'[' => {
			// FIXME: array of type
			let mut j = 1;
			while let Some(c) = descriptor.chars().nth(j) {
				match c {
					'B' |
					'C' |
					'D' |
					'F' |
					'I' |
					'J' |
					'S' |
					'Z' |
					';' => break,
					_ => (),
				}
				j += 1;
			}
			(ReturnTypes::Type(Types::Reference(Reference::new())), j + 1)
		},
		'V' if is_return_type => (ReturnTypes::Void, 1),
		_ => panic!("Invalid method descriptor type: {}", descriptor),
	}
}