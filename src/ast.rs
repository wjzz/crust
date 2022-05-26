pub type Type = String; // TODO: change to enum later

pub type Name = String;

#[derive(Debug)]
pub enum CExpr {
    Number(u64),
}

#[derive(Debug)]
pub enum CStm {
    Return(CExpr),
}

pub type CBlock = Vec<CStm>;

// Top level declarations

#[derive(Debug)]
pub enum CDecl {
    Fun { return_tp: Type, name: Name, parameters: Vec<(Type, Name)>, body: CBlock }, // function definition
    // Var { tp: Type, name: Name, rhs: CExpr }, // variable declaration
    // TODO: add struct
    // TODO: add typedef
    // TODO: add const
}

pub type CProgram = Vec<CDecl>;
