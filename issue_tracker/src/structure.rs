use {
    serde::{Deserialize, Serialize}
};


#[derive(Debug, Deserialize, Serialize)]
pub struct  Storage {
    pub issue_id: String,
    pub created_at: String,
    pub updated_at: String,
    pub status: String,
    pub committer_id: String,
    pub resolver_id: String,
    pub body: String
}