use sqlx::PgPool;

pub struct Tag {
    pub name: String,
}

pub struct TagRepository {
    db: PgPool,
}

impl TagRepository {
    pub async fn tag_exists(name: &str, db: &PgPool) -> Result<bool, anyhow::Error> {
        sqlx::query_scalar!(
            "SELECT EXISTS(SELECT 1 FROM e621_tag WHERE name = $1)",
            name
        )
        .fetch_one(db)
        .await
        .map(|exists| exists.unwrap_or(false))
        .map_err(|e| anyhow::anyhow!("Failed to check if tag exists: {}", e))
    }

    pub async fn is_forbidden(name: &str, db: &PgPool) -> Result<bool, anyhow::Error> {
        sqlx::query_scalar!(
            "SELECT
                EXISTS(
                    SELECT 1
                    FROM e621_forbidden_tag
                    WHERE e621_tag_id = (
                        SELECT id FROM e621_tag WHERE name = $1
                    )
                )",
            name
        )
        .fetch_one(db)
        .await
        .map(|exists| exists.unwrap_or(false))
        .map_err(|e| anyhow::anyhow!("Failed to check if tag is forbidden: {}", e))
    }
    pub async fn is_default(name: &str, db: &PgPool) -> Result<bool, anyhow::Error> {
        sqlx::query_scalar!(
            "SELECT EXISTS(SELECT 1 FROM e621_default_tag WHERE e621_tag_id = (
                SELECT id FROM e621_tag WHERE name = $1
            ))",
            name
        )
        .fetch_one(db)
        .await
        .map(|exists| exists.unwrap_or(false))
        .map_err(|e| anyhow::anyhow!("Failed to check if tag is default: {}", e))
    }

    pub async fn create_tag(name: &str, db: &PgPool) -> Result<(), anyhow::Error> {
        sqlx::query!("INSERT INTO e621_tag (name) VALUES ($1)", name)
            .execute(db)
            .await?;

        Ok(())
    }

    pub async fn create_forbidden_tag(name: &str, db: &PgPool) -> Result<(), anyhow::Error> {
        sqlx::query!(
            "INSERT INTO e621_forbidden_tag (e621_tag_id) VALUES ((SELECT id FROM e621_tag WHERE name = $1))",
            name
        )
        .execute(db)
        .await?;

        Ok(())
    }

    pub async fn create_default_tag(name: &str, db: &PgPool) -> Result<(), anyhow::Error> {
        sqlx::query!(
            "INSERT INTO e621_default_tag (e621_tag_id) VALUES ((SELECT id FROM e621_tag WHERE name = $1))",
            name
        )
        .execute(db)
        .await?;

        Ok(())
    }

    pub async fn get_forbidden_tags(db: &PgPool) -> Result<Vec<String>, anyhow::Error> {
        sqlx::query!(
            "SELECT e621_tag.name FROM e621_forbidden_tag JOIN e621_tag ON e621_forbidden_tag.e621_tag_id = e621_tag.id"
        )
        .fetch_all(db)
        .await
        .map(|tags| tags.into_iter().map(|tag| tag.name).collect::<Vec<String>>())
        .map_err(|e| anyhow::anyhow!("Failed to get forbidden tags: {}", e))
    }

    pub async fn get_forbidden_tags_from_collection(
        tags: Vec<String>,
        db: &PgPool,
    ) -> Result<Vec<String>, anyhow::Error> {
        let matching_tags = sqlx::query!(
            "SELECT e621_tag.name FROM e621_forbidden_tag JOIN e621_tag ON e621_forbidden_tag.e621_tag_id = e621_tag.id WHERE e621_tag.name IN ($1)",
            tags.iter().map(|tag| tag.as_str()).collect::<Vec<&str>>().join(",")
        )
        .fetch_all(db)
        .await?
        .into_iter()
        .map(|tag| tag.name)
        .collect::<Vec<String>>();

        Ok(matching_tags)
    }
}
