use lazy_static::lazy_static;
use regex::Regex;
use TokKind::*;

#[derive(Debug, PartialEq, Eq)]
pub struct Tok {
    kind: TokKind,
    pos: usize,
    str: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokKind {
    As,
    LPar,
    RPar,
    LBrc,
    RBrc,
    SCol,
    Add,
    Sub,
    Inc,
    Dec,
    Key,
    Var,
    Lit,
    Cmt,
    Spc,
    Nl,
    CR,
    Tab,
}

lazy_static! {
    static ref KEYWORDS: Vec<String> = ["char", "int"].iter().map(|s| s.to_string()).collect();
    static ref REGEXES: Vec<(Regex, TokKind)> = [
            (r"=", As),
            (r"\(", LPar),
            (r"\)", RPar),
            (r"\{", LBrc),
            (r"\}", RBrc),
            (r";", SCol),
            (r"\+", Add),
            (r"-", Sub),
            (r"\+\+", Inc),
            (r"--", Dec),
            (&KEYWORDS.iter().map(|s| s.to_owned()).reduce(|acc: String, key: String| acc + "|" + &key).unwrap() , Key),
            (r"[a-zA-Z_]\w*", Var), // NOTE: Var MUST come after Key, otherwise keywords would be matched as variables
            (concat!(
                r#""(\\.|[^\\"])*?""#, // string literal
                "|",
                r"('[^\']?')|('\\.+?')", // char literal
                "|",
                r"\d+" // int literal
            ), Lit),
            (concat!(
                r"//.*", // single line comment
                "|",
                r"\/\*(.|[\r\n])*?\*\/" // multiline comment
            ), Cmt),
            (r" +", Spc),
            (r"\n", Nl),
            (r"\r", CR),
            (r"\t", Tab),
        ].iter().map(|(s, t)| (Regex::new(s).unwrap(), t.clone())).collect();
}

fn longest_match(str: &str, ind: usize) -> Option<(Tok, usize)> {
    let mut max: Option<(Tok, usize)> = None;
    for (rgx, tk_k) in REGEXES.iter() {
        let find = rgx.find_at(str, ind);
        if find.is_none() {
            continue;
        }
        let find = find.unwrap();
        if find.start() != ind {
            continue;
        }
        if max.is_none() || find.len() > max.as_ref().unwrap().1 {
            max = Some((
                Tok {
                    pos: find.start(),
                    str: find.as_str().to_owned(),
                    kind: tk_k.clone(),
                },
                find.len(),
            ));
        }
    }
    max
}

pub fn tokenize(src: &str) -> Result<Vec<Tok>, String> {
    let mut ind = 0;
    let mut out = Vec::new();
    loop {
        if ind == src.len() {
            return Ok(out);
        }
        let find = longest_match(src, ind);
        if find.is_none() {
            return Err("L + Bozo".to_string());
        }
        let find = find.unwrap();
        out.push(find.0);
        ind += find.1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_lexer_test() -> Result<(), String> {
        let src = "char c = 3;";
        let actual = tokenize(src)?;
        let expected = vec![
            Tok {
                pos: 0,
                str: "char".to_string(),
                kind: Key,
            },
            Tok {
                pos: 4,
                str: " ".to_string(),
                kind: Spc,
            },
            Tok {
                pos: 5,
                str: "c".to_string(),
                kind: Var,
            },
            Tok {
                pos: 6,
                str: " ".to_string(),
                kind: Spc,
            },
            Tok {
                pos: 7,
                str: "=".to_string(),
                kind: As,
            },
            Tok {
                pos: 8,
                str: " ".to_string(),
                kind: Spc,
            },
            Tok {
                pos: 9,
                str: "3".to_string(),
                kind: Lit,
            },
            Tok {
                pos: 10,
                str: ";".to_string(),
                kind: SCol,
            },
        ];
        assert_eq!(actual, expected);
        Ok(())
    }
}
