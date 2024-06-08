use crate::lexer::KeywordType as KT;
use crate::lexer::LocatedToken as LT;
use crate::lexer::Location as L;
use crate::lexer::Span;
use crate::lexer::Token::Keyword as K;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::all_consuming;
use nom::combinator::map;
use nom::error;
use nom::IResult;


macro_rules! kw_map {
    ($t: literal, $kt: expr) => {
        map(all_consuming(tag($t)), |m| LT::of(L::from(&m), K($kt)))
    };
}
pub fn keyword(input: Span) -> IResult<Span, LT> {
    error::context(
        "keyword",
        alt((
            alt((
                kw_map!("auto", KT::Auto),
                kw_map!("break", KT::Break),
                kw_map!("case", KT::Case),
                kw_map!("char", KT::Char),
                kw_map!("const", KT::Const),
                kw_map!("continue", KT::Continue),
                kw_map!("default", KT::Default),
                kw_map!("do", KT::Do),
                kw_map!("double", KT::Double),
                kw_map!("else", KT::Else),
                kw_map!("enum", KT::Enum),
                kw_map!("extern", KT::Extern),
                kw_map!("float", KT::Float),
                kw_map!("for", KT::For),
                kw_map!("goto", KT::Goto),
                kw_map!("if", KT::If),
                kw_map!("inline", KT::Inline),
                kw_map!("int", KT::Int),
                kw_map!("long", KT::Long),
                kw_map!("register", KT::Register),
            )),
            alt((
                kw_map!("restrict", KT::Restrict),
                kw_map!("return", KT::Return),
                kw_map!("short", KT::Short),
                kw_map!("signed", KT::Signed),
                kw_map!("sizeof", KT::Sizeof),
                kw_map!("static", KT::Static),
                kw_map!("struct", KT::Struct),
                kw_map!("switch", KT::Switch),
                kw_map!("typedef", KT::Typedef),
                kw_map!("union", KT::Union),
                kw_map!("unsigned", KT::Unsigned),
                kw_map!("void", KT::Void),
                kw_map!("while", KT::While),
                kw_map!("_Alignas", KT::_Alignas),
                kw_map!("_Alignof", KT::_Alignof),
                kw_map!("_Atomic", KT::_Atomic),
                kw_map!("_Bool", KT::_Bool),
                kw_map!("_Complex", KT::_Complex),
                kw_map!("_Generic", KT::_Generic),
                kw_map!("_Imaginary", KT::_Imaginary),
            )),
            alt((
                kw_map!("_Noreturn", KT::_Noreturn),
                kw_map!("_Static_assert", KT::_StaticAssert),
                kw_map!("_Thread_local", KT::_ThreadLocal),
            )),
        )),
    )(input)
}