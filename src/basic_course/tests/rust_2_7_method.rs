
#[cfg(test)]
pub mod tests {
        use basic_course::rust_2_7_method::method;
    #[test]
    fn test_method(){
        method::define_method();
        method::multipart_param_method();
        method::association_function();
        method::multipart_impl();
        method::impl_enum();
    }
}