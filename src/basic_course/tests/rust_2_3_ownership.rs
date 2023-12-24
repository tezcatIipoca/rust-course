#[cfg(test)]
pub mod tests {
    use basic_course::rust_2_3_ownership::ownership;
    use basic_course::rust_2_3_ownership::borrowing;

    #[test]
    pub fn test_ownership()->(){
        ownership::ownership_rule();
        ownership::variable_bind();
        ownership::function_args_return();
    }
    #[test]
    pub fn test_borrowing(){
        borrowing::ref_and_deref();
        borrowing::immut_ref();
        borrowing::ref_and_deref();
    }
}