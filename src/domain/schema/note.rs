use serde::{Deserialize, Serialize};


#[derive(Deserialize, Debug, Default)]
pub struct FilterNote{
    pub page: Option<usize>,
    pub limit: Option<usize>,
}

#[derive(Deserialize, Debug)]
pub struct ParamNote {
    pub id: String
}

#[derive(Serialize,Deserialize, Debug)]
pub struct CreateNote {
    pub title: String,
    pub content: String,
    pub is_published: bool
}

#[derive(Serialize,Deserialize, Debug)]
pub struct EditNote {
    pub title: Option<String>,
    pub content: Option<String>,
    pub is_published: Option<bool>
}