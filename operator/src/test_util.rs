#[macro_export]
macro_rules! test_resource {($fname:expr) => (
  concat!(env!("CARGO_MANIFEST_DIR"), "/resources/tests/", $fname) // assumes Linux ('/')!
)}
