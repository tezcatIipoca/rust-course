#[cfg(test)]
pub mod tests {
    use basic_course::rust_2_5_flow_control::flow_control;

    #[test]
    pub fn test_flow_control() {
        flow_control::if_();
        flow_control::else_if();
        flow_control::for_();
        flow_control::continue_();
        flow_control::break_();
        flow_control::while_();
        flow_control::loop_();
    }
}