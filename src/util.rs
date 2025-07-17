use crate::token_type::LiteralValue;

pub struct Utils {}

impl Utils {
    pub fn print_literal(literal: &LiteralValue) -> String {
        match literal {
            LiteralValue::Number(num) => {
                format!("{}", num)
            }
            val => val.to_string(),
        }
    }

    pub fn get_char_range(source: &str, start_char: usize, end_char: usize) -> String {
        let mut start_byte = 0;
        let mut end_byte = source.len();

        for (i, (byte_idx, _)) in source.char_indices().enumerate() {
            if i == start_char {
                start_byte = byte_idx;
            }
            if i == end_char {
                end_byte = byte_idx;
                break;
            }
        }

        source[start_byte..end_byte].to_owned()
    }

}