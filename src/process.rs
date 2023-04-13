use crate::storage::Storage;
use crate::structs::CommandError;
use crate::utils::check_expected_num_args;
use log::error;

pub fn process_echo(mut commands: Vec<&str>) -> Result<Option<String>, CommandError> {
    let contains_argument = check_expected_num_args(commands.clone(), 1);
    match contains_argument {
        Ok(_) => {
            let message = commands.remove(0);
            let len_char_msg = message.len().to_string();
            Ok(Some(format!("${}\r\n{}\r\n", len_char_msg, message)))
        }
        Err(_e) => Err(CommandError::InvalidNumberOfArguments {
            message: "Invalid number of arguments".to_string(),
        }),
    }
}

pub fn process_set(mut commands: Vec<&str>) -> Result<Option<String>, CommandError> {
    let contains_arguments = check_expected_num_args(commands.clone(), 2);
    match contains_arguments {
        Ok(_) => {
            let key = commands.remove(0);
            let value = commands.remove(0);
            let mut storage = Storage::new();

            match storage.set(key, value) {
                Ok(_) => Ok(Some("+OK\r\n".to_string())),
                Err(e) => {
                    error!("Error SET: {:?}", e);
                    Err(CommandError::InvalidCommand {
                        message: format!(
                            "Error during insertion of the key-value {}: {}",
                            key, value
                        ),
                    })
                }
            }
        }
        Err(_e) => Err(CommandError::InvalidNumberOfArguments {
            message: "Invalid number of arguments".to_string(),
        }),
    }
}

pub fn process_get(mut commands: Vec<&str>) -> Result<Option<String>, CommandError> {
    let contains_arguments = check_expected_num_args(commands.clone(), 1);
    match contains_arguments {
        Ok(_) => {
            let key = commands.remove(0);
            let storage = Storage::new();

            match storage.get(key) {
                Ok(value) => {
                    let len_char_msg = value.len().to_string();
                    Ok(Some(format!("${}\r\n{}\r\n", len_char_msg, value)))
                }
                Err(_e) => {
                    error!("Error GET: {:?}", _e);
                    Err(CommandError::InvalidCommand {
                        message: format!("Key {} not found", key),
                    })
                }
            }
        }
        Err(_e) => Err(CommandError::InvalidNumberOfArguments {
            message: "Invalid number of arguments".to_string(),
        }),
    }
}
