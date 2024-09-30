use std::sync::Mutex;
use crate::globals::TokenType;
use crate::globals::NodeType;

#[macro_use]
extern crate lazy_static;

mod globals;


lazy_static! {
    static ref ERRORS: Mutex<Vec<String>> = Mutex::new(Vec::new());
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct TreeNode {
    node_type: NodeType,
    token: Option<TokenType>,
    value: Option<String>,
    children: Vec<TreeNode>,
}

pub fn log_error(error: String) {
    let mut errors = ERRORS.lock().unwrap();
    if !errors.contains(&error) {
        errors.push(error);
    }
}

fn match_token(tokens: &[(TokenType, String, usize, usize)], expected: TokenType, current_token: &mut usize) -> Result<(), String> {
    if *current_token < tokens.len() && tokens[*current_token].0 == expected {
        *current_token += 1;
        Ok(())
    } else {
        println!("token in match: {:?}", tokens.get(*current_token));
        Err(format!("Error de sintaxis: se esperaba {:?} en la posición {:?}", expected, tokens.get(*current_token)))
    }
}

pub fn parse_program(tokens: &[(TokenType, String, usize, usize)], current_token: &mut usize, errors: &mut Vec<String>) -> Result<TreeNode, String> {
    let mut root = TreeNode::new(NodeType::MainRoot);
    while *current_token < tokens.len() && tokens[*current_token].0 != TokenType::ENDFILE {
        match parse_statement(tokens, current_token) {
            Ok(statement_node) => root.children.push(statement_node),
            Err(err) => errors.push(err.to_string()), // Convertir el error en una cadena antes de agregarlo al vector
        }
    }

    Ok(root)
}
fn parse_statement(tokens: &[(TokenType, String, usize, usize)], current_token: &mut usize) -> Result<TreeNode, String> {
    match tokens.get(*current_token) {
        Some((TokenType::ID, _, _, _)) => {
            if let Some((TokenType::INCREMENT, _, _, _)) = tokens.get(*current_token + 1) {
                return parse_increment_statement(tokens, current_token);
            } else if let Some((TokenType::DECREMENT, _, _, _)) = tokens.get(*current_token + 1) {
                return parse_decrement_statement(tokens, current_token);
            }
        }
        _ => {}
    }

    match tokens.get(*current_token) {
        Some((TokenType::COLON, _, _, _)) => {
            *current_token+=1;
            return Err("Error de sintaxis: token fuera de un case ':'".to_string());
        }
        _ => {}
    }


    match tokens.get(*current_token) {
        Some((TokenType::IF, _, _, _)) => return parse_if_statement(tokens, current_token),
        Some((TokenType::WHILE, _, _, _)) => return parse_while_statement(tokens, current_token),
        Some((TokenType::WRITE, _, _, _)) => return parse_write_statement(tokens, current_token),
        Some((TokenType::READ, _, _, _)) => return parse_read_statement(tokens, current_token),
        Some((TokenType::DO, _, _, _)) => return parse_do_while_statement(tokens, current_token),
        Some((TokenType::REPEAT, _, _, _)) => return parse_repeat_until_statement(tokens, current_token),
        Some((TokenType::RETURN, _, _, _)) => return parse_return_statement(tokens, current_token),
        Some((TokenType::CIN, _, _, _)) => return parse_cin_statement(tokens, current_token),
        Some((TokenType::COUT, _, _, _)) => return parse_cout_statement(tokens, current_token),
        Some((TokenType::MAIN, _, _, _)) => return parse_main_function(tokens, current_token),
        Some((TokenType::INTEGER, _, _, _)) => return parse_int_variable_declaration(tokens, current_token),
        Some((TokenType::DOUBLE, _, _, _)) => return parse_double_variable_declaration(tokens, current_token),
        Some((TokenType::ID, _, _, _)) => {
            let assignment_node = parse_assignment(tokens, current_token)?;
            if let Some((TokenType::SEMICOLON, _, _, _)) = tokens.get(*current_token) {
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

fn is_part_of_expression(tokens: &[(TokenType, String, usize, usize)], current_token: &mut usize) -> bool {
    if parse_expression(tokens, current_token).is_ok() {
        return true;
    }
    false
}


fn parse_int_variable_declaration(tokens: &[(TokenType, String, usize, usize)], current_token: &mut usize) -> Result<TreeNode, String> {
    let mut node = TreeNode::new(NodeType::IntStatement);

    // Parsear la palabra clave 'int'
    match_token(tokens, TokenType::INTEGER, current_token)?;

    // Parsear los identificadores
    loop {
        match tokens.get(*current_token) {
            Some((TokenType::ID, id, _, _)) => {
                node.children.push(TreeNode {
                    node_type: NodeType::Factor,
                    token: Some(TokenType::ID),
                    value: Some(id.clone()),
                    children: Vec::new(),
                });
                *current_token += 1;
                if let Some((TokenType::COMMA, _, _, _)) = tokens.get(*current_token) {
                    *current_token += 1; // Avanzar si hay una coma
                } else {
                    break; // Salir del bucle si no hay más identificadores
                }
            }
            _ => return Err(format!("Error de sintaxis: se esperaba un identificador en la posición {:?}", tokens.get(*current_token))),
        }
    }

    // Verificar si hay un punto y coma al final
    if let Some((TokenType::SEMICOLON, _, _, _)) = tokens.get(*current_token) {
        *current_token += 1; // Avanzar si hay un punto y coma
        Ok(node)
    } else {
        Err(format!("Error de sintaxis: se esperaba ';' en la posición {:?}", *current_token))
    }
}

fn parse_double_variable_declaration(tokens: &[(TokenType, String, usize, usize)], current_token: &mut usize) -> Result<TreeNode, String> {
    let mut node = TreeNode::new(NodeType::DoubleStatement);

    match_token(tokens, TokenType::DOUBLE, current_token)?;
    loop {
        match tokens.get(*current_token) {
            Some((TokenType::ID, id, _, _)) => {
                node.children.push(TreeNode {
                    node_type: NodeType::Factor,
                    token: Some(TokenType::ID),
                    value: Some(id.clone()),
                    children: Vec::new(),
                });
                *current_token += 1;
                if let Some((TokenType::COMMA, _, _, _)) = tokens.get(*current_token) {
                    *current_token += 1; // Avanzar si hay una coma
                } else {
                    break; // Salir del bucle si no hay más identificadores
                }
            }
            _ => return Err(format!("Error de sintaxis: se esperaba un identificador en la posición {:?}", tokens.get(*current_token))),
        }
    }

    // Verificar si hay un punto y coma al final
    if let Some((TokenType::SEMICOLON, _, _, _)) = tokens.get(*current_token) {
        *current_token += 1; // Avanzar si hay un punto y coma
        Ok(node)
    } else {
        Err(format!("Error de sintaxis: se esperaba ';' en la posición {:?}", *current_token))
    }
}


fn parse_if_statement(tokens: &[(TokenType, String, usize, usize)], current_token: &mut usize) -> Result<TreeNode, String> {
    let mut node = TreeNode::new(NodeType::IfStatement);
    match_token(tokens, TokenType::IF, current_token)?;
    let condition_node = parse_expression(tokens, current_token)?;
    node.children.push(condition_node);
    if  let Err(err) = match_token(tokens, TokenType::LBRACE, current_token) {
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
    if  let Err(err) = match_token(tokens, TokenType::RBRACE, current_token) {
        log_error(err.to_string());
    }
    if let Some((TokenType::ELSE, _, _, _)) = tokens.get(*current_token) {
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

fn parse_else_statement(tokens: &[(TokenType, String, usize, usize)], current_token: &mut usize) -> Result<TreeNode, String> {
    let mut node = TreeNode::new(NodeType::ElseStatement);
    match_token(tokens, TokenType::ELSE, current_token)?;
    if  let Err(err) = match_token(tokens, TokenType::LBRACE, current_token) {
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
    if  let Err(err) = match_token(tokens, TokenType::RBRACE, current_token) {
        log_error(err.to_string());
    }
    Ok(node)
}


fn parse_do_while_statement(tokens: &[(TokenType, String, usize, usize)], current_token: &mut usize) -> Result<TreeNode, String> {
    let mut node = TreeNode::new(NodeType::DoWhileStatement);
    match_token(tokens, TokenType::DO, current_token)?;
    if  let Err(err) = match_token(tokens, TokenType::LBRACE, current_token) {
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
    if  let Err(err) = match_token(tokens, TokenType::RBRACE, current_token) {
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


fn parse_while_statement(tokens: &[(TokenType, String, usize, usize)], current_token: &mut usize) -> Result<TreeNode, String> {
    let mut node = TreeNode::new(NodeType::WhileStatement);
    match_token(tokens, TokenType::WHILE, current_token)?;
    let condition_node = parse_expression(tokens, current_token)?;
    node.children.push(condition_node);
    if  let Err(err) = match_token(tokens, TokenType::LBRACE, current_token) {
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
    if  let Err(err) = match_token(tokens, TokenType::RBRACE, current_token) {
        log_error(err.to_string());
    }
    Ok(node)
}

fn parse_repeat_until_statement(tokens: &[(TokenType, String, usize, usize)], current_token: &mut usize) -> Result<TreeNode, String> {
    let mut node = TreeNode::new(NodeType::RepeatUntilStatement);
    match_token(tokens, TokenType::REPEAT, current_token)?;
    if  let Err(err) = match_token(tokens, TokenType::LBRACE, current_token) {
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
    if  let Err(err) = match_token(tokens, TokenType::RBRACE, current_token) {
        log_error(err.to_string());
    }
    if  let Err(err) = match_token(tokens, TokenType::UNTIL, current_token) {
        log_error(err.to_string());
    }
    let condition_node = parse_expression(tokens, current_token)?;
    node.children.push(condition_node);
    if let Some((TokenType::SEMICOLON, _, _, _)) = tokens.get(*current_token) {
        *current_token += 1;
    } else {
        return Err(format!("Error de sintaxis: se esperaba ';' en la posición {:?}", *current_token));
    }
    Ok(node)
}

fn parse_main_function(tokens: &[(TokenType, String, usize, usize)], current_token: &mut usize) -> Result<TreeNode, String> {
    let mut node = TreeNode::new(NodeType::MainFunction);
    match_token(tokens, TokenType::MAIN, current_token)?;
    if  let Err(err) = match_token(tokens, TokenType::LPAREN, current_token) {
        log_error(err.to_string());
    }
    if  let Err(err) = match_token(tokens, TokenType::RPAREN, current_token) {
        log_error(err.to_string());
    }
    if  let Err(err) = match_token(tokens, TokenType::LBRACE, current_token) {
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
    if  let Err(err) = match_token(tokens, TokenType::RBRACE, current_token) {
        log_error(err.to_string());
    }
    Ok(node)
}

fn parse_write_statement(tokens: &[(TokenType, String, usize, usize)], current_token: &mut usize) -> Result<TreeNode, String> {
    let mut node = TreeNode::new(NodeType::WriteStatement);
    match_token(tokens, TokenType::WRITE, current_token)?;
    if let Some((TokenType::ID, id, _, _)) = tokens.get(*current_token) {
        node.children.push(TreeNode {
            node_type: NodeType::Factor,
            token: Some(TokenType::ID),
            value: Some(id.clone()),
            children: Vec::new(),
        });
        *current_token += 1;
    } else {
        return Err(format!("Error de sintaxis: se esperaba un identificador en la posición {:?}", tokens.get(*current_token)));
    }
    if let Some((TokenType::SEMICOLON, _, _, _)) = tokens.get(*current_token) {
        *current_token += 1;
        Ok(node)
    } else {
        Err(format!("Error de sintaxis: se esperaba ';' en la posición {:?}", *current_token))
    }
}

fn parse_read_statement(tokens: &[(TokenType, String, usize, usize)], current_token: &mut usize) -> Result<TreeNode, String> {
    let mut node = TreeNode::new(NodeType::ReadStatement);
    match_token(tokens, TokenType::READ, current_token)?;
    if let Some((TokenType::ID, id, _, _)) = tokens.get(*current_token) {
        node.children.push(TreeNode {
            node_type: NodeType::Factor,
            token: Some(TokenType::ID),
            value: Some(id.clone()),
            children: Vec::new(),
        });
        *current_token += 1;
    } else {
        return Err(format!("Error de sintaxis: se esperaba un identificador en la posición {:?}", tokens.get(*current_token)));
    }
    if let Some((TokenType::SEMICOLON, _, _, _)) = tokens.get(*current_token) {
        *current_token += 1;
        Ok(node)
    } else {
        Err(format!("Error de sintaxis: se esperaba ';' en la posición {:?}", *current_token))
    }
}

fn parse_return_statement(tokens: &[(TokenType, String, usize, usize)], current_token: &mut usize) -> Result<TreeNode, String> {
    let mut node = TreeNode::new(NodeType::ReturnStatement);
    match_token(tokens, TokenType::RETURN, current_token)?;
    let expression_node = parse_expression(tokens, current_token)?;
    node.children.push(expression_node);
    if let Some((TokenType::SEMICOLON, _, _, _)) = tokens.get(*current_token) {
        *current_token += 1;
    } else {
        return Err(format!("Error de sintaxis: se esperaba ';' en la posición {:?}", *current_token));
    }
    Ok(node)
}

fn parse_cin_statement(tokens: &[(TokenType, String, usize, usize)], current_token: &mut usize) -> Result<TreeNode, String> {
    let mut node = TreeNode::new(NodeType::CinStatement);
    match_token(tokens, TokenType::CIN, current_token)?;
    if let Some((TokenType::ID, id, _, _)) = tokens.get(*current_token) {
        node.children.push(TreeNode {
            node_type: NodeType::Factor,
            token: Some(TokenType::ID),
            value: Some(id.clone()),
            children: Vec::new(),
        });
        *current_token += 1;
    } else {
        return Err(format!("Error de sintaxis: se esperaba un identificador en la posición {:?}", tokens.get(*current_token)));
    }
    if let Some((TokenType::SEMICOLON, _, _, _)) = tokens.get(*current_token) {
        *current_token += 1;
        Ok(node)
    } else {
        Err(format!("Error de sintaxis: se esperaba ';' en la posición {:?}", *current_token))
    }
}

fn parse_cout_statement(tokens: &[(TokenType, String, usize, usize)], current_token: &mut usize) -> Result<TreeNode, String> {
    let mut node = TreeNode::new(NodeType::CoutStatement);
    match_token(tokens, TokenType::COUT, current_token)?;
    let expression_node = parse_expression(tokens, current_token)?;
    node.children.push(expression_node);
    if let Some((TokenType::SEMICOLON, _, _, _)) = tokens.get(*current_token) {
        *current_token += 1;
    } else {
        return Err(format!("Error de sintaxis: se esperaba ';' en la posición {:?}", *current_token));
    }
    Ok(node)
}

fn parse_increment_statement(tokens: &[(TokenType, String, usize, usize)], current_token: &mut usize) -> Result<TreeNode, String> {
    let mut node = TreeNode::new(NodeType::Increment);
    if let Some((TokenType::ID, id, _, _)) = tokens.get(*current_token) {
        node.children.push(TreeNode {
            node_type: NodeType::Factor,
            token: Some(TokenType::ID),
            value: Some(id.clone()),
            children: Vec::new(),
        });
        *current_token += 2;
        if let Some((TokenType::SEMICOLON, _, _, _)) = tokens.get(*current_token) {
            *current_token += 1;
        } else {
            return Err(format!("Error de sintaxis: se esperaba ';' en la posición {:?}", *current_token));
        }
        Ok(node)
    } else {
        Err(format!("Error de sintaxis: se esperaba un identificador en la posición {:?}", tokens.get(*current_token)))
    }
}

fn parse_decrement_statement(tokens: &[(TokenType, String, usize, usize)], current_token: &mut usize) -> Result<TreeNode, String> {
    let mut node = TreeNode::new(NodeType::Decrement);
    if let Some((TokenType::ID, id, _, _)) = tokens.get(*current_token) {
        node.children.push(TreeNode {
            node_type: NodeType::Factor,
            token: Some(TokenType::ID),
            value: Some(id.clone()),
            children: Vec::new(),
        });
        *current_token += 2;
        if let Some((TokenType::SEMICOLON, _, _, _)) = tokens.get(*current_token) {
            *current_token += 1;
        } else {
            return Err(format!("Error de sintaxis: se esperaba ';' en la posición {:?}", *current_token));
        }
        Ok(node)
    } else {
        Err(format!("Error de sintaxis: se esperaba un identificador en la posición {:?}", tokens.get(*current_token)))
    }
}


fn parse_expression(tokens: &[(TokenType, String, usize, usize)], current_token: &mut usize) -> Result<TreeNode, String> {
    let mut node = parse_term(tokens, current_token)?;
    while let Some((token, value, _, _)) = tokens.get(*current_token) {
        match token {
            TokenType::PLUS | TokenType::MINUS | TokenType::LT | TokenType::LTE | TokenType::GT | TokenType::GTE | TokenType::EQ | TokenType::NEQ | TokenType::AND | TokenType::OR => {
                *current_token += 1;
                let term_node = parse_term(tokens, current_token)?;
                let mut expression_node = TreeNode::new(NodeType::Expression);
                expression_node.children.push(node);
                expression_node.children.push(TreeNode {
                    node_type: NodeType::Factor,
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

fn parse_term(tokens: &[(TokenType, String, usize, usize)], current_token: &mut usize) -> Result<TreeNode, String> {
    let mut node = parse_factor(tokens, current_token)?;
    while let Some((token, value, _, _)) = tokens.get(*current_token) {
        match token {
            TokenType::TIMES | TokenType::DIVIDE | TokenType::MODULO | TokenType::POWER => {
                *current_token += 1;
                let factor_node = parse_factor(tokens, current_token)?;
                let mut term_node = TreeNode::new(NodeType::Term);
                term_node.children.push(node);
                term_node.children.push(TreeNode {
                    node_type: NodeType::Factor,
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

fn parse_factor(tokens: &[(TokenType, String, usize, usize)], current_token: &mut usize) -> Result<TreeNode, String> {
    if let Some((token, value, _, _)) = tokens.get(*current_token) {
        let mut node = TreeNode::new(NodeType::Factor);
        match token {
            TokenType::NumInt | TokenType::NumReal | TokenType::ID => {
                node.token = Some(token.clone());
                node.value = Some(value.clone());
                *current_token += 1;
                Ok(node)
            }
            TokenType::LPAREN => {
                *current_token += 1;
                let expression_node = parse_expression(tokens, current_token)?;
                if  let Err(err) = match_token(tokens, TokenType::RPAREN, current_token) {
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

fn parse_assignment(tokens: &[(TokenType, String, usize, usize)], current_token: &mut usize) -> Result<TreeNode, String> {
    let mut node = TreeNode::new(NodeType::Assignment);
    if let Some((TokenType::ID, id, _, _)) = tokens.get(*current_token) {
        node.children.push(TreeNode {
            node_type: NodeType::Factor,
            token: Some(TokenType::ID),
            value: Some(id.clone()),
            children: Vec::new(),
        });
        *current_token += 1;
        if  let Err(err) = match_token(tokens, TokenType::ASSIGN, current_token) {
            log_error(err.to_string());
        }
        let expression_node = parse_expression(tokens, current_token)?;
        node.children.push(expression_node);
        Ok(node)
    } else {
        Err(format!("Error de sintaxis: se esperaba un identificador en la posición {:?}", tokens.get(*current_token)))
    }
}
