#[derive(Debug)]
pub enum RunMode {
    DEFAULT,
    DEBUG,
}

#[derive(Debug)]
pub enum ErrorState {
    FailedToInitRunMode = 1,
    FailedToInitLogger = 1 << 1,
    InvalidInput = 1 << 10,
    FailedToCreateLogFile = 1 << 11,
    StringToUSizeParseError = 1 << 20,
    StringToInt64ParseError = 1 << 21,
}

impl RunMode {
    const FAILED_TO_INIT_RUN_MODE: &'static str = "Failed to initialize RunMode.";
    pub fn new(mode: Option<String>) -> Result<Self, ErrorState> {
        match mode {
            Some(mode_inner) => match mode_inner.as_str() {
                "DEBUG" | "debug" | "Debug" => {
                    log::debug!("DEBUG MODE");

                    Ok(Self::DEBUG)
                }
                "" | "DEFAULT" | "default" | "Default" => Ok(Self::DEFAULT),
                _ => {
                    log::error!(
                        "Invalid input: {}\n\t{}",
                        Self::FAILED_TO_INIT_RUN_MODE,
                        mode_inner
                    );

                    Err(ErrorState::FailedToInitRunMode)
                }
            },
            None => Ok(Self::DEFAULT),
        }
    }
}
