const INVALID_INPUT_MSG: &str = "Invalid input.";
const TOTAL_LENGTH_AND_TARGET_NOT_SET: &str =
    "Maximum length and target for top N-th names are not set";
const TOTAL_LENGTH_AND_TARGET_ALREADY_SET: &str =
    "Maximum length and target for top N-th names are already set";

pub fn read_user_input(inputs: &mut Vec<String>) -> Result<(), crate::modes::ErrorState> {
    use crate::modes::ErrorState;
    use std::io;

    let lines = io::stdin().lines();
    let mut total_length: usize = 0;
    let mut top_n: usize = 0;

    for (index, line) in lines.into_iter().enumerate() {
        if let Ok(read_line) = line {
            let line_inner: Vec<&str> = read_line.split(" ").collect();

            match line_inner.len() {
                1 => {
                    if inputs.len() > 0 && total_length == 0 && top_n == 0 {
                        log::error!(
                            "{} {} (input: {})",
                            INVALID_INPUT_MSG,
                            TOTAL_LENGTH_AND_TARGET_NOT_SET,
                            read_line
                        );

                        return Err(ErrorState::InvalidInput);
                    }

                    if index <= top_n {
                        inputs[index - 1] = read_line;
                    };
                }
                2 => {
                    if inputs.len() > 0 {
                        log::error!(
                            "{} {} (input: {})",
                            INVALID_INPUT_MSG,
                            TOTAL_LENGTH_AND_TARGET_ALREADY_SET,
                            read_line
                        );

                        return Err(ErrorState::InvalidInput);
                    }

                    for inner_idx in 0..line_inner.len() {
                        match line_inner[inner_idx].parse::<usize>() {
                            Ok(number) => {
                                if inner_idx == 0 {
                                    total_length = number;
                                } else {
                                    top_n = number;
                                }
                            }
                            Err(err) => {
                                log::error!("{}", err);

                                return Err(ErrorState::StringToUSizeParseError);
                            }
                        }
                        for _ in 0..top_n {
                            inputs.push(String::from(""));
                        }
                    }
                }
                _ => {
                    log::error!("{} (input: {})", INVALID_INPUT_MSG, read_line);

                    return Err(ErrorState::InvalidInput);
                }
            }
        }

        if index >= total_length {
            inputs.sort();

            return Ok(());
        }
    }

    Ok(())
}

pub fn check_ans(inputs: &Vec<String>) {
    for name in inputs.iter() {
        println!("{}", *name);
    }
}
