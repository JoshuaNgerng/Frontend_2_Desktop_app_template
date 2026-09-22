use anyhow::Result;
use polars::prelude::*;
use sqlx::{QueryBuilder, Sqlite};

pub trait BulkInsert {
    type Context;
    type Columns<'a>;

    fn prepare<'a>(
        df: &'a DataFrame,
    ) -> Result<Self::Columns<'a>>;

    fn insert(
        qb: &mut QueryBuilder<Sqlite>,
        start: usize,
        end: usize,
        cols: &Self::Columns<'_>,
        ctx: &Self::Context,
    ) -> Result<()>;
}
