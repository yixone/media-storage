use serde::Serialize;

// TODO: add Storage and DB info
/// DTO for server information response
#[derive(Serialize)]
pub struct ServerInfoResponse {
    /// Server version
    pub version: String,
}
