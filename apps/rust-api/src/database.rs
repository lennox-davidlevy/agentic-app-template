// use anyhow::Result;
// use sqlx::SqlitePool;
//
// pub async fn initialize_database() -> Result<SqlitePool> {
//     let pool = SqlitePool::connect("sqlite:data/app.db").await?;
//
//     create_users_table(&pool).await?;
//
//     println!("Database initialized successfully");
//     Ok(pool)
// }
//
// async fn create_users_table(pool: &SqlitePool) -> Result<()> {
//     let query = r#"
//     CREATE TABLE IF NOT EXISTS users (
//         id INTEGER PRIMARY KEY AUTOINCREMENT,
//         name TEXT NOT NULL,
//         email TEXT NOT NULL UNIQUE
//     )
//     "#;
//
//     sqlx::query(query).execute(pool).await?;
//     println!("Users table created or verified");
//
//     Ok(())
// }
//
// pub async fn test_database_connection(pool: &SqlitePool) -> Result<()> {
//     let row: (i64,) = sqlx::query_as("SELECT 1 as test").fetch_one(pool).await?;
//
//     println!(
//         "Database connection test successful. Test query result: {}",
//         row.0
//     );
//     Ok(())
// }
