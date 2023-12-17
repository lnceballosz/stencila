//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.6

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "trials")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub current_timestamp: DateTimeUtc,
    pub user_instruction: String,
    pub agent_response: String,
    pub generate_detail: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
