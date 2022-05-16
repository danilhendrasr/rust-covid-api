#[derive(serde::Deserialize)]
pub struct QueryParams {
    pub since: Option<i32>,
    pub upto: Option<i32>,
}
