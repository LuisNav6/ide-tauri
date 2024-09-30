

mod globals;

// Función para obtener el siguiente carácter no en blanco de la línea actual
fn get_next_char(line: &str, linepos: &mut usize, bufsize: usize) -> char {
    if *linepos >= bufsize {
        '\0' // Devuelve un carácter nulo al final de la línea
    } else {
        let c = line.chars().nth(*linepos).unwrap_or('\0'); // Usa unwrap_or para devolver un carácter nulo si el índice está fuera de rango
        *linepos += 1;
        c
    }
}

// Función para retroceder un carácter en la línea actual
fn unget_next_char(linepos: &mut usize) {
    if *linepos > 0 {
        *linepos -= 1;
    }
}

// Función para buscar palabras reservadas y devolver su TokenType correspondiente
fn reserved_lookup(s: &str) -> globals::TokenType {
    match s {
        "if" => globals::TokenType::IF,
        "else" => globals::TokenType::ELSE,
        "do" => globals::TokenType::DO,
        "while" => globals::TokenType::WHILE,
        "repeat" => globals::TokenType::REPEAT,
        "until" => globals::TokenType::UNTIL,
        "read" => globals::TokenType::READ,
        "write" => globals::TokenType::WRITE,
        "int" => globals::TokenType::INTEGER,
        "double" => globals::TokenType::DOUBLE,
        "main" => globals::TokenType::MAIN,
        "return" => globals::TokenType::RETURN,
        "/*" => globals::TokenType::InMultipleComment,
        "cin" => globals::TokenType::CIN,
        "cout" => globals::TokenType::COUT,
        _ => globals::TokenType::ID,
    }
}

// Función para realizar el análisis léxico y devolver los tokens
fn get_token(content: &str) -> (Vec<(globals::TokenType, String, usize, usize)>, Vec<(globals::TokenType, String, usize, usize)>) {
    let mut tokens = Vec::new();
    let mut errors = Vec::new();
    let mut lineno = 1;
    let mut state = globals::StateType::Start;
    let mut token_string = String::new();
    let mut linepos = 0;
    let bufsize = content.len();
    let mut column_number = 0;
    while linepos <= bufsize {
        let c = get_next_char(content, &mut linepos, bufsize);
        match state {
            globals::StateType::Start => {
                if c == '\n' {
                    lineno += 1;
                    column_number = 1;
                }
                if c.is_whitespace() {
                    // Ignorar espacios en blanco
                    column_number +=1;
                } else if c.is_ascii_alphabetic() || c == '_' {
                    state = globals::StateType::InId;
                    token_string.push(c);
                    column_number +=1;
                } else if c.is_digit(10) {
                    state = globals::StateType::InNum;
                    token_string.push(c);
                    column_number +=1;
                } else if c == '/' {
                    let next_char = get_next_char(content, &mut linepos, bufsize);
                    if next_char == '/' {
                        let next_char = get_next_char(content, &mut linepos, bufsize);
                        if next_char == '\n' {
                            lineno += 1;
                        } else {
                            unget_next_char(&mut linepos);
                            state = globals::StateType::InComment;
                            lineno += 1;
                        }
                    } else if next_char == '*' {
                        lineno += 1;
                        let next_char = get_next_char(content, &mut linepos, bufsize);
                        if next_char == '\n' {
                            lineno += 1;
                        } else {
                            unget_next_char(&mut linepos);
                            state = globals::StateType::InMultiComment;
                            lineno += 1;
                        }
                    } else {
                        tokens.push((globals::TokenType::DIVIDE, "/".to_string(), lineno, column_number - 1));
                        unget_next_char(&mut linepos)
                    }
                } else {
                    match c {
                        '=' => {
                            let next_char = get_next_char(content, &mut linepos, bufsize);
                            if next_char == '=' {
                                tokens.push((globals::TokenType::EQ, "==".to_string(), lineno, column_number - 1));
                            } else {
                                tokens.push((globals::TokenType::ASSIGN, "=".to_string(), lineno, column_number - 1));
                                unget_next_char(&mut linepos);

                            }
                        }
                        '!' => {
                            let next_char = get_next_char(content, &mut linepos, bufsize);
                            if next_char == '=' {
                                tokens.push((globals::TokenType::NEQ, "!=".to_string(), lineno, column_number - 1));
                            } else {
                                errors.push((globals::TokenType::ERROR, "!".to_string(), lineno, column_number - 1));
                                unget_next_char(&mut linepos);

                            }
                        }
                        '<' => {
                            let next_char = get_next_char(content, &mut linepos, bufsize);
                            if next_char == '=' {
                                tokens.push((globals::TokenType::LTE, "<=".to_string(), lineno, column_number - 1));
                            } else {
                                tokens.push((globals::TokenType::LT, "<".to_string(), lineno, column_number - 1));
                                unget_next_char(&mut linepos);

                            }
                        }
                        '>' => {
                            let next_char = get_next_char(content, &mut linepos, bufsize);
                            if next_char == '=' {
                                tokens.push((globals::TokenType::GTE, ">=".to_string(), lineno, column_number - 1));
                            } else {
                                tokens.push((globals::TokenType::GT, ">".to_string(), lineno, column_number - 1));
                                unget_next_char(&mut linepos);

                            }
                        }
                        '+' => {
                            let next_char = get_next_char(content, &mut linepos, bufsize);
                            if next_char == '+' {
                                tokens.push((globals::TokenType::INCREMENT, "++".to_string(), lineno, column_number - 1));
                            } else {
                                tokens.push((globals::TokenType::PLUS, "+".to_string(), lineno, column_number - 1));
                                unget_next_char(&mut linepos);

                            }
                        }
                        '-' => {
                            let next_char = get_next_char(content, &mut linepos, bufsize);
                            if next_char == '-' {
                                tokens.push((globals::TokenType::DECREMENT, "--".to_string(), lineno, column_number - 1));
                            } else {
                                tokens.push((globals::TokenType::MINUS, "-".to_string(), lineno, column_number - 1));
                                unget_next_char(&mut linepos);

                            }
                        }
                        '*' => tokens.push((globals::TokenType::TIMES, "*".to_string(), lineno, column_number)),
                        '%' => tokens.push((globals::TokenType::MODULO, "%".to_string(), lineno, column_number)),
                        '^' => tokens.push((globals::TokenType::POWER, "^".to_string(), lineno, column_number)),
                        '(' => tokens.push((globals::TokenType::LPAREN, "(".to_string(), lineno, column_number)),
                        ')' => tokens.push((globals::TokenType::RPAREN, ")".to_string(), lineno, column_number)),
                        '{' => tokens.push((globals::TokenType::LBRACE, "{".to_string(), lineno, column_number)),
                        '}' => tokens.push((globals::TokenType::RBRACE, "}".to_string(), lineno, column_number)),
                        ',' => tokens.push((globals::TokenType::COMMA, ",".to_string(), lineno, column_number)),
                        ';' => tokens.push((globals::TokenType::SEMICOLON, ";".to_string(), lineno, column_number)),
                        '&' => tokens.push((globals::TokenType::AND, "&".to_string(), lineno, column_number)),
                        '|' => tokens.push((globals::TokenType::OR, "|".to_string(), lineno, column_number)),
                        ':' => tokens.push((globals::TokenType::COLON, ":".to_string(), lineno, column_number)),
                        '\0' => {
                            state = globals::StateType::EndFile;
                        }
                        _ => errors.push((globals::TokenType::ERROR, c.to_string(), lineno, column_number - 1)),
                    }
                }
            }
            globals::StateType::InId => {
                if c.is_ascii_alphanumeric() || c == '_' {
                    token_string.push(c);
                } else {
                    tokens.push((reserved_lookup(&token_string), token_string.clone(), lineno, (column_number - 1)));
                    token_string.clear();
                    state = globals::StateType::Start;
                    unget_next_char(&mut linepos); // Retornar un carácter
                }
            }
            globals::StateType::InNum => {
                if c.is_digit(10) {
                    token_string.push(c);
                } else if c == '.' {
                    state = globals::StateType::InReal;
                    token_string.push(c);
                } else {
                    tokens.push((globals::TokenType::NumInt, token_string.clone(), lineno, (column_number - 1)));
                    token_string.clear();
                    state = globals::StateType::Start;
                    unget_next_char(&mut linepos); // Retornar un carácter
                }
            }
            globals::StateType::InReal => {
                if c.is_digit(10) {
                    token_string.push(c);
                } else if token_string.ends_with('.') {
                    errors.push((globals::TokenType::ERROR, token_string.clone(), lineno, (column_number - 1)));
                    token_string.clear();
                    state = globals::StateType::Start;
                    unget_next_char(&mut linepos); //retornar un carácter
                } else {
                    tokens.push((globals::TokenType::NumReal, token_string.clone(), lineno, (column_number - 1)));
                    token_string.clear();
                    state = globals::StateType::Start;
                    unget_next_char(&mut linepos); // Retornar un carácter
                }
            }
            globals::StateType::InComment => {
                if c == '\n' || c == '\0' {
                    state = globals::StateType::Start;
                    column_number = 1;
                }
            }
            globals::StateType::InMultiComment => {
                if c == '*' {
                    lineno += 1;
                    let next_char = get_next_char(content, &mut linepos, bufsize);
                    if next_char == '/' {
                        state = globals::StateType::Start;
                        lineno += 1;
                    } else {
                        unget_next_char(&mut linepos)
                    }
                } else if c == '\0' {
                    tokens.push((globals::TokenType::InMultipleComment, "/*".to_string(), lineno, column_number - 1));
                    println!("Error: '/*' Multiline comment not closed.");
                    state = globals::StateType::EndFile;
                }
            }
            globals::StateType::EndFile => {
                tokens.push((globals::TokenType::ENDFILE, "\0".to_string(), lineno, column_number - 1));
                break; // Salir del bucle while
            }
            _ => (),
        }
    }
    (tokens, errors)
}