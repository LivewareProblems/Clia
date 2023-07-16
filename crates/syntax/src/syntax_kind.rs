#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[allow(non_camel_case_types)]
#[repr(u16)]
pub enum SyntaxKind {
    BINARY_OP, // '+'
    ERROR,
    INT_NUMBER,
    // composite nodes
    // Tokens
    PLUS,
    MINUS,
    SOURCE, // top-level node: a list of s-expressions
}
use SyntaxKind::*;

impl SyntaxKind {
    pub fn is_punct(self) -> bool {
        matches!(self, PLUS | MINUS)
    }

    pub fn is_literal(self) -> bool {
        matches!(self, INT_NUMBER)
    }

    pub fn is_token(self) -> bool {
        self.is_literal() || self.is_punct()
    }

    pub fn from_char(c: char) -> Option<SyntaxKind> {
        let tok = match c {
            '+' => PLUS,
            '-' => MINUS,
            _ => return None,
        };
        Some(tok)
    }

    pub fn from_ts_kind(ts_kind: &str) -> Option<SyntaxKind> {
        match ts_kind {
            "source_file" => Some(SOURCE),
            "binary_op" => Some(BINARY_OP),
            "integer" => Some(INT_NUMBER),
            "+" => Some(PLUS),
            _ => todo!(),
        }
    }
}
/// Some boilerplate is needed, as rowan settled on using its own
/// `struct SyntaxKind(u16)` internally, instead of accepting the
/// user's `enum SyntaxKind` as a type parameter.
///
/// First, to easily pass the enum variants into rowan via `.into()`:
impl From<SyntaxKind> for rowan::SyntaxKind {
    fn from(kind: SyntaxKind) -> Self {
        Self(kind as u16)
    }
}

/// Second, implementing the `Language` trait teaches rowan to convert between
/// these two SyntaxKind types, allowing for a nicer SyntaxNode API where
/// "kinds" are values from our `enum SyntaxKind`, instead of plain u16 values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CliaLang {}
impl rowan::Language for CliaLang {
    type Kind = SyntaxKind;
    fn kind_from_raw(raw: rowan::SyntaxKind) -> Self::Kind {
        assert!(raw.0 <= SOURCE as u16);
        unsafe { std::mem::transmute::<u16, SyntaxKind>(raw.0) }
    }
    fn kind_to_raw(kind: Self::Kind) -> rowan::SyntaxKind {
        kind.into()
    }
}
