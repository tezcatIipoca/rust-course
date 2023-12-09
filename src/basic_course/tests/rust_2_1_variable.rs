#[cfg(test)]
pub mod tests {
    use basic_course::rust_2_1_variable::variable;

    #[test]
    fn test_variable() {
        variable::variable_variability();
        variable::ignored_unused_variable();
        variable::variable_deconstruction();
        variable::destructuring_assignments();
        variable::variable_shadowing();
    }

    #[test]
    fn test_constants() {

    }
}


