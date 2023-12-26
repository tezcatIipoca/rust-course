#[cfg(test)]
pub mod tests {
    use basic_course::rust_2_4_compound_type::string_slice;
    use basic_course::rust_2_4_compound_type::tuple;
    use basic_course::rust_2_4_compound_type::struct_type;
    use basic_course::rust_2_4_compound_type::enum_type;
    use basic_course::rust_2_4_compound_type::array;

    #[test]
    pub fn test_string_slice() {
        string_slice::compound_type();
        string_slice::string();
        string_slice::slice();
        string_slice::str_literal();
        string_slice::define_str();
        string_slice::convert_str();
        string_slice::str_index();
        string_slice::str_slice();
        string_slice::manipulating_str();
        string_slice::str_escape();
        string_slice::utf8_str();
        string_slice::str_analyze();
    }

    #[test]
    pub fn test_tuple() {
        tuple::tuple();
        tuple::match_tuple();
        tuple::access_tuple();
        tuple::tuple_example();
    }

    #[test]
    pub fn test_struct() {
        struct_type::struct_syntax();
        struct_type::struct_mem();
        struct_type::tuple_struct();
        struct_type::unit_like_struct();
        struct_type::struct_ownership();
        struct_type::struct_print();
    }

    #[test]
    pub fn test_enum() {
        enum_type::enum_value();
        enum_type::uniform_type();
        enum_type::option_enum();
    }

    #[test]
    pub fn test_array() {
        array::create_array();
        array::access_array();
        array::array_slice();
    }
}