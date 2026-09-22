use polars::prelude::*;

//
// Main trait
//
pub trait FromCell: Sized {
    fn extract(
        df: &DataFrame,
        col: &str,
        row: usize,
    ) -> anyhow::Result<Self>;
}

//
// Internal customization trait
//
// This is what allows Option<T> to have:
// - generic fallback behavior
// - special handling for specific T
//
trait OptionBehavior: Sized {
    fn extract_option(
        df: &DataFrame,
        col: &str,
        row: usize,
    ) -> anyhow::Result<Option<Self>>;
}

//
// Marker trait:
//
// Types implementing this use the DEFAULT option behavior.
//
// IMPORTANT:
// Do NOT implement this for String,
// because String has custom behavior.
//
trait UseDefaultOptionBehavior {}

impl UseDefaultOptionBehavior for i32 {}
impl UseDefaultOptionBehavior for i64 {}
impl UseDefaultOptionBehavior for f64 {}

impl FromCell for i64 {
    fn extract(
        df: &DataFrame, col: &str, row: usize
    ) -> anyhow::Result<Self> {
        let s = df.column(col)?.cast(&DataType::Int64)?;
        Ok(
            s.i64()?
            .get(row)
            .ok_or_else(|| anyhow::anyhow!("null"))?
        )
    }
}

impl FromCell for i32 {
    fn extract(
        df: &DataFrame, col: &str, row: usize
    ) -> anyhow::Result<Self> {
        let s = df.column(col)?.cast(&DataType::Int32)?;
        Ok(
            s.i32()?
            .get(row)
            .ok_or_else(|| anyhow::anyhow!("null"))?
        )
    }
}

impl FromCell for u64 {
    fn extract(
        df: &DataFrame, col: &str, row: usize
    ) -> anyhow::Result<Self> {
        let s = df.column(col)?.cast(&DataType::UInt64)?;
        Ok(
            s.u64()?
            .get(row)
            .ok_or_else(|| anyhow::anyhow!("null"))?
        )
    }
}

impl FromCell for u32 {
    fn extract(
        df: &DataFrame, col: &str, row: usize
    ) -> anyhow::Result<Self> {
        let s = df.column(col)?.cast(&DataType::UInt32)?;
        Ok(
            s.u32()?
            .get(row)
            .ok_or_else(|| anyhow::anyhow!("null"))?
        )
    }
}

impl FromCell for f64 {
    fn extract(
        df: &DataFrame, col: &str, row: usize
    ) -> anyhow::Result<Self> {
        let s = df.column(col)?.cast(&DataType::Float64)?;
        Ok(
            s.f64()?
            .get(row)
            .ok_or_else(|| anyhow::anyhow!("null"))?
        )
    }
}

impl FromCell for String {
    fn extract(
        df: &DataFrame, col: &str, row: usize
    ) -> anyhow::Result<Self> {
        let s = df.column(col)?.cast(&DataType::String)?;
        Ok(
            s.str()?
            .get(row)
            .ok_or_else(|| anyhow::anyhow!("null"))?
            .to_string()
        )
    }
}

/* generic optional extract interface for extract  */
impl<T: OptionBehavior> FromCell for Option<T> {
    fn extract(
        df: &DataFrame,
        col: &str,
        row: usize,
    ) -> anyhow::Result<Self> {
        T::extract_option(df, col, row)
    }
}

//
// ============================================================
// Default Option<T> behavior
// ============================================================
//
// Any type implementing:
// - FromCell
// - UseDefaultOptionBehavior
//
// automatically gets:
//
// Err(_) => None
//
impl<T> OptionBehavior for T
where
    T: FromCell + UseDefaultOptionBehavior,
{
    fn extract_option(
        df: &DataFrame,
        col: &str,
        row: usize,
    ) -> anyhow::Result<Option<Self>> {
        match T::extract(df, col, row) {
            Ok(v) => Ok(Some(v)),
            Err(_) => Ok(None),
        }
    }
}

//
// ============================================================
// SPECIAL CASE: String
// ============================================================
//
// This overrides the default behavior.
//
impl OptionBehavior for String {
    fn extract_option(
        df: &DataFrame,
        col: &str,
        row: usize,
    ) -> anyhow::Result<Option<Self>> {
        let s = String::extract(df, col, row)?;

        // Example custom rule:
        // empty strings become None
        if s.trim().is_empty() {
            Ok(None)
        } else {
            Ok(Some(s))
        }
    }
}

/*
ref
T = String
  ├── String
  │     FromCell → direct impl (non-option)
  │
  └── Option<String>
        FromCell → Option<T> impl
                    └── OptionBehavior for String (custom)

T = i32
  ├── i32
  │     FromCell → direct impl
  │
  └── Option<i32>
        FromCell → Option<T> impl
                    └── OptionBehavior fallback
*/