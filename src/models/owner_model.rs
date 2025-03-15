use std::time::SystemTime;
use mongodb::bson::{oid::ObjectId, DateTime};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Owner {
    pub _id: ObjectId,
    pub name: String,
    pub email: String,
    pub phone: String,
    pub address: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct OwnerRequest {
    pub name: String,
    pub email: String,
    pub phone: String,
    pub address: String,
}

impl TryFrom<OwnerRequest> for Owner {
    type Error = Box<dyn std::error::Error>;

    fn try_from(item: OwnerRequest) -> Result<Self, Self::Error> {
        Ok(Self {
            _id: ObjectId::new(),
            name: item.name,
            email: item.email,
            phone: item.phone,
            address: item.address,
        })
    }
}
