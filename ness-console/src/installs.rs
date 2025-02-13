use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::future::Future;
use uuid::Uuid;

use crate::context::Context;

#[derive(Deserialize, JsonSchema, Serialize)]
pub struct Install {
    pub id: Uuid,
}

pub trait Installs {
    fn install_list(&self) -> impl Future<Output = Vec<Install>> + Send;
}

impl Installs for Context {
    async fn install_list(&self) -> Vec<Install> {
        todo!()
    }
}
