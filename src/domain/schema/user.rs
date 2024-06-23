use serde::{Deserialize, Serialize};

#[derive(Serialize,Debug, Default)]
pub struct FilterUser {
    pub page : Option<usize>,
    pub limit : Option<usize>,
}


#[derive(Serialize,Debug)]
pub struct UserParam{
    pub id : String,
}

#[derive(Serialize,Deserialize,Debug)]
pub struct CreatUser {
    pub name : String,
    pub email : String,
    pub password : String
}


#[derive(Serialize,Deserialize,Debug)]
pub struct EditUser {
    pub name : Option<String>,
    pub email : Option<String>,
    pub password : Option<String>,
}