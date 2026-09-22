use polars::prelude::*;
use crate::iter::ChunkIter;

#[derive(Debug)]
pub struct FieldSchema {
    pub name: &'static str,
    pub ty: &'static str,
    pub nullable: bool,
}

pub trait Csv2DbModel: Sized {
    // fn schema() -> Vec<FieldSchema>;

    fn from_df_row(df: &DataFrame, row: usize) -> anyhow::Result<Self>;

    fn from_df(df: &DataFrame) -> anyhow::Result<Vec<Self>>
    where
        Self: Sized,
    {
        (0..df.height())
            .map(|row| Self::from_df_row(df, row))
            .collect()
    }

    fn from_df_chunk(
        df: &DataFrame,
        start: usize,
        chunk_size: usize,
    ) -> anyhow::Result<Vec<Self>>
    where
        Self: Sized,
    {
        let end = (start + chunk_size).min(df.height());

        let mut out = Vec::with_capacity(end - start);

        for row in start..end {
            // println!("checking chunking process: {}", row);
            let check = Self::from_df_row(df, row);
            let val = match check {
                Ok(v) => v,
                Err(e) => {
                    println!("debug error extracting values from df: {}", e);
                    return anyhow::Result::Err(e);
                }
            };
            out.push(val);
        }

        Ok(out)
    }

    fn iter_chunks<'a>(
        df: &'a DataFrame,
        chunk_size: usize,
    ) -> impl Iterator<Item = anyhow::Result<Vec<Self>>>
    where
        Self: Sized,
    {
        ChunkIter {
            df,
            chunk_size,
            current: 0,
            _marker: std::marker::PhantomData,
        }
    }
}