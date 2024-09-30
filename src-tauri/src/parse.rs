use std::sync::Mutex;

#[macro_use]
extern crate lazy_static;

mod globals;

lazy_static! {
    static ref ERRORS: Mutex<Vec<String>> = Mutex::new(Vec::new());
}

pub fn log_error(error: String) {
    let mut errors = ERRORS.lock().unwrap();
    if !errors.contains(&error) {
        errors.push(error);
    }
}

fn match_token(tokens: &[(globals::TokenType, String, usize, usize)], expected: globals::TokenType, current_token: &mut usize) -> Result<(), String> {
    if *current_token < tokens.len() && tokens[*current_token].0 == expected {
        *current_token += 1;
        Ok(())
    } else {
        println!("token in match: {:?}", tokens.get(*current_token));
        Err(format!("Error de sintaxis: se esperaba {:?} en la posición {:?}", expected, tokens.get(*current_token)))
    }
}

pub fn parse_program(tokens: &[(globals::TokenType, String, usize, usize)], current_token: &mut usize, errors: &mut Vec<String>) -> Result<globals::TreeNode, String> {
    let mut root = globals::TreeNode::new(globals::NodeType::MainRoot);
    while *current_token < tokens.len() && tokens[*current_token].0 != globals::TokenType::ENDFILE {
        match parse_statement(tokens, current_token) {
            Ok(statement_node) => root.children.push(statement_node),
            Err(err) => errors.push(err.to_string()), // Convertir el error en una cadena antes de agregarlo al vector
        }
    }

    Ok(root)
}
fn parse_statement(tokens: &[(globals::TokenType, String, usize, usize)], current_token: &mut usize) -> Result<globals::TreeNode, String> {
    match tokens.get(*current_token) {
        Some((globals::TokenType::ID, _, _, _)) => {
            if let Some((globals::TokenType::INCREMENT, _, _, _)) = tokens.get(*current_token + 1) {
                return parse_increment_statement(tokens, current_token);
            } else if let Some((globals::TokenType::DECREMENT, _, _, _)) = tokens.get(*current_token + 1) {
                return parse_decrement_statement(tokens, current_token);
            }
        }
        _ => {}
    }

    match tokens.get(*current_token) {
        Some((globals::TokenType::COLON, _, _, _)) => {
            *current_token+=1;
            return Err("Error de sintaxis: token fuera de un case ':'".to_string());
        }
        _ => {}
    }


    match tokens.get(*current_token) {
        Some((globals::TokenType::IF, _, _, _)) => return parse_if_statement(tokens, current_token),
        Some((globals::TokenType::WHILE, _, _, _)) => return parse_while_statement(tokens, current_token),
        Some((globals::TokenType::WRITE, _, _, _)) => return parse_write_statement(tokens, current_token),
        Some((globals::TokenType::READ, _, _, _)) => return parse_read_statement(tokens, current_token),
        Some((globals::TokenType::DO, _, _, _)) => return parse_do_while_statement(tokens, current_token),
        Some((globals::TokenType::REPEAT, _, _, _)) => return parse_repeat_until_statement(tokens, current_token),
        Some((globals::TokenType::RETURN, _, _, _)) => return parse_return_statement(tokens, current_token),
        Some((globals::TokenType::CIN, _, _, _)) => return parse_cin_statement(tokens, current_token),
        Some((globals::TokenType::COUT, _, _, _)) => return parse_cout_statement(tokens, current_token),
        Some((globals::TokenType::MAIN, _, _, _)) => return parse_main_function(tokens, current_token),
        Some((globals::TokenType::INTEGER, _, _, _)) => return parse_int_variable_declaration(tokens, current_token),
        Some((globals::TokenType::DOUBLE, _, _, _)) => return parse_double_variable_declaration(tokens, current_token),
        Some((globals::TokenType::ID, _, _, _)) => {
            let assignment_node = parse_assignment(tokens, current_token)?;
            if let Some((globals::TokenType::SEMICOLON, _, _, _)) = tokens.get(*current_token) {
                *current_token += 1;
                return Ok(assignment_node);
            } else {
                return Err(format!("Error de sintaxis: se esperaba ';' en la posición {:?}", *current_token));
            }
        }
        _ => if is_part_of_expression(tokens, current_token) {
            println!("token: {:?}",tokens.get(*current_token));
            return Err(format!("Error de sintaxis: se esperaba una asignación a un identificador antes de la posición {:?}", tokens.get(*current_token)));
        } else {
            return Err(format!("Error de sintaxis: token inesperado {:?}", tokens.get(*current_token)));
        },
    }
}

fn is_part_of_expression(tokens: &[(globals::TokenType, String, usize, usize)], current_token: &mut usize) -> bool {
    if parse_expression(tokens, current_token).is_ok() {
        return true;
    }
    false
}


fn parse_int_variable_declaration(tokens: &[(globals::TokenType, String, usize, usize)], current_token: &mut usize) -> Result<globals::TreeNode, String> {
    let mut node = globals::TreeNode::new(globals::NodeType::IntStatement);

    // Parsear la palabra clave 'int'
    match_token(tokens, globals::TokenType::INTEGER, current_token)?;

    // Parsear los identificadores
    loop {
        match tokens.get(*current_token) {
            Some((globals::TokenType::ID, id, _, _)) => {
                node.children.push(globals::TreeNode {
                    node_type: globals::NodeType::Factor,
                    token: Some(globals::TokenType::ID),
                    value: Some(id.clone()),
                    children: Vec::new(),
                });
                *current_token += 1;
                if let Some((globals::TokenType::COMMA, _, _, _)) = tokens.get(*current_token) {
                    *current_token += 1; // Avanzar si hay una coma
                } else {
                    break; // Salir del bucle si no hay más identificadores
                }
            }
            _ => return Err(format!("Error de sintaxis: se esperaba un identificador en la posición {:?}", tokens.get(*current_token))),
        }
    }

    // Verificar si hay un punto y coma al final
    if let Some((globals::TokenType::SEMICOLON, _, _, _)) = tokens.get(*current_token) {
        *current_token += 1; // Avanzar si hay un punto y coma
        Ok(node)
    } else {
        Err(format!("Error de sintaxis: se esperaba ';' en la posición {:?}", *current_token))
    }
}

fn parse_double_variable_declaration(tokens: &[(globals::TokenType, String, usize, usize)], current_token: &mut usize) -> Result<globals::TreeNode, String> {
    let mut node = globals::TreeNode::new(globals::NodeType::DoubleStatement);

    match_token(tokens, globals::TokenType::DOUBLE, current_token)?;
    loop {
        match tokens.get(*current_token) {
            Some((globals::TokenType::ID, id, _, _)) => {
                node.children.push(globals::TreeNode {
                    node_type: globals::NodeType::Factor,
                    token: Some(globals::TokenType::ID),
                    value: Some(id.clone()),
                    children: Vec::new(),
                });
                *current_token += 1;
                if let Some((globals::TokenType::COMMA, _, _, _)) = tokens.get(*current_token) {
                    *current_token += 1; // Avanzar si hay una coma
                } else {
                    break; // Salir del bucle si no hay más identificadores
                }
            }
            _ => return Err(format!("Error de sintaxis: se esperaba un identificador en la posición {:?}", tokens.get(*current_token))),
        }
    }

    // Verificar si hay un punto y coma al final
    if let Some((globals::TokenType::SEMICOLON, _, _, _)) = tokens.get(*current_token) {
        *current_token += 1; // Avanzar si hay un punto y coma
        Ok(node)
    } else {
        Err(format!("Error de sintaxis: se esperaba ';' en la posición {:?}", *current_token))
    }
}


fn parse_if_statement(tokens: &[(globals::TokenType, String, usize, usize)], current_token: &mut usize) -> Result<globals::TreeNode, String> {
    let mut node = globals::TreeNode::new(globals::NodeType::IfStatement);
    match_token(tokens, globals::TokenType::IF, current_token)?;
    let condition_node = parse_expression(tokens, current_token)?;
    node.children.push(condition_node);
    if  let Err(err) = match_token(tokens, globals::TokenType::LBRACE, current_token) {
        log_error(err.to_string());
    }
    let statement_node = parse_statement(tokens, current_token);
    match statement_node {
        Ok(statement_node) => {
            node.children.push(statement_node);
        }
        Err(err) => {
            log_error(err.to_string());
        }
    }
    if  let Err(err) = match_token(tokens, globals::TokenType::RBRACE, current_token) {
        log_error(err.to_string());
    }
    if let Some((globals::TokenType::ELSE, _, _, _)) = tokens.get(*current_token) {
        let else_node = parse_else_statement(tokens, current_token);
        match else_node {
            Ok(else_node) => {
                node.children.push(else_node);
            }
            Err(err) => {
                log_error(err.to_string());
            }
        }
    }
    Ok(node)
}

fn parse_else_statement(tokens: &[(globals::TokenType, String, usize, usize)], current_token: &mut usize) -> Result<globals::TreeNode, String> {
    let mut node = globals::TreeNode::new(globals::NodeType::ElseStatement);
    match_token(tokens, globals::TokenType::ELSE, current_token)?;
    if  let Err(err) = match_token(tokens, globals::TokenType::LBRACE, current_token) {
        log_error(err.to_string());
    }
    let statement_node = parse_statement(tokens, current_token);
    match statement_node {
        Ok(statement_node) => {
            node.children.push(statement_node);
        }
        Err(err) => {
            log_error(err.to_string());
        }
    }
    if  let Err(err) = match_token(tokens, globals::TokenType::RBRACE, current_token) {
        log_error(err.to_string());
    }
    Ok(node)
}


fn parse_do_while_statement(tokens: &[(globals::TokenType, String, usize, usize)], current_token: &mut usize) -> Result<globals::TreeNode, String> {
    let mut node = globals::TreeNode::new(globals::NodeType::DoWhileStatement);
    match_token(tokens, globals::TokenType::DO, current_token)?;
    if  let Err(err) = match_token(tokens, globals::TokenType::LBRACE, current_token) {
        log_error(err.to_string());
    }
    let statement_node = parse_statement(tokens, current_token);
    match statement_node {
        Ok(statement_node) => {
            node.children.push(statement_node);
        }
        Err(err) => {
            log_error(err.to_string());
        }
    }
    if  let Err(err) = match_token(tokens, globals::TokenType::RBRACE, current_token) {
        log_error(err.to_string());
    }
    if  let Err(err) = match_token(tokens, globals:TokenType::WHILE, current_token) {
        log_error(err.to_string());
    }
    let condition_node = parse_expression(tokens, current_token)?;
    node.children.push(condition_node);
    if let Some((globals:TokenType::SEMICOLON, _, _, _)) = tokens.get(*current_token) {
        *current_token += 1;
    } else {
        return Err(format!("Error de sintaxis: se esperaba ';' en la posición {:?}", *current_token));
    }
    Ok(node)
}


fn parse_while_statement(tokens: &[(globals::TokenType, String, usize, usize)], current_token: &mut usize) -> Result<globals::TreeNode, String> {
    let mut node = globals::TreeNode::new(globals::NodeType::WhileStatement);
    match_token(tokens, globals::TokenType::WHILE, current_token)?;
    let condition_node = parse_expression(tokens, current_token)?;
    node.children.push(condition_node);
    if  let Err(err) = match_token(tokens, globals::TokenType::LBRACE, current_token) {
        log_error(err.to_string());
    }
    let statement_node = parse_statement(tokens, current_token);
    match statement_node {
        Ok(statement_node) => {
            node.children.push(statement_node);
        }
        Err(err) => {
            log_error(err.to_string());
        }
    }
    if  let Err(err) = match_token(tokens, globals::TokenType::RBRACE, current_token) {
        log_error(err.to_string());
    }
    Ok(node)
}

fn parse_repeat_until_statement(tokens: &[(globals::TokenType, String, usize, usize)], current_token: &mut usize) -> Result<globals::TreeNode, String> {
    let mut node = globals::TreeNode::new(globals::NodeType::RepeatUntilStatement);
    match_token(tokens, globals::TokenType::REPEAT, current_token)?;
    if  let Err(err) = match_token(tokens, globals::TokenType::LBRACE, current_token) {
        log_error(err.to_string());
    }
    let statement_node = parse_statement(tokens, current_token);
    match statement_node {
        Ok(statement_node) => {
            node.children.push(statement_node);
        }
        Err(err) => {
            log_error(err.to_string());
        }
    }
    if  let Err(err) = match_token(tokens, globals::TokenType::RBRACE, current_token) {
        log_error(err.to_string());
    }
    if  let Err(err) = match_token(tokens, globals::TokenType::UNTIL, current_token) {
        log_error(err.to_string());
    }
    let condition_node = parse_expression(tokens, current_token)?;
    node.children.push(condition_node);
    if let Some((globals::TokenType::SEMICOLON, _, _, _)) = tokens.get(*current_token) {
        *current_token += 1;
    } else {
        return Err(format!("Error de sintaxis: se esperaba ';' en la posición {:?}", *current_token));
    }
    Ok(node)
}

fn parse_main_function(tokens: &[(globals::TokenType, String, usize, usize)], current_token: &mut usize) -> Result<globals::TreeNode, String> {
    let mut node = globals::TreeNode::new(globals::NodeType::MainFunction);
    match_token(tokens, globals::TokenType::MAIN, current_token)?;
    if  let Err(err) = match_token(tokens, globals::TokenType::LPAREN, current_token) {
        log_error(err.to_string());
    }
    if  let Err(err) = match_token(tokens, globals::TokenType::RPAREN, current_token) {
        log_error(err.to_string());
    }
    if  let Err(err) = match_token(tokens, globals::TokenType::LBRACE, current_token) {
        log_error(err.to_string());
    }
    let statement_node = parse_statement(tokens, current_token);
    match statement_node {
        Ok(statement_node) => {
            node.children.push(statement_node);
        }
        Err(err) => {
            log_error(err.to_string());
        }
    }
    if  let Err(err) = match_token(tokens, globals::TokenType::RBRACE, current_token) {
        log_error(err.to_string());
    }
    Ok(node)
}

fn parse_write_statement(tokens: &[(globals::TokenType, String, usize, usize)], current_token: &mut usize) -> Result<globals::TreeNode, String> {
    let mut node = globals::TreeNode::new(globals::NodeType::WriteStatement);
    match_token(tokens, globals::TokenType::WRITE, current_token)?;
    if let Some((globals::TokenType::ID, id, _, _)) = tokens.get(*current_token) {
        node.children.push(globals::TreeNode {
            node_type: globals::NodeType::Factor,
            token: Some(globals::TokenType::ID),
            value: Some(id.clone()),
            children: Vec::new(),
        });
        *current_token += 1;
    } else {
        return Err(format!("Error de sintaxis: se esperaba un identificador en la posición {:?}", tokens.get(*current_token)));
    }
    if let Some((globals::TokenType::SEMICOLON, _, _, _)) = tokens.get(*current_token) {
        *current_token += 1;
        Ok(node)
    } else {
        Err(format!("Error de sintaxis: se esperaba ';' en la posición {:?}", *current_token))
    }
}

fn parse_read_statement(tokens: &[(globals::TokenType, String, usize, usize)], current_token: &mut usize) -> Result<globals::TreeNode, String> {
    let mut node = globals::TreeNode::new(globals::NodeType::ReadStatement);
    match_token(tokens, globals::TokenType::READ, current_token)?;
    if let Some((globals::TokenType::ID, id, _, _)) = tokens.get(*current_token) {
        node.children.push(globals::TreeNode {
            node_type: globals::NodeType::Factor,
            token: Some(globals::TokenType::ID),
            value: Some(id.clone()),
            children: Vec::new(),
        });
        *current_token += 1;
    } else {
        return Err(format!("Error de sintaxis: se esperaba un identificador en la posición {:?}", tokens.get(*current_token)));
    }
    if let Some((globals::TokenType::SEMICOLON, _, _, _)) = tokens.get(*current_token) {
        *current_token += 1;
        Ok(node)
    } else {
        Err(format!("Error de sintaxis: se esperaba ';' en la posición {:?}", *current_token))
    }
}

fn parse_return_statement(tokens: &[(globals::TokenType, String, usize, usize)], current_token: &mut usize) -> Result<globals::TreeNode, String> {
    let mut node = globals::TreeNode::new(globals::NodeType::ReturnStatement);
    match_token(tokens, globals::TokenType::RETURN, current_token)?;
    let expression_node = parse_expression(tokens, current_token)?;
    node.children.push(expression_node);
    if let Some((globals::TokenType::SEMICOLON, _, _, _)) = tokens.get(*current_token) {
        *current_token += 1;
    } else {
        return Err(format!("Error de sintaxis: se esperaba ';' en la posición {:?}", *current_token));
    }
    Ok(node)
}

fn parse_cin_statement(tokens: &[(globals::TokenType, String, usize, usize)], current_token: &mut usize) -> Result<globals::TreeNode, String> {
    let mut node = globals::TreeNode::new(globals::NodeType::CinStatement);
    match_token(tokens, globals::TokenType::CIN, current_token)?;
    if let Some((globals::TokenType::ID, id, _, _)) = tokens.get(*current_token) {
        node.children.push(globals::TreeNode {
            node_type: globals::NodeType::Factor,
            token: Some(Tglobals::okenType::ID),
            value: Some(id.clone()),
            children: Vec::new(),
        });
        *current_token += 1;
    } else {
        return Err(format!("Error de sintaxis: se esperaba un identificador en la posición {:?}", tokens.get(*current_token)));
    }
    if let Some((globals::TokenType::SEMICOLON, _, _, _)) = tokens.get(*current_token) {
        *current_token += 1;
        Ok(node)
    } else {
        Err(format!("Error de sintaxis: se esperaba ';' en la posición {:?}", *current_token))
    }
}

fn parse_cout_statement(tokens: &[(globals::TokenType, String, usize, usize)], current_token: &mut usize) -> Result<globals::TreeNode, String> {
    let mut node = globals::TreeNode::new(globals::NodeType::CoutStatement);
    match_token(tokens, globals::TokenType::COUT, current_token)?;
    let expression_node = parse_expression(tokens, current_token)?;
    node.children.push(expression_node);
    if let Some((globals::TokenType::SEMICOLON, _, _, _)) = tokens.get(*current_token) {
        *current_token += 1;
    } else {
        return Err(format!("Error de sintaxis: se esperaba ';' en la posición {:?}", *current_token));
    }
    Ok(node)
}

fn parse_increment_statement(tokens: &[(globals::TokenType, String, usize, usize)], current_token: &mut usize) -> Result<globals::TreeNode, String> {
    let mut node = globals::TreeNode::new(Nglobals::odeType::Increment);
    if let Some((globals::TokenType::ID, id, _, _)) = tokens.get(*current_token) {
        node.children.push(globals::TreeNode {
            node_type: globals::NodeType::Factor,
            token: Some(globals::TokenType::ID),
            value: Some(id.clone()),
            children: Vec::new(),
        });
        *current_token += 2;
        if let Some((globals::TokenType::SEMICOLON, _, _, _)) = tokens.get(*current_token) {
            *current_token += 1;
        } else {
            return Err(format!("Error de sintaxis: se esperaba ';' en la posición {:?}", *current_token));
        }
        Ok(node)
    } else {
        Err(format!("Error de sintaxis: se esperaba un identificador en la posición {:?}", tokens.get(*current_token)))
    }
}

fn parse_decrement_statement(tokens: &[(globals::TokenType, String, usize, usize)], current_token: &mut usize) -> Result<globals::TreeNode, String> {
    let mut node = globals::TreeNode::new(globals::NodeType::Decrement);
    if let Some((globals::TokenType::ID, id, _, _)) = tokens.get(*current_token) {
        node.children.push(globals::TreeNode {
            node_type: globals::NodeType::Factor,
            token: Some(globals::TokenType::ID),
            value: Some(id.clone()),
            children: Vec::new(),
        });
        *current_token += 2;
        if let Some((globals::TokenType::SEMICOLON, _, _, _)) = tokens.get(*current_token) {
            *current_token += 1;
        } else {
            return Err(format!("Error de sintaxis: se esperaba ';' en la posición {:?}", *current_token));
        }
        Ok(node)
    } else {
        Err(format!("Error de sintaxis: se esperaba un identificador en la posición {:?}", tokens.get(*current_token)))
    }
}


fn parse_expression(tokens: &[(globals::TokenType, String, usize, usize)], current_token: &mut usize) -> Result<globals::TreeNode, String> {
    let mut node = parse_term(tokens, current_token)?;
    while let Some((token, value, _, _)) = tokens.get(*current_token) {
        match token {
            globals::TokenType::PLUS | globals::TokenType::MINUS | globals::TokenType::LT | globals::TokenType::LTE | globals::TokenType::GT | globals::TokenType::GTE | globals::TokenType::EQ | globals::TokenType::NEQ | globals::TokenType::AND | globals::TokenType::OR => {
                *current_token += 1;
                let term_node = parse_term(tokens, current_token)?;
                let mut expression_node = globals::TreeNode::new(globals::NodeType::Expression);
                expression_node.children.push(node);
                expression_node.children.push(globals::TreeNode {
                    node_type: globals::NodeType::Factor,
                    token: Some(token.clone()),
                    value: Some(value.clone()),
                    children: Vec::new(),
                });
                expression_node.children.push(term_node);
                node = expression_node;
            }
            _ => break,
        }
    }
    Ok(node)
}

fn parse_term(tokens: &[(globals::TokenType, String, usize, usize)], current_token: &mut usize) -> Result<globals::TreeNode, String> {
    let mut node = parse_factor(tokens, current_token)?;
    while let Some((token, value, _, _)) = tokens.get(*current_token) {
        match token {
            globals::TokenType::TIMES | globals::TokenType::DIVIDE | globals::TokenType::MODULO | globals::TokenType::POWER => {
                *current_token += 1;
                let factor_node = parse_factor(tokens, current_token)?;
                let mut term_node = globals::TreeNode::new(globals::NodeType::Term);
                term_node.children.push(node);
                term_node.children.push(globals::TreeNode {
                    node_type: globals::NodeType::Factor,
                    token: Some(token.clone()),
                    value: Some(value.clone()),
                    children: Vec::new(),
                });
                term_node.children.push(factor_node);
                node = term_node;
            }
            _ => break,
        }
    }
    Ok(node)
}

fn parse_factor(tokens: &[(globals::TokenType, String, usize, usize)], current_token: &mut usize) -> Result<globals::TreeNode, String> {
    if let Some((token, value, _, _)) = tokens.get(*current_token) {
        let mut node = globals::TreeNode::new(globals::NodeType::Factor);
        match token {
            globals::TokenType::NumInt | globals::TokenType::NumReal | globals::TokenType::ID => {
                node.token = Some(token.clone());
                node.value = Some(value.clone());
                *current_token += 1;
                Ok(node)
            }
            globals::TokenType::LPAREN => {
                *current_token += 1;
                let expression_node = parse_expression(tokens, current_token)?;
                if  let Err(err) = match_token(tokens, globals::TokenType::RPAREN, current_token) {
                    log_error(err.to_string());
                }
                node.children.push(expression_node);
                Ok(node)
            }
            _ => Err(format!("Error de sintaxis: token inesperado {:?}", tokens.get(*current_token))),
        }
    } else {
        Err(format!("Error de sintaxis: token inesperado en la posición {:?}", tokens.get(*current_token)))
    }
}

fn parse_assignment(tokens: &[(globals::TokenType, String, usize, usize)], current_token: &mut usize) -> Result<globals::TreeNode, String> {
    let mut node = globals::TreeNode::new(globals::NodeType::Assignment);
    if let Some((globals::TokenType::ID, id, _, _)) = tokens.get(*current_token) {
        node.children.push(globals::globals::TreeNode {
            node_type: globals::NodeType::Factor,
            token: Some(globals::TokenType::ID),
            value: Some(id.clone()),
            children: Vec::new(),
        });
        *current_token += 1;
        if  let Err(err) = match_token(tokens, globals::TokenType::ASSIGN, current_token) {
            log_error(err.to_string());
        }
        let expression_node = parse_expression(tokens, current_token)?;
        node.children.push(expression_node);
        Ok(node)
    } else {
        Err(format!("Error de sintaxis: se esperaba un identificador en la posición {:?}", tokens.get(*current_token)))
    }
}