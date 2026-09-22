mod iter;
pub mod csv_2_db;
pub mod extract;

pub mod bulk_csv_insert;

// going to deprecate

pub use csv_2_db::FieldSchema;
pub use csv_2_db::Csv2DbModel;

pub use extract::FromCell;

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
