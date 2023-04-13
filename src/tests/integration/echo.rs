#[test]
pub fn test_process_echo_simple() {
    use crate::process::process_echo;

    let arg = vec!["hello"];
    let expected = Ok(Some("$5\r\nhello\r\n".to_string()));
    let result = process_echo(arg);
    assert_eq!(result, expected);
}

#[test]
pub fn test_process_echo_empty() {
    use crate::process::process_echo;

    // TODO: Fix this test
    let arg = vec![""];
    let expected = Ok(Some("$0\r\n\r\n".to_string()));
    let result = process_echo(arg);
    assert_eq!(result, expected);
}
