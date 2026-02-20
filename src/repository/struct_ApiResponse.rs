use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct ApiResponse<T> {
    pub status: u16,
    pub data: T,
    pub message: String,
}
