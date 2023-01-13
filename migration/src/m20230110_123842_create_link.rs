use sea_orm_migration::prelude::*;
use super::m20230110_123833_create_user::User;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .create_table(
                Table::create()
                    .table(Link::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Link::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Link::UserId).integer().not_null())
                    .foreign_key(ForeignKey::create()
                        .name("user-id")
                        .from(Link::Table, Link::UserId)
                        .to(User::Table, User::Id),
                    )
                    .col(ColumnDef::new(Link::Url).string().not_null())
                    .col(ColumnDef::new(Link::Slug).string().not_null())
                    .to_owned(),
            )
            .await?;

        // sample seed data
        let insert = Query::insert()
            .into_table(Link::Table)
            .columns([Link::UserId, Link::Url, Link::Slug])
            .values_panic([1.into(), "https://campsite.bio/fivehanz".into(), "bio".into()])
            .values_panic([1.into(), "https://www.linkedin.com/in/fivehanz/".into(), "linkedin".into()])
            .to_owned();

        // insert the data
        manager.exec_stmt(insert).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        // todo!();

        manager
            .drop_table(Table::drop().table(Link::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum Link {
    Table,
    Id,
    UserId,
    Url,
    Slug,
}
