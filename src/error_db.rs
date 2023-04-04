//! Defines the error codes, used in the application, and their corresponding descriptions
//!

#[allow(non_camel_case_types)]
#[derive(Debug)]
/// Defines the list of error codes used in the application
pub enum ErrorCodes {
    /// Unable to convert terminal to raw mode
    E0001_ENABLE_RAW_MODE_FAILURE,
    /// Unable to open terminal with crossterm backend
    E0002_NEW_CROSSTERM_TERMINAL_FAILURE,
    /// Unable to clear crossterm terminal
    E0003_TERMINAL_CLEAR_FAILURE,
    /// Unable to render application components on terminal
    E0004_APP_RENDERING_FAILURE,
    /// Unable to read key press events from keyboard
    E0005_KEYBOARD_READ_FAILURE,
    /// Unable to convert data structure to JSON serializable format
    E0006_SERDE_JSON_SERIALIZATION_FAILURE,
    /// Unable to read file provided
    E0007_FILE_READ_FAILURE,
    /// Unable to select index in List State provided
    E0008_LIST_STATE_SELECTION_FAILURE,
    /// Unable to write content to file provided
    E0009_FILE_WRITE_FAILURE,
    /// Didn't receive any response from HTTP link provided
    E0010_HTTP_REQUEST_FAILURE,
    /// Unable to parse HTTP response
    E0011_HTTP_RESPONSE_PARSE_FAILURE,
    /// Unable to parse RSS content in HTTP response
    E0012_RSS_CHANNEL_PARSE_FAILURE,
    /// Unable to read articles list from Articles database
    E0013_ARTICLES_LIST_READ_FAILURE,
    /// Unable to read RSS feeds list from RSS database
    E0014_RSS_LIST_READ_FAILURE,
    /// Unable to disable raw mode in terminal
    E0015_DISABLE_RAW_MODE_FAILURE,
    /// Unable to clear contents in the terminal
    E0015_TERMINAL_CLEAR_FAILURE,
    /// Unable to show cursor in the terminal
    E0016_TERMINAL_SHOW_CURSOR_FAILURE,
    /// Unable to fetch max RSS id from the database
    E0017_RSS_MAX_ID_FETCH_FAILURE,
    /// Unable to build Tokio multi-thread runtime
    E0018_TOKIO_RUNTIME_BUILDER_FAILURE,
    /// Unable to find config file for log4rs logging
    E0019_LOGGING_CONFIG_FILE_READ_FAILURE,
    /// Unable to parse provided timestamp into RFC2822 format
    E0020_RFC2822_TIMESTAMP_PARSE_FAILURE,
    /// Unable to fetch max timestamp from Articles database
    E0021_ARTICLE_MAX_TIMESTAMP_FETCH_FAILURE,
}

#[derive(Debug)]
/// Defines metadata for mapping the error codes to corresponding error descriptions
pub struct ErrorMessages {
    /// Error codes defined as per "ErrorCodes" enum
    pub error_code: ErrorCodes,
    /// Error descriptions for corresponding error codes
    pub error_message: String,
}

impl ErrorMessages {
    /// Provides the error code - error description mapping based on input
    pub fn new(err_code: ErrorCodes) -> ErrorMessages {
        let err_msg = match err_code {
            ErrorCodes::E0001_ENABLE_RAW_MODE_FAILURE => {
                String::from("Unable to convert terminal to raw mode.")
            }
            ErrorCodes::E0002_NEW_CROSSTERM_TERMINAL_FAILURE => {
                String::from("Unable to open terminal with crossterm backend.")
            }
            ErrorCodes::E0003_TERMINAL_CLEAR_FAILURE => {
                String::from("Unable to clear crossterm terminal.")
            }
            ErrorCodes::E0004_APP_RENDERING_FAILURE => {
                String::from("Unable to render application components on terminal.")
            }
            ErrorCodes::E0005_KEYBOARD_READ_FAILURE => {
                String::from("Unable to read key press events from keyboard.")
            }
            ErrorCodes::E0006_SERDE_JSON_SERIALIZATION_FAILURE => {
                String::from("Unable to convert data structure to JSON serializable format.")
            }
            ErrorCodes::E0007_FILE_READ_FAILURE => String::from("Unable to read file provided."),
            ErrorCodes::E0008_LIST_STATE_SELECTION_FAILURE => {
                String::from("Unable to select index in List State provided.")
            }
            ErrorCodes::E0009_FILE_WRITE_FAILURE => {
                String::from("Unable to write content to file provided.")
            }
            ErrorCodes::E0010_HTTP_REQUEST_FAILURE => {
                String::from("Didn't receive any response from HTTP link provided.")
            }
            ErrorCodes::E0011_HTTP_RESPONSE_PARSE_FAILURE => {
                String::from("Unable to parse HTTP response.")
            }
            ErrorCodes::E0012_RSS_CHANNEL_PARSE_FAILURE => {
                String::from("Unable to parse RSS content in HTTP response.")
            }
            ErrorCodes::E0013_ARTICLES_LIST_READ_FAILURE => {
                String::from("Unable to read articles list from Articles database.")
            }
            ErrorCodes::E0014_RSS_LIST_READ_FAILURE => {
                String::from("Unable to read RSS feeds list from RSS database.")
            }
            ErrorCodes::E0015_DISABLE_RAW_MODE_FAILURE => {
                String::from("Unable to disable raw mode in terminal.")
            }
            ErrorCodes::E0015_TERMINAL_CLEAR_FAILURE => {
                String::from("Unable to clear contents in the terminal.")
            }
            ErrorCodes::E0016_TERMINAL_SHOW_CURSOR_FAILURE => {
                String::from("Unable to show cursor in the terminal.")
            }
            ErrorCodes::E0017_RSS_MAX_ID_FETCH_FAILURE => {
                String::from("Unable to fetch max RSS id from the database.")
            }
            ErrorCodes::E0018_TOKIO_RUNTIME_BUILDER_FAILURE => {
                String::from("Unable to build Tokio multi-thread runtime.")
            }
            ErrorCodes::E0019_LOGGING_CONFIG_FILE_READ_FAILURE => {
                String::from("Unable to find config file for log4rs logging.")
            }
            ErrorCodes::E0020_RFC2822_TIMESTAMP_PARSE_FAILURE => {
                String::from("Unable to parse provided timestamp into RFC2822 format.")
            }
            ErrorCodes::E0021_ARTICLE_MAX_TIMESTAMP_FETCH_FAILURE => {
                String::from("Unable to fetch max timestamp from Articles database.")
            }
        };
        ErrorMessages {
            error_code: err_code,
            error_message: err_msg,
        }
    }
}
