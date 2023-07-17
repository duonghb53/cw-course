pub mod query {
    use crate::msg::ValueResponse;
    pub fn value(value: u64) -> ValueResponse {
        ValueResponse { value }
    }
}
