use crate::lexer::LocatedToken as LT;
use crate::lexer::Location as L;
use crate::lexer::Span;
use crate::lexer::Token::Operator as O;
use crate::lexer::OperatorType as OT;


use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::one_of;
use nom::character::complete::digit1;
use nom::combinator::map;
use nom::combinator::not;
use nom::combinator::peek;
use nom::sequence::terminated;
use nom::error;
use nom::IResult;

pub fn operator(input: Span) -> IResult<Span, LT> {
    error::context(
        "operator",
        alt((
            alt((
                map(terminated(tag("&"), peek(not(one_of("&=")))), |m| LT::of(L::from(&m), O(OT::Amp))),
                map(tag("&&"), |m| LT::of(L::from(&m), O(OT::AmpAmp))),
                map(tag("&="), |m| LT::of(L::from(&m), O(OT::AmpEql))),
                map(terminated(tag("!"), peek(not(tag("=")))), |m| LT::of(L::from(&m), O(OT::Bang))),
                map(tag("!="), |m| LT::of(L::from(&m), O(OT::BangEql))),
                map(terminated(tag("^"), peek(not(tag("=")))), |m| LT::of(L::from(&m), O(OT::Caret))),
                map(tag("^="), |m| LT::of(L::from(&m), O(OT::CaretEql))),
                map(terminated(tag("-"), peek(not(one_of("-=>")))), |m| LT::of(L::from(&m), O(OT::Dash))),
                map(tag("--"), |m| LT::of(L::from(&m), O(OT::DashDash))),
                map(tag("-="), |m| LT::of(L::from(&m), O(OT::DashEql))),
                map(tag("->"), |m| LT::of(L::from(&m), O(OT::DashGreaterThan))),
                map(terminated(tag("."), peek(not(alt((digit1,tag(".")))))), |m| LT::of(L::from(&m), O(OT::Dot))),
                map(tag("..."), |m| LT::of(L::from(&m), O(OT::DotDotDot))),
                map(terminated(tag("="), peek(not(tag("=")))), |m| LT::of(L::from(&m), O(OT::Eql))),
                map(tag("=="), |m| LT::of(L::from(&m), O(OT::EqlEql))),
                map(terminated(tag("/"), peek(not(tag("=")))), |m| LT::of(L::from(&m), O(OT::FwdSlash))),
                map(tag("/="), |m| LT::of(L::from(&m), O(OT::FwdSlashEql))),
                map(terminated(tag(">"), peek(not(one_of(">=")))), |m| LT::of(L::from(&m), O(OT::GreaterThan))),
                map(tag(">="), |m| LT::of(L::from(&m), O(OT::GreaterThanEql))),
                map(terminated(tag(">>"), peek(not(tag("=")))), |m| LT::of(L::from(&m), O(OT::GreaterThanGreaterThan))),
                map(tag(">>="), |m| LT::of(L::from(&m), O(OT::GreaterThanGreaterThanEql))),
            )), alt((
                map(terminated(tag("<"), peek(not(one_of("<=%:")))), |m| LT::of(L::from(&m), O(OT::LessThan))),
                map(tag("<="), |m| LT::of(L::from(&m), O(OT::LessThanEql))),
                map(terminated(tag("<<"), peek(not(tag("=")))), |m| LT::of(L::from(&m), O(OT::LessThanLessThan))),
                map(tag("<<="), |m| LT::of(L::from(&m), O(OT::LessThanLessThanEql))),
                map(terminated(tag("%"), peek(not(one_of("=>")))), |m| LT::of(L::from(&m), O(OT::Percent))),
                map(tag("%="), |m| LT::of(L::from(&m), O(OT::PercentEql))),
                map(terminated(tag("|"), peek(not(one_of("|=")))), |m| LT::of(L::from(&m), O(OT::Pipe))),
                map(tag("|="), |m| LT::of(L::from(&m), O(OT::PipeEql))),
                map(tag("||"), |m| LT::of(L::from(&m), O(OT::PipePipe))),
                map(terminated(tag("+"), peek(not(one_of("+=")))), |m| LT::of(L::from(&m), O(OT::Plus))),
                map(tag("+="), |m| LT::of(L::from(&m), O(OT::PlusEql))),
                map(tag("++"), |m| LT::of(L::from(&m), O(OT::PlusPlus))),
                map(tag("?"), |m| LT::of(L::from(&m), O(OT::Question))),
                map(terminated(tag("*"), peek(not(tag("=")))), |m| LT::of(L::from(&m), O(OT::Star))),
                map(tag("*="), |m| LT::of(L::from(&m), O(OT::StarEql))),
                map(tag("~"), |m| LT::of(L::from(&m), O(OT::Tilde))),
            )),
        )),
    )(input)
}
