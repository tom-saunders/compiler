use core::fmt::Debug;

#[derive(PartialEq, Clone)]
pub enum Token {
    Amp,
    AmpAmp,
    AmpEql,
    Bang,
    BangEql,
    Caret,
    CaretEql,
    CharLit(i32),
    #[allow(non_camel_case_types)]
    CharLit_L(i32),
    #[allow(non_camel_case_types)]
    CharLit_u(i32),
    #[allow(non_camel_case_types)]
    CharLit_U(i32),
    Colon,
    Comma,
    Dash,
    DashDash,
    DashEql,
    DashGTh,
    Dot,
    Ellipsis,
    Eql,
    EqlEql,
    FSl,
    FSlEql,
    GTh,
    GThEql,
    GThGTh,
    GThGThEql,
    Identifier(String),
    KwAuto,
    KwBreak,
    KwCase,
    KwChar,
    KwConst,
    KwContinue,
    KwDefault,
    KwDo,
    KwDouble,
    KwElse,
    KwEnum,
    KwExtern,
    KwFloat,
    KwFor,
    KwGoto,
    KwIf,
    KwInline,
    KwInt,
    KwLong,
    KwRegister,
    KwRestrict,
    KwReturn,
    KwShort,
    KwSigned,
    KwSizeof,
    KwStatic,
    KwStruct,
    KwSwitch,
    KwTypedef,
    KwUnion,
    KwUnsigned,
    KwVoid,
    KwWhile,
    #[allow(non_camel_case_types)]
    Kw_Alignas,
    #[allow(non_camel_case_types)]
    Kw_Alignof,
    #[allow(non_camel_case_types)]
    Kw_Atomic,
    #[allow(non_camel_case_types)]
    Kw_Bool,
    #[allow(non_camel_case_types)]
    Kw_Complex,
    #[allow(non_camel_case_types)]
    Kw_Generic,
    #[allow(non_camel_case_types)]
    Kw_Imaginary,
    #[allow(non_camel_case_types)]
    Kw_Noreturn,
    #[allow(non_camel_case_types)]
    Kw_Static_assert,
    #[allow(non_camel_case_types)]
    Kw_Thread_local,
    LBrace,
    LSquare,
    LParen,
    LTh,
    LThEql,
    LThLTh,
    LThLThEql,
    Pct,
    PctEql,
    Pipe,
    PipeEql,
    PipePipe,
    Plus,
    PlusEql,
    PlusPlus,
    Question,
    RBrace,
    RParen,
    RSquare,
    Semi,
    Star,
    StarEql,
    StringLit(Vec<i8>),
    #[allow(non_camel_case_types)]
    StringLit_L(Vec<i32>),
    #[allow(non_camel_case_types)]
    StringLit_u(Vec<i16>),
    #[allow(non_camel_case_types)]
    StringLit_u8(Vec<i8>),
    #[allow(non_camel_case_types)]
    StringLit_U(Vec<i32>),
    Tilde,
    Unknown(String),
}
struct IntDebug<I> {
    i: I,
}

impl Debug for IntDebug<i8> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#04x}", self.i)
    }
}

impl Debug for IntDebug<i16> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#06x}", self.i)
    }
}

impl Debug for IntDebug<i32> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#010x}", self.i)
    }
}

impl Debug for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Amp => write!(f, "Amp"),
            Self::AmpAmp => write!(f, "AmpAmp"),
            Self::AmpEql => write!(f, "AmpEql"),
            Self::Bang => write!(f, "Bang"),
            Self::BangEql => write!(f, "BangEql"),
            Self::Caret => write!(f, "Caret"),
            Self::CaretEql => write!(f, "CaretEql"),
            Self::CharLit(i) => {
                let o: IntDebug<i32> = IntDebug{i: *i};
                f.debug_tuple("CharLit").field(&o).finish()
            }
            Self::CharLit_L(i) => {
                let o: IntDebug<i32> = IntDebug{i: *i};
                f.debug_tuple("CharLit_L").field(&o).finish()
            }
            Self::CharLit_u(i) => {
                let o: IntDebug<i32> = IntDebug{i: *i};
                f.debug_tuple("CharLit_u").field(&o).finish()
            }
            Self::CharLit_U(i) => {
                let o: IntDebug<i32> = IntDebug{i: *i};
                f.debug_tuple("CharLit_U").field(&o).finish()
            }
            Self::Colon => write!(f, "Colon"),
            Self::Comma => write!(f, "Comma"),
            Self::Dash => write!(f, "Dash"),
            Self::DashDash => write!(f, "DashDash"),
            Self::DashEql => write!(f, "DashEql"),
            Self::DashGTh => write!(f, "DashGTh"),
            Self::Dot => write!(f, "Dot"),
            Self::Ellipsis => write!(f, "Ellipsis"),
            Self::Eql => write!(f, "Eql"),
            Self::EqlEql => write!(f, "EqlEql"),
            Self::FSl => write!(f, "FSl"),
            Self::FSlEql => write!(f, "FSlEql"),
            Self::GTh => write!(f, "GTh"),
            Self::GThEql => write!(f, "GThEql"),
            Self::GThGTh => write!(f, "GThGTh"),
            Self::GThGThEql => write!(f, "GThGThEql"),
            Self::Identifier(arg0) => f.debug_tuple("Identifier").field(arg0).finish(),
            Self::KwAuto => write!(f, "KwAuto"),
            Self::KwBreak => write!(f, "KwBreak"),
            Self::KwCase => write!(f, "KwCase"),
            Self::KwChar => write!(f, "KwChar"),
            Self::KwConst => write!(f, "KwConst"),
            Self::KwContinue => write!(f, "KwContinue"),
            Self::KwDefault => write!(f, "KwDefault"),
            Self::KwDo => write!(f, "KwDo"),
            Self::KwDouble => write!(f, "KwDouble"),
            Self::KwElse => write!(f, "KwElse"),
            Self::KwEnum => write!(f, "KwEnum"),
            Self::KwExtern => write!(f, "KwExtern"),
            Self::KwFloat => write!(f, "KwFloat"),
            Self::KwFor => write!(f, "KwFor"),
            Self::KwGoto => write!(f, "KwGoto"),
            Self::KwIf => write!(f, "KwIf"),
            Self::KwInline => write!(f, "KwInline"),
            Self::KwInt => write!(f, "KwInt"),
            Self::KwLong => write!(f, "KwLong"),
            Self::KwRegister => write!(f, "KwRegister"),
            Self::KwRestrict => write!(f, "KwRestrict"),
            Self::KwReturn => write!(f, "KwReturn"),
            Self::KwShort => write!(f, "KwShort"),
            Self::KwSigned => write!(f, "KwSigned"),
            Self::KwSizeof => write!(f, "KwSizeof"),
            Self::KwStatic => write!(f, "KwStatic"),
            Self::KwStruct => write!(f, "KwStruct"),
            Self::KwSwitch => write!(f, "KwSwitch"),
            Self::KwTypedef => write!(f, "KwTypedef"),
            Self::KwUnion => write!(f, "KwUnion"),
            Self::KwUnsigned => write!(f, "KwUnsigned"),
            Self::KwVoid => write!(f, "KwVoid"),
            Self::KwWhile => write!(f, "KwWhile"),
            Self::Kw_Alignas => write!(f, "Kw_Alignas"),
            Self::Kw_Alignof => write!(f, "Kw_Alignof"),
            Self::Kw_Atomic => write!(f, "Kw_Atomic"),
            Self::Kw_Bool => write!(f, "Kw_Bool"),
            Self::Kw_Complex => write!(f, "Kw_Complex"),
            Self::Kw_Generic => write!(f, "Kw_Generic"),
            Self::Kw_Imaginary => write!(f, "Kw_Imaginary"),
            Self::Kw_Noreturn => write!(f, "Kw_Noreturn"),
            Self::Kw_Static_assert => write!(f, "Kw_Static_assert"),
            Self::Kw_Thread_local => write!(f, "Kw_Thread_local"),
            Self::LBrace => write!(f, "LBrace"),
            Self::LSquare => write!(f, "LSquare"),
            Self::LParen => write!(f, "LParen"),
            Self::LTh => write!(f, "LTh"),
            Self::LThEql => write!(f, "LThEql"),
            Self::LThLTh => write!(f, "LThLTh"),
            Self::LThLThEql => write!(f, "LThLThEql"),
            Self::Pct => write!(f, "Pct"),
            Self::PctEql => write!(f, "PctEql"),
            Self::Pipe => write!(f, "Pipe"),
            Self::PipeEql => write!(f, "PipeEql"),
            Self::PipePipe => write!(f, "PipePipe"),
            Self::Plus => write!(f, "Plus"),
            Self::PlusEql => write!(f, "PlusEql"),
            Self::PlusPlus => write!(f, "PlusPlus"),
            Self::Question => write!(f, "Question"),
            Self::RBrace => write!(f, "RBrace"),
            Self::RParen => write!(f, "RParen"),
            Self::RSquare => write!(f, "RSquare"),
            Self::Semi => write!(f, "Semi"),
            Self::Star => write!(f, "Star"),
            Self::StarEql => write!(f, "StarEql"),
            Self::StringLit(v) => {
                let o: Vec<IntDebug<i8>> = v.iter().map(|i| IntDebug{i: *i}).collect();
                f.debug_tuple("StringLit").field(&o).finish()
            }
            Self::StringLit_L(v) => {
                let o: Vec<IntDebug<i32>> = v.iter().map(|i| IntDebug{i: *i}).collect();
                f.debug_tuple("StringLit_L").field(&o).finish()
            }
            Self::StringLit_u(v) => {
                let o: Vec<IntDebug<i16>> = v.iter().map(|i| IntDebug{i: *i}).collect();
                f.debug_tuple("StringLit_u").field(&o).finish()
            }
            Self::StringLit_u8(v) => {
                let o: Vec<IntDebug<i8>> = v.iter().map(|i| IntDebug{i: *i}).collect();
                f.debug_tuple("StringLit_u8").field(&o).finish()
            }
            Self::StringLit_U(v) => {
                let o: Vec<IntDebug<i32>> = v.iter().map(|i| IntDebug{i: *i}).collect();
                f.debug_tuple("StringLit_U").field(&o).finish()
            }
            Self::Tilde => write!(f, "Tilde"),
            Self::Unknown(arg0) => f.debug_tuple("Unknown").field(arg0).finish(),
        }
    }
}
