pub mod error; pub mod format; pub mod model; pub mod engine; pub use error::CoreError; pub use format::FileFormat; pub use model::*; pub use engine::*;
#[cfg(test)] mod tests { use super::*; #[test] fn unicode_format(){assert_eq!(FileFormat::from_path(std::path::Path::new("报告.XLSX")).unwrap(),FileFormat::Xlsx);} }
