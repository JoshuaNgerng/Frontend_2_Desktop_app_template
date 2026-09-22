
use sqlx::{Database, QueryBuilder};

pub trait Repository {
    fn add_filter_ref<'a, T, DB>(
        qb: &mut QueryBuilder<DB>,
        condition: &str,
        operator: &str,
        value: &T
    )
    where  DB: Database, 
    T: 'a + Send + Sync + sqlx::Encode<'a, DB> + sqlx::Type<DB>,
    {
        qb.push(" AND ");
        qb.push(format!(" {condition} {operator} "));
        qb.push_bind(value);
    }
    fn add_optional_filter_ref<'a, T, DB>(
        qb: &mut QueryBuilder<DB>,
        condition: &str,
        operator: &str,
        value: Option<T>,
    )
    where
        DB: sqlx::Database,
        T: 'a + Send + Sync + sqlx::Encode<'a, DB> + sqlx::Type<DB>,
    {
        if let Some(v) = value {
            Self::add_filter_ref(qb, condition, operator, &v)
        };
    }
    fn add_filter<'a, DB>(
        qb: &mut QueryBuilder<DB>,
        condition: &str,
        operator: &str,
        value: impl sqlx::Encode<'a, DB> + sqlx::Type<DB> + Send + Sync,
    )
    where
        DB: Database, 
        // T: Send + Sync + sqlx::Encode<'a, DB> + sqlx::Type<DB>,
    {
        qb.push(" AND ");
        qb.push(format!(" {condition} {operator} "));
        qb.push_bind(value);
    }
    fn add_optional_filter<'a, T, DB>(
        qb: &mut QueryBuilder<DB>,
        condition: &str,
        operator: &str,
        value: Option<T>, 
    )
    where
        DB: sqlx::Database, 
        T: sqlx::Encode<'a, DB> + sqlx::Type<DB> + Send + Sync,
    {
        if let Some(v) = value {
            Self::add_filter(qb, condition, operator, v)
        };

    }
    fn make_ctx<'a, DB, P, F>(
        ctx_name: &str,
        qb: &mut QueryBuilder<DB>,
        func: F,
        params: P,
    )
    where
        DB: sqlx::Database, 
        F: FnOnce(&mut QueryBuilder<DB>, P),
    {
        qb.push("WITH ");
        qb.push(ctx_name);
        qb.push(" AS (");

        func(qb, params);

        qb.push(") ");
    }
}
