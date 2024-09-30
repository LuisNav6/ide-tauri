use serde::{Serialize, Deserialize};
// Enum para representar los tipos de tokens
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum TokenType {
    // Tokens de control
    ENDFILE, //cierre de archivo
    ERROR, //error - no hay tokens que coincidan

    // Palabras reservadas
    IF,
    ELSE,
    DO,
    WHILE,
    REPEAT,
    UNTIL,
    READ,
    WRITE,
    INTEGER,
    DOUBLE,
    MAIN,
    AND,
    OR,
    RETURN,
    CIN,
    COUT,

    // Tokens de múltiples caracteres
    ID,
    NumInt,
    NumReal,

    // Operadores aritméticos
    PLUS,
    MINUS,
    TIMES,
    DIVIDE,
    MODULO,
    POWER,

    // Operadores relacionales
    EQ,   // igualdad
    NEQ,  // diferente
    LT,   // menor que
    LTE,  // menor o igual que
    GT,   // mayor que
    GTE,  // mayor o igual que

    // Símbolos especiales
    LPAREN,    // paréntesis izquierdo
    RPAREN,    // paréntesis derecho
    LBRACE,    // llave izquierda
    RBRACE,    // llave derecha
    COMMA,     // coma
    COLON,     // dos puntos
    SEMICOLON, // punto y coma
    ASSIGN,    // asignación

    //Incrementador
    INCREMENT,
    
    //Decrementador
    DECREMENT,

    // Símbolo de comentario múltiple no cerrado
    InMultipleComment,
}

// Enum para representar los estados en el DFA del escáner
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum StateType {
    Start,
    InAssign,
    InComment,
    InMultiComment,
    InNum,
    InReal,
    InId,
    Done,
    EndFile,
}
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum NodeType {
    MainRoot,
    IntStatement,
    DoubleStatement,
    Statement,
    Expression,
    Term,
    Factor,
    Assignment,
    IfStatement,
    ElseStatement,
    WhileStatement,
    WriteStatement,
    ReadStatement,
    DoWhileStatement,
    RepeatUntilStatement,
    SwitchStatement,
    CaseStatement,
    DefaultStatement,
    MainFunction,
    ReturnStatement,
    CinStatement,
    CoutStatement,
    Increment,
    Decrement,
    Error
}
