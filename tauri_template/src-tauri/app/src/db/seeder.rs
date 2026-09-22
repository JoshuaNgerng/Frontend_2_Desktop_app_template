mod seed {
    pub const DEFAULT_EXAMPLE: &str = include_str!("../../../seeder/example.sql");
}

pub async fn run_seeders(pool: &sqlx::SqlitePool) -> Result<(), sqlx::Error> {
    let sql_list = vec![seed::DEFAULT_EXAMPLE];
    let mut session = pool.begin().await?;
    for sql in sql_list {
        // println!("debug \n{}", sql);
        sqlx::query(sql).execute(&mut *session).await?;
    }
    session.commit().await?;
    Ok(())
}