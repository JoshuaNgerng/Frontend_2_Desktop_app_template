use polars::prelude::*;
use crate::csv_2_db::Csv2DbModel;
pub struct ChunkIter<'a, T> {
    pub df: &'a DataFrame,
    pub chunk_size: usize,
    pub current: usize,
    pub _marker: std::marker::PhantomData<T>,
}

impl<'a, T> Iterator for ChunkIter<'a, T>
where
    T: Csv2DbModel,
{
    type Item = anyhow::Result<Vec<T>>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current >= self.df.height() {
            return None;
        }

        let start = self.current;

        self.current += self.chunk_size;

        Some(T::from_df_chunk(
            self.df,
            start,
            self.chunk_size,
        ))
    }
}
