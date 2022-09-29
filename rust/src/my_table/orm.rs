use sea_orm::entity::prelude::*;

use serde::{Deserialize, Serialize};

use crate::json::Json;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "my_table")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub status: String,
    pub metadata: Json<Metadata>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Metadata {
    pub parent_id: Option<i32>,
}

pub trait Scope: QueryFilter {
    // scope :active, -> { where(status: 'Active') }
    fn active(self) -> Self {
        self.filter(Column::Status.eq("Active"))
    }
}

impl Scope for Select<Entity> {}
