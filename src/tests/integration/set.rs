#[test]
pub fn test_process_set_simple() {
    use crate::process::process_set;
    use crate::storage::main::Storage;
    use std::sync::Mutex;

    let storage = Mutex::new(Storage::new());
    let args = vec!["key", "value"];
    let expected = Ok(Some("+OK\r\n".to_string()));
    let result = process_set(args, storage.lock().unwrap());
    assert_eq!(result, expected);
}
