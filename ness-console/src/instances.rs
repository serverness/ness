use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::future::Future;
use uuid::Uuid;

use crate::context::Context;

#[derive(Deserialize, JsonSchema, Serialize)]
pub struct Instance {
    pub id: Uuid,
}

pub trait Instances {
    fn instance_list(&self) -> impl Future<Output = Vec<Instance>> + Send;
}

impl Instances for Context {
    async fn instance_list(&self) -> Vec<Instance> {
        todo!()
    }
}
