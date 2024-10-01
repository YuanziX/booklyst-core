use sea_orm_migration::{prelude::*, schema::*};

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
                    .col(pk_auto(Book::Id))
                    .col(string(Book::Title))
                    .col(string(Book::Author))
                    .col(string(Book::Description))
                    .col(unsigned(Book::Price))
                    .col(unsigned(Book::AvailableStock))
                    .col(boolean(Book::IsRentable).default(true))
                    .col(unsigned(Book::RentalPricePerDay))
                    .col(date_time(Book::CreatedAt).default(Expr::current_timestamp()))
                    .col(date_time(Book::UpdatedAt).default(Expr::current_timestamp()))
                    .to_owned(),
            )
            .await?;

        manager.create_index(
            Index::create()
            .table(Book::Table)
            .name("idx_book_author")
            .col(Book::Author)
            .to_owned(),
        )
        .await?;

        manager.create_index(
            Index::create()
            .table(Book::Table)
            .name("idx_book_rentable")
            .col(Book::IsRentable)
            .to_owned(),
        )
        .await?;

    Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_index(Index::drop().name("idx_book_author").to_owned()).await?;
        manager.drop_index(Index::drop().name("idx_book_rentable").to_owned()).await?;

        manager
            .drop_table(Table::drop().table(Book::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum Book {
    Table,
    Id,
    Title,
    Author,
    Description,
    Price,
    AvailableStock,
    IsRentable,
    RentalPricePerDay,
    CreatedAt,
    UpdatedAt,
}
