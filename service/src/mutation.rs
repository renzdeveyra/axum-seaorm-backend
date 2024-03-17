use ::entity::{
  bakery, bakery::Entity as Bakery,
  chef, chef::Entity as Chef
};

use sea_orm::*;

pub struct Mutation;

impl Mutation {
  pub async fn create_bakery(
    db: &DbConn,
    bakery_info: bakery::Model,
  ) -> Result<bakery::ActiveModel, DbErr> {
    bakery::ActiveModel {
      name: Set(bakery_info.name.to_owned()),
      profit_margin: Set(bakery_info.profit_margin),
      ..Default::default()
    }
    .save(db)
    .await
  }

  pub async fn update_bakery_by_id(
    db: &DbConn,
    id: i32,
    bakery_info: bakery::Model,
  ) -> Result<bakery::Model, DbErr> {
    let bakery: bakery::ActiveModel = Bakery::find_by_id(id)
      .one(db)
      .await?
      .ok_or(DbErr::Custom("Bakery doesn't exist.".to_owned()))
      .map(Into::into)?;

    bakery::ActiveModel {
      id: bakery.id,
      name: Set(bakery_info.name.to_owned()),
      profit_margin: Set(bakery_info.profit_margin)
    }
    .update(db)
    .await
  }

  pub async fn delete_bakery(db: &DbConn, id: i32) -> Result<DeleteResult, DbErr> {
    let bakery: bakery::ActiveModel = Bakery::find_by_id(id)
        .one(db)
        .await?
        .ok_or(DbErr::Custom("Cannot find bakery.".to_owned()))
        .map(Into::into)?;

    bakery.delete(db).await
  }

  pub async fn delete_all_bakeries(db: &DbConn) -> Result<DeleteResult, DbErr> {
    Bakery::delete_many().exec(db).await
  }

  pub async fn create_chef(
    db: &DbConn,
    chef_info: chef::Model,
  ) -> Result<chef::ActiveModel, DbErr> {
    chef::ActiveModel {
      name: Set(chef_info.name.to_owned()),
      contact_details: Set(Some(chef_info.contact_details.into())),
      bakery_id: Set(chef_info.bakery_id),
      ..Default::default()
    }
    .save(db)
    .await
  }

  pub async fn update_chef_by_id(
    db: &DbConn,
    id: i32,
    chef_info: chef::Model,
  ) -> Result<chef::Model, DbErr> {
    let chef: chef::ActiveModel = Chef::find_by_id(id)
      .one(db)
      .await?
      .ok_or(DbErr::Custom("Your Chef isn't here".to_owned()))
      .map(Into::into)?;

    chef::ActiveModel {
      id: chef.id,
      name: Set(chef_info.name.to_owned()),
      contact_details: Set(Some(chef_info.contact_details.into())),
      bakery_id: Set(chef_info.bakery_id),
      ..Default::default()
    }
    .update(db)
    .await
  }

  pub async fn delete_chef(db: &DbConn, id: i32) -> Result<DeleteResult, DbErr> {
    let chef: chef::ActiveModel = Chef::find_by_id(id)
        .one(db)
        .await?
        .ok_or(DbErr::Custom("Cannot find bakery.".to_owned()))
        .map(Into::into)?;

    chef.delete(db).await
  }

  pub async fn delete_all_chefs(db: &DbConn) -> Result<DeleteResult, DbErr> {
    Bakery::delete_many().exec(db).await
  }
}
