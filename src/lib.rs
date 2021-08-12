use std::fs;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn create_log() {
        let log = Log::new(String::from(
            "1477123675|ERROR|handler.cpp|127|findHandlers|Division by zero",
        ));
        assert_eq!(
            Log {
                timestamp: 1477123675,
                log_type: LogType::Error,
                source_file_name: "handler.cpp".to_string(),
                line: 127,
                function_name: "findHandlers".to_string(),
                log_text: "Division by zero".to_string(),
            },
            log
        );
    }
}

#[derive(Debug, Eq, PartialEq, Hash)]
pub struct Log {
    pub timestamp: u128,
    pub log_type: LogType<String>,
    pub source_file_name: String,
    pub line: u64,
    pub function_name: String,
    pub log_text: String,
}

impl Log {
    pub fn new(log: String) -> Self {
        let vals: Vec<&str> = log.split("|").collect();
        Self {
            timestamp: vals[0].parse().unwrap(),
            log_type: {
                match vals[1] {
                    "ERROR" => LogType::Error,
                    "WARNING" => LogType::Warning,
                    "TRACE" => LogType::Trace,
                    "INFO" => LogType::Info,
                    _ => LogType::Unknown(vals[1].to_string()),
                }
            },
            source_file_name: vals[2].to_string(),
            line: vals[3].parse().unwrap(),
            function_name: vals[4].to_string(),
            log_text: vals[5].to_string(),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Hash)]
pub enum LogType<T> {
    Error,
    Warning,
    Trace,
    Info,
    Unknown(T),
}

pub fn get_path_logs_file(dir: &str) -> Vec<std::path::PathBuf> {
    let dirs: Result<Vec<_>, _> = fs::read_dir(dir).unwrap().collect();
    let mut result = Vec::new();
    for file in dirs.unwrap().iter() {
        if file.file_type().unwrap().is_file()
            && file.file_name().to_str().unwrap().ends_with(".log")
        {
            result.push(file.path());
        }
    }
    result
}
