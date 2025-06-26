use sea_orm_migration::{prelude::*};

use super::{m20220101_000001_create_user_table::User , m20250623_204658_create_author_table::Author};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
       

        manager
            .create_table(
                Table::create()
                    .table(Book::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Book::Id)
                                .integer()
                                .not_null()
                                .auto_increment()
                                .primary_key(),
                            )
                    .col(ColumnDef::new(Book::UserId).integer().not_null())
                    .foreign_key(ForeignKey::create()
                                .name("fk_Book_User_Id")
                                .from(Book::Table,Book::UserId)
                                .to(User::Table,User::Id)
                            )
                    .col(ColumnDef::new(Book::AuthorId).integer().not_null())
                    .foreign_key(ForeignKey::create()
                                .name("fk_Book_Autho_Id")
                                .from(Book::Table,Book::UserId)
                                .to(Author::Table,Author::Id)
                            )
                    .col(ColumnDef::new(Book::Title).string().not_null())
                    .col(ColumnDef::new(Book::Cover).string().not_null())
                    .col(ColumnDef::new(Book::Year).integer().not_null())
                    .col(ColumnDef::new(Book::CreatedAt)
                                                        .timestamp()
                                                        .extra("DEFAULT CURRENT_TIMESTAMP".to_owned()))
                    .col(ColumnDef::new(Book::UpdatedAt)
                                                        .timestamp()
                                                        .extra("DEFAULT CURRENT_TIMESTAMP".to_owned()))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {


        manager
            .drop_table(Table::drop().table(Book::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Book {
    Table,
    Id,
    UserId,
    AuthorId,
    Title,
    Year,
    Cover,
    CreatedAt,
    UpdatedAt,
}
