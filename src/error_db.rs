#[allow(non_camel_case_types)]
#[derive(Debug)]
pub enum ErrorCodes {
    E0001_ENABLE_RAW_MODE_FAILURE,
    E0002_NEW_CROSSTERM_TERMINAL_FAILURE,
    E0003_TERMINAL_CLEAR_FAILURE,
    E0004_APP_RENDERING_FAILURE,
    E0005_KEYBOARD_READ_FAILURE,
    E0006_SERDE_JSON_SERIALIZATION_FAILURE,
    E0007_FILE_READ_FAILURE,
    E0008_LIST_STATE_SELECTION_FAILURE,
    E0009_FILE_WRITE_FAILURE,
    E0010_HTTP_REQUEST_FAILURE,
    E0011_HTTP_RESPONSE_PARSE_FAILURE,
    E0012_RSS_CHANNEL_PARSE_FAILURE,
    E0013_ARTICLES_LIST_READ_FAILURE,
    E0014_RSS_LIST_READ_FAILURE,
    E0015_DISABLE_RAW_MODE_FAILURE,
    E0015_TERMINAL_CLEAR_FAILURE,
    E0016_TERMINAL_SHOW_CURSOR_FAILURE,
    E0017_RSS_MAX_ID_FETCH_FAILURE,
}

#[derive(Debug)]
pub struct ErrorMessages {
    pub error_code: ErrorCodes,
    pub error_message: String,
}

impl ErrorMessages {
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
        };
        ErrorMessages {
            error_code: err_code,
            error_message: err_msg,
        }
    }
}
