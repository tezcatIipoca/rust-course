#[cfg(test)]
pub mod tests {
    use basic_course::rust_2_2_base_type::base_type;
    use basic_course::rust_2_2_base_type::char_bool;
    use basic_course::rust_2_2_base_type::numbers;
    use basic_course::rust_2_2_base_type::statement_expression;
    use basic_course::rust_2_2_base_type::function;

    #[test]
    fn test_base_type() {
        base_type::base_type();
        base_type::type_guess_mark();
    }

    #[test]
    fn test_numbers() {
        numbers::int_type();
        numbers::float_type();
        numbers::number_crunching();
        numbers::bit_arithmetic();
        numbers::range();
        numbers::rational_complex();
        numbers::summary();
    }

    #[test]
    fn test_char_bool() {
        char_bool::char();
        char_bool::bool();
        char_bool::unit();
    }

    #[test]
    fn statement_expression() {
        statement_expression::statement();
        statement_expression::expression();
    }

    #[test]
    fn function() {
        function::function_gist();
        function::function_param();
        function::function_return();
        function::function_non_return();
        function::function_diverge();
    }
}


