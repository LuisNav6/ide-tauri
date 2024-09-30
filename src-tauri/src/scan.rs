use crate::globals::TokenType;
use crate::globals::StateType;
use crate::globals::NodeType;
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
fn reserved_lookup(s: &str) -> TokenType {
    match s {
        "if" => TokenType::IF,
        "else" => TokenType::ELSE,
        "do" => TokenType::DO,
        "while" => TokenType::WHILE,
        "repeat" => TokenType::REPEAT,
        "until" => TokenType::UNTIL,
        "read" => TokenType::READ,
        "write" => TokenType::WRITE,
        "int" => TokenType::INTEGER,
        "double" => TokenType::DOUBLE,
        "main" => TokenType::MAIN,
        "return" => TokenType::RETURN,
        "/*" => TokenType::InMultipleComment,
        "cin" => TokenType::CIN,
        "cout" => TokenType::COUT,
        _ => TokenType::ID,
    }
}

// Función para realizar el análisis léxico y devolver los tokens
pub fn get_token(content: &str) -> (Vec<(TokenType, String, usize, usize)>, Vec<(TokenType, String, usize, usize)>) {
    let mut tokens = Vec::new();
    let mut errors = Vec::new();
    let mut lineno = 1;
    let mut state = StateType::Start;
    let mut token_string = String::new();
    let mut linepos = 0;
    let bufsize = content.len();
    let mut column_number = 0;
    while linepos <= bufsize {
        let c = get_next_char(content, &mut linepos, bufsize);
        match state {
            StateType::Start => {
                if c == '\n' {
                    lineno += 1;
                    column_number = 1;
                }
                if c.is_whitespace() {
                    // Ignorar espacios en blanco
                    column_number +=1;
                } else if c.is_ascii_alphabetic() || c == '_' {
                    state = StateType::InId;
                    token_string.push(c);
                    column_number +=1;
                } else if c.is_digit(10) {
                    state = StateType::InNum;
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
                            state = StateType::InComment;
                            lineno += 1;
                        }
                    } else if next_char == '*' {
                        lineno += 1;
                        let next_char = get_next_char(content, &mut linepos, bufsize);
                        if next_char == '\n' {
                            lineno += 1;
                        } else {
                            unget_next_char(&mut linepos);
                            state = StateType::InMultiComment;
                            lineno += 1;
                        }
                    } else {
                        tokens.push((TokenType::DIVIDE, "/".to_string(), lineno, column_number - 1));
                        unget_next_char(&mut linepos)
                    }
                } else {
                    match c {
                        '=' => {
                            let next_char = get_next_char(content, &mut linepos, bufsize);
                            if next_char == '=' {
                                tokens.push((TokenType::EQ, "==".to_string(), lineno, column_number - 1));
                            } else {
                                tokens.push((TokenType::ASSIGN, "=".to_string(), lineno, column_number - 1));
                                unget_next_char(&mut linepos);

                            }
                        }
                        '!' => {
                            let next_char = get_next_char(content, &mut linepos, bufsize);
                            if next_char == '=' {
                                tokens.push((TokenType::NEQ, "!=".to_string(), lineno, column_number - 1));
                            } else {
                                errors.push((TokenType::ERROR, "!".to_string(), lineno, column_number - 1));
                                unget_next_char(&mut linepos);

                            }
                        }
                        '<' => {
                            let next_char = get_next_char(content, &mut linepos, bufsize);
                            if next_char == '=' {
                                tokens.push((TokenType::LTE, "<=".to_string(), lineno, column_number - 1));
                            } else {
                                tokens.push((TokenType::LT, "<".to_string(), lineno, column_number - 1));
                                unget_next_char(&mut linepos);

                            }
                        }
                        '>' => {
                            let next_char = get_next_char(content, &mut linepos, bufsize);
                            if next_char == '=' {
                                tokens.push((TokenType::GTE, ">=".to_string(), lineno, column_number - 1));
                            } else {
                                tokens.push((TokenType::GT, ">".to_string(), lineno, column_number - 1));
                                unget_next_char(&mut linepos);

                            }
                        }
                        '+' => {
                            let next_char = get_next_char(content, &mut linepos, bufsize);
                            if next_char == '+' {
                                tokens.push((TokenType::INCREMENT, "++".to_string(), lineno, column_number - 1));
                            } else {
                                tokens.push((TokenType::PLUS, "+".to_string(), lineno, column_number - 1));
                                unget_next_char(&mut linepos);

                            }
                        }
                        '-' => {
                            let next_char = get_next_char(content, &mut linepos, bufsize);
                            if next_char == '-' {
                                tokens.push((TokenType::DECREMENT, "--".to_string(), lineno, column_number - 1));
                            } else {
                                tokens.push((TokenType::MINUS, "-".to_string(), lineno, column_number - 1));
                                unget_next_char(&mut linepos);

                            }
                        }
                        '*' => tokens.push((TokenType::TIMES, "*".to_string(), lineno, column_number)),
                        '%' => tokens.push((TokenType::MODULO, "%".to_string(), lineno, column_number)),
                        '^' => tokens.push((TokenType::POWER, "^".to_string(), lineno, column_number)),
                        '(' => tokens.push((TokenType::LPAREN, "(".to_string(), lineno, column_number)),
                        ')' => tokens.push((TokenType::RPAREN, ")".to_string(), lineno, column_number)),
                        '{' => tokens.push((TokenType::LBRACE, "{".to_string(), lineno, column_number)),
                        '}' => tokens.push((TokenType::RBRACE, "}".to_string(), lineno, column_number)),
                        ',' => tokens.push((TokenType::COMMA, ",".to_string(), lineno, column_number)),
                        ';' => tokens.push((TokenType::SEMICOLON, ";".to_string(), lineno, column_number)),
                        '&' => tokens.push((TokenType::AND, "&".to_string(), lineno, column_number)),
                        '|' => tokens.push((TokenType::OR, "|".to_string(), lineno, column_number)),
                        ':' => tokens.push((TokenType::COLON, ":".to_string(), lineno, column_number)),
                        '\0' => {
                            state = StateType::EndFile;
                        }
                        _ => errors.push((TokenType::ERROR, c.to_string(), lineno, column_number - 1)),
                    }
                }
            }
            StateType::InId => {
                if c.is_ascii_alphanumeric() || c == '_' {
                    token_string.push(c);
                } else {
                    tokens.push((reserved_lookup(&token_string), token_string.clone(), lineno, (column_number - 1)));
                    token_string.clear();
                    state = StateType::Start;
                    unget_next_char(&mut linepos); // Retornar un carácter
                }
            }
            StateType::InNum => {
                if c.is_digit(10) {
                    token_string.push(c);
                } else if c == '.' {
                    state = StateType::InReal;
                    token_string.push(c);
                } else {
                    tokens.push((TokenType::NumInt, token_string.clone(), lineno, (column_number - 1)));
                    token_string.clear();
                    state = StateType::Start;
                    unget_next_char(&mut linepos); // Retornar un carácter
                }
            }
            StateType::InReal => {
                if c.is_digit(10) {
                    token_string.push(c);
                } else if token_string.ends_with('.') {
                    errors.push((TokenType::ERROR, token_string.clone(), lineno, (column_number - 1)));
                    token_string.clear();
                    state = StateType::Start;
                    unget_next_char(&mut linepos); //retornar un carácter
                } else {
                    tokens.push((TokenType::NumReal, token_string.clone(), lineno, (column_number - 1)));
                    token_string.clear();
                    state = StateType::Start;
                    unget_next_char(&mut linepos); // Retornar un carácter
                }
            }
            StateType::InComment => {
                if c == '\n' || c == '\0' {
                    state = StateType::Start;
                    column_number = 1;
                }
            }
            StateType::InMultiComment => {
                if c == '*' {
                    lineno += 1;
                    let next_char = get_next_char(content, &mut linepos, bufsize);
                    if next_char == '/' {
                        state = StateType::Start;
                        lineno += 1;
                    } else {
                        unget_next_char(&mut linepos)
                    }
                } else if c == '\0' {
                    tokens.push((TokenType::InMultipleComment, "/*".to_string(), lineno, column_number - 1));
                    println!("Error: '/*' Multiline comment not closed.");
                    state = StateType::EndFile;
                }
            }
            StateType::EndFile => {
                tokens.push((TokenType::ENDFILE, "\0".to_string(), lineno, column_number - 1));
                break; // Salir del bucle while
            }
            _ => (),
        }
    }
    (tokens, errors)
}
