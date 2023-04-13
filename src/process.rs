use std::sync::MutexGuard;

use log::error;

use crate::errors::CommandError;
use crate::storage::main::Storage;
use crate::utils::check_expected_num_args;

pub fn process_echo(mut commands: Vec<&str>) -> Result<Option<String>, CommandError> {
    let contains_argument = check_expected_num_args(commands.clone(), 1);
    match contains_argument {
        Ok(_) => {
            let message = commands.remove(0);
            let len_char_msg = message.len().to_string();
            Ok(Some(format!("${}\r\n{}\r\n", len_char_msg, message)))
        }
        Err(_e) => Err(CommandError::InvalidNumberOfArguments {
            message: "-ERR Invalid number of arguments".to_string(),
        }),
    }
}

pub fn process_set(
    mut commands: Vec<&str>,
    mut storage: MutexGuard<Storage>,
) -> Result<Option<String>, CommandError> {
    let contains_arguments = check_expected_num_args(commands.clone(), 2);
    match contains_arguments {
        Ok(_) => {
            let key = commands.remove(0);
            let value = commands.remove(0);

            // if commands.contains(&"PX") {
            //
            // }

            match storage.set(key, value, None) {
                Ok(_) => Ok(Some("+OK\r\n".to_string())),
                Err(e) => {
                    error!("Error SET: {:?}", e);
                    Err(CommandError::InvalidCommand {
                        message: "$-1".to_string(),
                    })
                }
            }
        }
        Err(_e) => Err(CommandError::InvalidNumberOfArguments {
            message: "-ERR Invalid number of arguments".to_string(),
        }),
    }
}

pub fn process_get(
    mut commands: Vec<&str>,
    storage: MutexGuard<Storage>,
) -> Result<Option<String>, CommandError> {
    let contains_arguments = check_expected_num_args(commands.clone(), 1);
    match contains_arguments {
        Ok(_) => {
            let key = commands.remove(0);

            match storage.get(key) {
                Ok(value) => {
                    let len_char_msg = value.len().to_string();
                    Ok(Some(format!("${}\r\n{}\r\n", len_char_msg, value)))
                }
                Err(_e) => {
                    error!("Error GET: {:?}", _e);
                    Err(CommandError::InvalidCommand {
                        message: "$-1\r\n".to_string(),
                    })
                }
            }
        }
        Err(_e) => Err(CommandError::InvalidNumberOfArguments {
            message: "-ERR Invalid number of arguments".to_string(),
        }),
    }
}

// pub fn process_save(mut commands: Vec<&str>, storage: Storage) -> Result<Option<String>, CommandError> {
//     let contains_arguments = check_expected_num_args(commands.clone(), 1);
//     match contains_arguments {
//         Ok(_) => {
//             let path = commands.remove(0);
//
//             match storage.save(path) {
//                 Some(_) => Ok(Some("+OK\r\n".to_string())),
//                 Err(_e) => {
//                     error!("Error SAVE: {:?}", _e);
//                     Err(CommandError::InvalidCommand {
//                         message: format!("Error during saving the database in {}", path),
//                     })
//                 }
//             }
//         }
//         Err(_e) => Err(CommandError::InvalidNumberOfArguments {
//             message: "Invalid number of arguments".to_string(),
//         }),
//     }
// }

// pub fn process_load(mut commands: Vec<&str>, storage: Storage) -> Result<Option<String>, CommandError> {
//     let contains_arguments = check_expected_num_args(commands.clone(), 1);
//     match contains_arguments {
//         Ok(_) => {
//             let path = commands.remove(0);
//
//             match storage.save(path) {
//                 Some(_) => Ok(Some("+OK\r\n".to_string())),
//                 Err(_e) => {
//                     error!("Error SAVE: {:?}", _e);
//                     Err(CommandError::InvalidCommand {
//                         message: format!("Error during saving the database in {}", path),
//                     })
//                 }
//             }
//         }
//         Err(_e) => Err(CommandError::InvalidNumberOfArguments {
//             message: "Invalid number of arguments".to_string(),
//         }),
//     }
// }
