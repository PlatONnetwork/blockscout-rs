//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.15

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "contract_url")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub chain_id: String,
    #[sea_orm(primary_key, auto_increment = false)]
    pub address: String,
    pub url: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
