use range::Range;

pub mod range;

pub struct Symbol {
    pub name: String,
}

pub struct Ident {
    pub data: Symbol,
    pub range: Range,
}

pub enum Expr {
    Int(Range, usize),
    Str(Range, String),
    Atom(Ident),
    List(Vec<Expr>),
}

impl Symbol {
    pub fn new(name: String) -> Symbol {
        Symbol { name }
    }
}

impl Ident {
    pub fn from_string(str: String, range: Range) -> Ident {
        Ident {
            data: Symbol::new(str),
            range,
        }
    }
}
