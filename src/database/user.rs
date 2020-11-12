#[derive(Serialize, Deserialize)]
pub(crate) struct User {
    pub(crate) color: String,
    pub(crate) name: String,
    pub(crate) _id: String,
}