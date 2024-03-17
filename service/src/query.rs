use ::entity::{
    bakery, bakery::Entity as Bakery,
    chef, chef::Entity as Chef,
};
use sea_orm::*;

pub struct Query;

impl Query {
    pub async fn find_bakery_by_id(db: &DbConn, id: i32) -> Result<Option<bakery::Model>, DbErr> {
        Bakery::find_by_id(id).one(db).await
    }

    pub async fn find_chef_by_id(db: &DbConn, id: i32) -> Result<Option<chef::Model>, DbErr> {
        Chef::find_by_id(id).one(db).await
    }

    pub async fn find_bakery_in_page(
        db: &DbConn,
        page: u64,
        posts_per_page: u64,
    ) -> Result<(Vec<bakery::Model>, u64), DbErr> {
        // Setup paginator
        let paginator = Bakery::find()
            .order_by_asc(bakery::Column::Id)
            .paginate(db, posts_per_page);
        let num_pages = paginator.num_pages().await?;

        // Fetch paginated posts
        paginator.fetch_page(page - 1).await.map(|p| (p, num_pages))
    }

    pub async fn find_chef_in_page(
        db: &DbConn,
        page: u64,
        posts_per_page: u64,
    ) -> Result<(Vec<chef::Model>, u64), DbErr> {
        // Setup paginator
        let paginator = Chef::find()
            .order_by_asc(chef::Column::Id)
            .paginate(db, posts_per_page);
        let num_pages = paginator.num_pages().await?;

        // Fetch paginated posts
        paginator.fetch_page(page - 1).await.map(|p| (p, num_pages))
    }
}