#[test]
pub fn test_get_simple() {
    use crate::process::{process_get, process_set};
    use crate::storage::main::Storage;
    use std::sync::Mutex;

    let storage = Mutex::new(Storage::new());
    let args = vec!["key", "value"];
    process_set(args.clone(), storage.lock().unwrap()).unwrap();
    let expected = Ok(Some("$5\r\nvalue\r\n".to_string()));
    let result = process_get(args, storage.lock().unwrap());
    assert_eq!(result, expected);
}
