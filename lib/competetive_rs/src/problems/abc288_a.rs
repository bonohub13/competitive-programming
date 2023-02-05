pub fn read_user_input(
    inputs: &mut Vec<[i64; 2]>,
    mode: &crate::modes::RunMode,
) -> Result<(), crate::modes::ErrorState> {
    use crate::modes::{ErrorState, RunMode};
    use std::io;

    let lines = io::stdin().lines();
    let mut iter_counter: usize = 0;

    for line in lines.into_iter() {
        if let Ok(read_line) = line {
            let line_inner: Vec<&str> = read_line.split(" ").collect();

            match line_inner.len() {
                1 => {
                    if inputs.len() == 0 {
                        match line_inner[0].parse::<usize>() {
                            Ok(size) => {
                                for _ in 0..size {
                                    inputs.push([0, 0]);
                                }
                                match *mode {
                                    RunMode::DEBUG => {
                                        log::debug!("inputs length is resized to {}", inputs.len());
                                    }
                                    _ => (),
                                }
                            }
                            Err(err) => {
                                log::error!("{}", err);

                                return Err(ErrorState::StringToUSizeParseError);
                            }
                        }
                    } else {
                        log::error!("Invalid number of inputs");

                        return Err(ErrorState::InvalidInput);
                    }
                }
                2 => {
                    if inputs.len() > 0 {
                        let mut inputs_i: [i64; 2] = [0, 0];
                        for i in 0..2 {
                            match line_inner[i].parse::<i64>() {
                                Ok(data) => inputs_i[i] = data,
                                Err(err) => {
                                    log::error!("{}", err);

                                    return Err(ErrorState::StringToInt64ParseError);
                                }
                            }
                        }

                        inputs[iter_counter - 1] = inputs_i;
                    }
                }
                _ => {
                    log::error!("Invalid number of inputs");

                    return Err(ErrorState::InvalidInput);
                }
            }

            iter_counter += 1;
            if (inputs.len() > 0 && iter_counter > inputs.len()) || inputs.len() == 0 {
                break;
            }
        }
    }
    Ok(())
}

pub fn check_ans(inputs: &Vec<[i64; 2]>) {
    for &input in inputs.iter() {
        println!("{}", input[0] + input[1]);
    }
}
