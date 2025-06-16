use sqlx::{query, query_as, Error};

use crate::repository::postgres::PostgresRepository;
use crate::models::item::{ Item, ItemPayload };

impl PostgresRepository {
    pub async fn create_item(&self, input: ItemPayload) -> Result<Item, Error> {
        let item = query_as::<_, Item>(
            r#"
                INSERT INTO items (name, description)
                VALUES ($1, $2)
                RETURNING id, name, description, created_at, updated_at
            "#
        )
        .bind(input.name).bind(input.description)
        .fetch_one(&self.pool)
        .await?;

        Ok(item)
    }

    pub async fn get_all_items(&self) -> Result<Vec<Item>, Error> {
        let items = query_as::<_, Item>(
            r#"
                SELECT id, name, description, created_at, updated_at
                FROM items
                ORDER BY created_at DESC
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(items)
    }

    pub async fn get_item_by_id(&self, id: i32) -> Result<Option<Item>, Error> {
        let item = query_as::<_, Item>(
            r#"
                SELECT id, name, description, created_at, updated_at
                FROM items
                WHERE id = $1
            "#
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(item)
    }

    pub async fn update_item_by_id(&self, id: i32, input: &ItemPayload) -> Result<Option<Item>, Error> {
        let item = query_as::<_, Item>(
            r#"
                UPDATE items
                SET name = $1, description = $2, updated_at = NOW()
                WHERE id = $3
                RETURNING id, name, description, created_at, updated_at
            "#
        )
        .bind(&input.name)
        .bind(&input.description)
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(item)
    }

    pub async fn delete_item_by_id(&self, id: i32) -> Result<bool, Error> {
        let res = query(
            r#"
                DELETE FROM items
                WHERE id = $1
            "#
        )
        .bind(id)
        .execute(&self.pool)
        .await?;
        
        Ok(res.rows_affected() > 0)
    }
}
