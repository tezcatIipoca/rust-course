#[cfg(test)]
pub mod tests {
    use basic_course::rust_2_6_match_pattern::{all_patterns, match_if_let};
    use basic_course::rust_2_6_match_pattern::option;
    use basic_course::rust_2_6_match_pattern::pattern_match;
    // use basic_course::rust_2_6_match_pattern::pattern_match;
    // use basic_course::rust_2_6_match_pattern::all_patterns;

    #[test]
    fn test_match_if_let() {
        match_if_let::match_if_let();
        match_if_let::match_matching();
        match_if_let::if_let();
        match_if_let::matches_macro();
        match_if_let::variable_shadowing();
    }

    #[test]
    fn test_option() {
        option::structure_option();
        option::match_option();
    }

    #[test]
    fn test_pattern_match() {
        pattern_match::match_use();
    }

    #[test]
    fn test_all_patterns() {
        all_patterns::match_literal();
        all_patterns::match_named_variable();
        all_patterns::single_branch_multi_mode();
        all_patterns::match_range();
        all_patterns::structure_value();
        all_patterns::ignore_mode_value();
        all_patterns::match_guard();
        all_patterns::at_bind();
    }

    #[test]
    fn t1(){
        println!("{}", 0.1+0.2);

    }
}