#[derive(Debug, PartialEq, Clone)]
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