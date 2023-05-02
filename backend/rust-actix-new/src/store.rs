use serde::{Deserialize, Serialize};
use sqlx::postgres::PgPool;
use uuid::Uuid;

#[derive(Debug, Serialize)]
pub struct Workspace {
    #[serde(skip_serializing)]
    pub id: i32,
    pub identifier: Uuid,
}

impl Workspace {
    pub async fn find(pool: &PgPool, identifier: &Uuid) -> Option<Self> {
        sqlx::query_as!(
            Self,
            "
            select id, identifier
            from workspace
            where identifier = $1
            limit 1
            ",
            &identifier
        )
            .fetch_optional(pool)
            .await
            .unwrap()
    }
}

// Unit-less only because, for now, there are no params like 'title'
// but there likely will be, so follow a similar pattern to the others.
#[derive(Debug)]
pub struct NewWorkspace;

impl NewWorkspace {
    pub async fn create(&self, pool: &PgPool) -> Workspace {
        let identifier = Uuid::new_v4();

        sqlx::query_as!(
            Workspace,
            "
            insert into workspace (identifier)
            values ($1)
            returning id, identifier
            ",
            &identifier
        )
            .fetch_one(pool)
            .await
            .unwrap()
    }
}

#[derive(Debug, Serialize)]
pub struct Board {
    #[serde(skip_serializing)]
    pub id: i32,
    pub identifier: Uuid,
    #[serde(rename = "workspace")]
    pub workspace_identifier: Uuid,
    pub title: String,
}

impl Board {
    pub async fn find_all(pool: &PgPool, workspace_identifier: &Uuid) -> Vec<Self> {
        sqlx::query_as!(
            Self,
            "
            select
                b.id,
                b.identifier,
                b.title,
                w.identifier as workspace_identifier

            from board b
            join workspace w on w.id = b.workspace_id

            where w.identifier = $1
            ",
            &workspace_identifier
        )
            .fetch_all(pool)
            .await
            .unwrap()
    }

    pub async fn delete(pool: &PgPool, board_identifier: &Uuid) -> bool {
        let rows_affected = sqlx::query!(
            "
            delete from board
            where identifier = $1
            ",
            &board_identifier
        )
            .execute(pool)
            .await
            .unwrap()
            .rows_affected();

        rows_affected > 0
    }
}

#[derive(Debug, Deserialize)]
pub struct NewBoard {
    #[serde(rename = "workspace")]
    pub workspace_identifier: Uuid,
    pub title: String,
}

impl NewBoard {
    pub async fn create(&self, pool: &PgPool) -> Board {
        let identifier = Uuid::new_v4();

        sqlx::query_as!(
            Board,
            r#"
            insert into board (
                identifier,
                workspace_id,
                title
            )
            values (
                $1,
                (select id from workspace where identifier = $2),
                $3
            )
            returning id, identifier, title, $2 as "workspace_identifier!"
            "#,
            &identifier,
            self.workspace_identifier,
            &self.title,
        )
            .fetch_one(pool)
            .await
            .unwrap()
    }
}

#[derive(Debug, Serialize)]
pub struct Card {
    #[serde(skip_serializing)]
    pub id: i32,
    pub identifier: Uuid,
    #[serde(rename = "board")]
    pub board_identifier: Uuid,
    pub title: String,
    pub body: Option<String>,
}

impl Card {
    pub async fn find_all(pool: &PgPool, workspace_identifier: &Uuid) -> Vec<Self> {
        sqlx::query_as!(
            Self,
            "
            select
                c.id,
                c.identifier,
                c.title,
                c.body,
                b.identifier as board_identifier

            from card c
            join board b on b.id = c.board_id
            join workspace w on w.id = b.workspace_id

            where w.identifier = $1
            ",
            &workspace_identifier
        )
            .fetch_all(pool)
            .await
            .unwrap()
    }
}

#[derive(Debug, Deserialize)]
pub struct NewCard {
    #[serde(rename = "board")]
    pub board_identifier: Uuid,
    pub title: String,
}

impl NewCard {
    pub async fn create(&self, pool: &PgPool) -> Card {
        let identifier = Uuid::new_v4();

        sqlx::query_as!(
            Card,
            r#"
            insert into card (
                identifier,
                board_id,
                title
            )
            values (
                $1,
                (select id from board where identifier = $2),
                $3
            )
            returning id, identifier, title, body, $2 as "board_identifier!"
            "#,
            &identifier,
            self.board_identifier,
            &self.title,
        )
            .fetch_one(pool)
            .await
            .unwrap()
    }
}