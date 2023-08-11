use async_graphql::SimpleObject;
use serde::Serialize;

#[derive(Serialize, SimpleObject)]
#[serde(rename_all = "camelCase")]
pub struct Me<'a> {
    pub email: &'a str,
}
