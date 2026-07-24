//! The S-expression surface: a hand-rolled reader (no dependencies), a
//! canonical printer, and the compiler from generic S-expressions to the
//! typed AST. This is the *data* surface — templates, goldens, downstream
//! markers; the typed builders in [`crate::ast`] are the *code* surface.
//! One AST underneath, so the two cannot drift.
//!
//! # Vocabulary (the format spec)
//!
//! ```text
//! CLAUSE := (clause SUBJ CORE KEY*)
//!           KEY: :tense present|past|future   :neg   :prodrop
//!                :mood cond                   :voice passive
//!                :force li|či|intonation|imp  :addressee 2sg|1pl|2pl
//!                :conj i|ili|a|ale            (verbal coordination)
//!                :topic subj|obj              :focus subj|obj
//!                :pred-case ins               (copular clauses)
//! CORE   := VP+ | (pred NP | (adj L) | (part L))
//! SUBJ   := NOMINAL
//! NOMINAL:= NP | PRON | NAME | (coord CONJ NOMINAL NOMINAL+)
//! NP     := (np [:case CASE] [:entity ID] [(det L)] [(num N)]
//!               (adj L)* (n L) [REL])
//! REL    := (rel :gap subj|obj|pp [(prep L)] [:case CASE]
//!                [SUBJ-NOMINAL] (vp …) [:tense …] [:neg]
//!                [:relativizer iže])
//! PRON   := (pron :1|:2|:3 :sg|:pl :m|:f|:n [:clitic])
//! NAME   := (name Word :m|:f|:n [:indecl])
//! VP     := (vp (v L…) [(adv L)]* [OBJ: NOMINAL] PP*)   ; "v myti sę" = reflexive
//! PP     := (pp (prep L) [:case CASE] NOMINAL)
//! CASE   := nom|acc|gen|loc|dat|ins
//! CONJ   := i|ili|a|ale
//! ```
//!
//! Leaves are flavored citation forms; integers are counts rendered as
//! digits.

use crate::ast::*;
use interslavic::{Case, Gender, Number, Person};
use std::fmt;

/// A parse or compile error with the byte offset it refers to.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SexprError {
    pub at: usize,
    pub msg: String,
}

impl fmt::Display for SexprError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "at byte {}: {}", self.at, self.msg)
    }
}
impl std::error::Error for SexprError {}

fn err<T>(at: usize, msg: impl Into<String>) -> Result<T, SexprError> {
    Err(SexprError {
        at,
        msg: msg.into(),
    })
}

/// Generic S-expression values, spans attached.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value {
    Sym(String, usize),
    Int(u64, usize),
    Key(String, usize),
    List(Vec<Value>, usize),
}

impl Value {
    fn at(&self) -> usize {
        match self {
            Value::Sym(_, at) | Value::Int(_, at) | Value::Key(_, at) | Value::List(_, at) => *at,
        }
    }
}

/// Parse exactly one toplevel form. Never panics; malformed input returns
/// a spanned error.
pub fn parse(input: &str) -> Result<Value, SexprError> {
    let mut tokens = tokenize(input)?;
    tokens.reverse();
    let value = parse_value(&mut tokens)?;
    if let Some(extra) = tokens.pop() {
        return err(extra.at(), "trailing input after the toplevel form");
    }
    Ok(value)
}

fn tokenize(input: &str) -> Result<Vec<Value>, SexprError> {
    let mut out = Vec::new();
    let mut chars = input.char_indices().peekable();
    while let Some(&(at, c)) = chars.peek() {
        if c.is_whitespace() {
            chars.next();
        } else if c == '(' || c == ')' {
            chars.next();
            out.push(Value::Sym(c.to_string(), at));
        } else {
            let mut atom = String::new();
            while let Some(&(_, c)) = chars.peek() {
                if c.is_whitespace() || c == '(' || c == ')' {
                    break;
                }
                atom.push(c);
                chars.next();
            }
            out.push(if let Some(key) = atom.strip_prefix(':') {
                if key.is_empty() {
                    return err(at, "bare `:` is not a keyword");
                }
                Value::Key(key.to_string(), at)
            } else if atom.chars().all(|c| c.is_ascii_digit()) {
                Value::Int(
                    atom.parse().map_err(|_| SexprError {
                        at,
                        msg: format!("integer out of range: {atom}"),
                    })?,
                    at,
                )
            } else {
                Value::Sym(atom, at)
            });
        }
    }
    Ok(out)
}

fn parse_value(tokens: &mut Vec<Value>) -> Result<Value, SexprError> {
    match tokens.pop() {
        None => err(0, "unexpected end of input"),
        Some(Value::Sym(s, at)) if s == "(" => {
            let mut items = Vec::new();
            loop {
                match tokens.last() {
                    None => return err(at, "unclosed `(`"),
                    Some(Value::Sym(s, _)) if s == ")" => {
                        tokens.pop();
                        return Ok(Value::List(items, at));
                    }
                    _ => items.push(parse_value(tokens)?),
                }
            }
        }
        Some(Value::Sym(s, at)) if s == ")" => err(at, "unexpected `)`"),
        Some(token) => Ok(token),
    }
}

// ---------------------------------------------------------------------------
// Compiling generic values into the typed AST.
// ---------------------------------------------------------------------------

fn case_of(text: &str, at: usize) -> Result<Case, SexprError> {
    Ok(match text {
        "nom" => Case::Nom,
        "acc" => Case::Acc,
        "gen" => Case::Gen,
        "loc" => Case::Loc,
        "dat" => Case::Dat,
        "ins" => Case::Ins,
        _ => return err(at, format!("unknown case `{text}`")),
    })
}

fn case_name(case: Case) -> &'static str {
    match case {
        Case::Nom => "nom",
        Case::Acc => "acc",
        Case::Gen => "gen",
        Case::Loc => "loc",
        Case::Dat => "dat",
        Case::Ins => "ins",
    }
}

fn conj_of(text: &str, at: usize) -> Result<Conj, SexprError> {
    Ok(match text {
        "i" => Conj::I,
        "ili" => Conj::Ili,
        "a" => Conj::A,
        "ale" => Conj::Ale,
        _ => return err(at, format!("unknown conjunction `{text}`")),
    })
}

fn head_of(list: &[Value], at: usize) -> Result<(&str, usize), SexprError> {
    match list.first() {
        Some(Value::Sym(s, at)) => Ok((s.as_str(), *at)),
        _ => err(at, "expected a list starting with a symbol"),
    }
}

fn sym_arg(list: &[Value], head: &str, at: usize) -> Result<String, SexprError> {
    match list {
        [_, Value::Sym(s, _)] => Ok(s.clone()),
        _ => err(at, format!("`({head} …)` takes exactly one symbol")),
    }
}

fn key_sym<'a>(
    rest: &mut std::iter::Peekable<std::slice::Iter<'a, Value>>,
    key_at: usize,
    what: &str,
) -> Result<(&'a str, usize), SexprError> {
    match rest.next() {
        Some(Value::Sym(s, at)) => Ok((s.as_str(), *at)),
        _ => err(key_at, format!("{what} needs a symbol value")),
    }
}

fn mark_once(seen_at: &mut Option<usize>, at: usize, what: &str) -> Result<(), SexprError> {
    if seen_at.replace(at).is_some() {
        return err(at, format!("duplicate {what}"));
    }
    Ok(())
}

pub fn compile_clause(value: &Value) -> Result<Clause, SexprError> {
    let Value::List(items, at) = value else {
        return err(value.at(), "expected `(clause …)`");
    };
    let (head, _) = head_of(items, *at)?;
    if head != "clause" {
        return err(*at, format!("expected `clause`, found `{head}`"));
    }
    let mut subject = None;
    let mut vps: Vec<VerbPhrase> = Vec::new();
    let mut predicate: Option<Predicate> = None;
    let mut pred_case = PredCase::default();
    let mut pred_case_at = None;
    let mut conjunction = Conj::I;
    let mut conjunction_at = None;
    let mut tense = TenseSpec::Present;
    let mut tense_at = None;
    let mut polarity = Polarity::Affirmative;
    let mut polarity_at = None;
    let mut force = Force::Declarative;
    let mut force_at = None;
    let mut addressee = Addressee::default();
    let mut addressee_at = None;
    let mut force_imperative = false;
    let mut mood = Mood::default();
    let mut mood_at = None;
    let mut voice = Voice::default();
    let mut voice_at = None;
    let mut prodrop = false;
    let mut prodrop_at = None;
    let mut topic = None;
    let mut topic_at = None;
    let mut focus = None;
    let mut focus_at = None;

    let mut rest = items[1..].iter().peekable();
    while let Some(item) = rest.next() {
        match item {
            Value::Key(key, key_at) => match key.as_str() {
                "tense" => {
                    mark_once(&mut tense_at, *key_at, "`:tense`")?;
                    let (s, s_at) = key_sym(&mut rest, *key_at, ":tense")?;
                    tense = tense_of(s, s_at)?;
                }
                "force" => {
                    mark_once(&mut force_at, *key_at, "`:force`")?;
                    let (s, s_at) = key_sym(&mut rest, *key_at, ":force")?;
                    match s {
                        "li" => force = Force::LiQuestion,
                        "či" => force = Force::CiQuestion,
                        "intonation" => force = Force::IntonationQuestion,
                        "imp" | "imperative" => force_imperative = true,
                        other => return err(s_at, format!("unknown force `{other}`")),
                    }
                }
                "addressee" => {
                    mark_once(&mut addressee_at, *key_at, "`:addressee`")?;
                    let (s, s_at) = key_sym(&mut rest, *key_at, ":addressee")?;
                    addressee = match s {
                        "2sg" => Addressee::You,
                        "1pl" => Addressee::We,
                        "2pl" => Addressee::YouAll,
                        other => return err(s_at, format!("unknown addressee `{other}`")),
                    };
                }
                "mood" => {
                    mark_once(&mut mood_at, *key_at, "`:mood`")?;
                    let (s, s_at) = key_sym(&mut rest, *key_at, ":mood")?;
                    mood = match s {
                        "cond" | "conditional" => Mood::Conditional,
                        other => return err(s_at, format!("unknown mood `{other}`")),
                    };
                }
                "voice" => {
                    mark_once(&mut voice_at, *key_at, "`:voice`")?;
                    let (s, s_at) = key_sym(&mut rest, *key_at, ":voice")?;
                    voice = match s {
                        "passive" => Voice::Passive,
                        other => return err(s_at, format!("unknown voice `{other}`")),
                    };
                }
                "conj" => {
                    mark_once(&mut conjunction_at, *key_at, "`:conj`")?;
                    let (s, s_at) = key_sym(&mut rest, *key_at, ":conj")?;
                    conjunction = conj_of(s, s_at)?;
                }
                "pred-case" => {
                    mark_once(&mut pred_case_at, *key_at, "`:pred-case`")?;
                    let (s, s_at) = key_sym(&mut rest, *key_at, ":pred-case")?;
                    pred_case = match s {
                        "nom" => PredCase::Nominative,
                        "ins" => PredCase::Instrumental,
                        other => return err(s_at, format!("unknown pred-case `{other}`")),
                    };
                }
                "topic" => {
                    mark_once(&mut topic_at, *key_at, "`:topic`")?;
                    let (s, s_at) = key_sym(&mut rest, *key_at, ":topic")?;
                    topic = Some(slot_ref_of(s, s_at)?);
                }
                "focus" => {
                    mark_once(&mut focus_at, *key_at, "`:focus`")?;
                    let (s, s_at) = key_sym(&mut rest, *key_at, ":focus")?;
                    focus = Some(slot_ref_of(s, s_at)?);
                }
                "neg" => {
                    mark_once(&mut polarity_at, *key_at, "`:neg`")?;
                    polarity = Polarity::Negative;
                }
                "prodrop" => {
                    mark_once(&mut prodrop_at, *key_at, "`:prodrop`")?;
                    prodrop = true;
                }
                other => return err(*key_at, format!("unknown clause key `:{other}`")),
            },
            Value::List(child, child_at) => match head_of(child, *child_at)? {
                ("vp", _) => vps.push(compile_vp(child, *child_at)?),
                ("pred", _) => {
                    if predicate.is_some() {
                        return err(*child_at, "`clause` takes at most one `(pred …)`");
                    }
                    predicate = Some(compile_pred(child, *child_at)?);
                }
                ("np" | "pron" | "name" | "coord", _) => {
                    if subject.is_some() {
                        return err(*child_at, "clause already has a subject");
                    }
                    subject = Some(compile_nominal(item)?);
                }
                (other, other_at) => {
                    return err(other_at, format!("unknown clause child `{other}`"));
                }
            },
            other => return err(other.at(), "unexpected atom inside `(clause …)`"),
        }
    }

    if let Some(addressee_at) = addressee_at
        && !force_imperative
    {
        return err(addressee_at, "`:addressee` requires `:force imp`");
    }
    if force_imperative {
        force = Force::Imperative(addressee);
    }
    let Some(subject) = subject else {
        return err(*at, "`clause` needs a subject nominal");
    };
    let core = match (predicate, vps.is_empty()) {
        (Some(predicate), true) => {
            if let Some(conjunction_at) = conjunction_at {
                return err(conjunction_at, "`:conj` requires a verbal clause");
            }
            // `:pred-case` is a nominal-predicate option. Reject it on
            // adjectival or participial predicates instead of accepting
            // and canonicalizing the option away.
            if let Some(pred_case_at) = pred_case_at
                && !matches!(predicate, Predicate::Nominal(_))
            {
                return err(pred_case_at, "`:pred-case` needs a nominal `(pred (np …))`");
            }
            ClauseCore::Copular {
                predicate,
                pred_case,
            }
        }
        (None, false) => {
            if let Some(pred_case_at) = pred_case_at {
                return err(pred_case_at, "`:pred-case` needs a nominal `(pred (np …))`");
            }
            ClauseCore::Verbal(Coordination {
                conjunction,
                items: vps,
            })
        }
        (Some(_), false) => return err(*at, "`clause` cannot have both vp and pred"),
        (None, true) => return err(*at, "`clause` needs a `(vp …)` or `(pred …)`"),
    };
    Ok(Clause {
        subject,
        core,
        tense,
        polarity,
        force,
        mood,
        voice,
        prodrop,
        topic,
        focus,
    })
}

fn tense_of(text: &str, at: usize) -> Result<TenseSpec, SexprError> {
    Ok(match text {
        "present" => TenseSpec::Present,
        "past" => TenseSpec::Past,
        "future" => TenseSpec::Future,
        other => return err(at, format!("unknown tense `{other}`")),
    })
}

fn slot_ref_of(text: &str, at: usize) -> Result<SlotRef, SexprError> {
    Ok(match text {
        "subj" => SlotRef::Subject,
        "obj" => SlotRef::Object,
        _ => return err(at, format!("unknown slot `{text}` (subj|obj)")),
    })
}

fn compile_pred(items: &[Value], at: usize) -> Result<Predicate, SexprError> {
    match &items[1..] {
        [Value::List(child, child_at)] => match head_of(child, *child_at)? {
            ("np", _) => Ok(Predicate::Nominal(compile_np(child, *child_at)?)),
            ("adj", _) => Ok(Predicate::Adjectival(sym_arg(child, "adj", *child_at)?)),
            ("part", _) => Ok(Predicate::Participial(sym_arg(child, "part", *child_at)?)),
            (other, other_at) => err(other_at, format!("unknown predicate `{other}`")),
        },
        _ => err(
            at,
            "`(pred …)` takes exactly one of (np …)/(adj …)/(part …)",
        ),
    }
}

fn compile_nominal(value: &Value) -> Result<Nominal, SexprError> {
    let Value::List(items, at) = value else {
        return err(value.at(), "expected a nominal");
    };
    match head_of(items, *at)? {
        ("np", _) => compile_np(items, *at).map(Nominal::Np),
        ("pron", _) => compile_pron(items, *at),
        ("name", _) => compile_name(items, *at),
        ("coord", _) => compile_coord(items, *at),
        (other, other_at) => err(other_at, format!("expected a nominal, found `{other}`")),
    }
}

fn compile_np(items: &[Value], at: usize) -> Result<NounPhrase, SexprError> {
    let mut np = NounPhrase::new("");
    let mut head_seen = false;
    let mut case_at = None;
    let mut entity_at = None;
    let mut rest = items[1..].iter().peekable();
    while let Some(item) = rest.next() {
        match item {
            Value::Key(key, key_at) if key == "case" => {
                mark_once(&mut case_at, *key_at, "`:case`")?;
                let (s, s_at) = key_sym(&mut rest, *key_at, ":case")?;
                np.case_override = Some(case_of(s, s_at)?);
            }
            Value::Key(key, key_at) if key == "entity" => {
                mark_once(&mut entity_at, *key_at, "`:entity`")?;
                let (s, _) = key_sym(&mut rest, *key_at, ":entity")?;
                np.entity = Some(s.to_string());
            }
            Value::Key(key, key_at) => return err(*key_at, format!("unknown np key `:{key}`")),
            Value::List(child, child_at) => match head_of(child, *child_at)? {
                ("det", _) => {
                    if np.determiner.is_some() {
                        return err(*child_at, "`(np …)` takes at most one `(det …)`");
                    }
                    np.determiner = Some(sym_arg(child, "det", *child_at)?);
                }
                ("adj", _) => np.adjectives.push(sym_arg(child, "adj", *child_at)?),
                ("n", _) => {
                    if head_seen {
                        return err(*child_at, "`(np …)` takes exactly one `(n …)` head");
                    }
                    np.head = sym_arg(child, "n", *child_at)?;
                    head_seen = true;
                }
                ("num", _) => match child.as_slice() {
                    [_, Value::Int(n, _)] if np.count.is_none() => np.count = Some(*n),
                    [_, Value::Int(_, _)] => {
                        return err(*child_at, "`(np …)` takes at most one `(num …)`");
                    }
                    _ => return err(*child_at, "`(num …)` takes one integer"),
                },
                ("rel", _) => {
                    if np.relative.is_some() {
                        return err(*child_at, "`(np …)` takes at most one `(rel …)`");
                    }
                    np.relative = Some(Box::new(compile_rel(child, *child_at)?));
                }
                (other, other_at) => {
                    return err(other_at, format!("unknown np child `{other}`"));
                }
            },
            other => return err(other.at(), "unexpected atom inside `(np …)`"),
        }
    }
    if !head_seen {
        return err(at, "`(np …)` needs a `(n …)` head");
    }
    Ok(np)
}

fn compile_rel(items: &[Value], at: usize) -> Result<RelClause, SexprError> {
    let mut gap_kind: Option<&str> = None;
    let mut gap_prep: Option<String> = None;
    let mut gap_prep_at = None;
    let mut gap_case: Option<Case> = None;
    let mut gap_case_at = None;
    let mut subject: Option<Nominal> = None;
    let mut vp: Option<VerbPhrase> = None;
    let mut tense = TenseSpec::Present;
    let mut tense_at = None;
    let mut polarity = Polarity::Affirmative;
    let mut polarity_at = None;
    let mut relativizer = Relativizer::default();
    let mut relativizer_at = None;
    let mut gap_at = None;

    let mut rest = items[1..].iter().peekable();
    while let Some(item) = rest.next() {
        match item {
            Value::Key(key, key_at) => match key.as_str() {
                "gap" => {
                    mark_once(&mut gap_at, *key_at, "`:gap`")?;
                    let (s, s_at) = key_sym(&mut rest, *key_at, ":gap")?;
                    gap_kind = Some(match s {
                        "subj" => "subj",
                        "obj" => "obj",
                        "pp" => "pp",
                        other => return err(s_at, format!("unknown gap `{other}`")),
                    });
                }
                "case" => {
                    mark_once(&mut gap_case_at, *key_at, "`:case`")?;
                    let (s, s_at) = key_sym(&mut rest, *key_at, ":case")?;
                    gap_case = Some(case_of(s, s_at)?);
                }
                "tense" => {
                    mark_once(&mut tense_at, *key_at, "`:tense`")?;
                    let (s, s_at) = key_sym(&mut rest, *key_at, ":tense")?;
                    tense = tense_of(s, s_at)?;
                }
                "relativizer" => {
                    mark_once(&mut relativizer_at, *key_at, "`:relativizer`")?;
                    let (s, s_at) = key_sym(&mut rest, *key_at, ":relativizer")?;
                    relativizer = match s {
                        "ktory" => Relativizer::Ktory,
                        "iže" => Relativizer::Iže,
                        other => return err(s_at, format!("unknown relativizer `{other}`")),
                    };
                }
                "neg" => {
                    mark_once(&mut polarity_at, *key_at, "`:neg`")?;
                    polarity = Polarity::Negative;
                }
                other => return err(*key_at, format!("unknown rel key `:{other}`")),
            },
            Value::List(child, child_at) => match head_of(child, *child_at)? {
                ("vp", _) => {
                    if vp.is_some() {
                        return err(*child_at, "`(rel …)` takes exactly one `(vp …)`");
                    }
                    vp = Some(compile_vp(child, *child_at)?);
                }
                ("prep", _) => {
                    if gap_prep.is_some() {
                        return err(*child_at, "`(rel …)` takes at most one `(prep …)`");
                    }
                    gap_prep_at = Some(*child_at);
                    gap_prep = Some(sym_arg(child, "prep", *child_at)?);
                }
                ("np" | "pron" | "name" | "coord", _) => {
                    if subject.is_some() {
                        return err(
                            *child_at,
                            "`(rel …)` takes at most one overt subject nominal",
                        );
                    }
                    subject = Some(compile_nominal(item)?);
                }
                (other, other_at) => return err(other_at, format!("unknown rel child `{other}`")),
            },
            other => return err(other.at(), "unexpected atom inside `(rel …)`"),
        }
    }

    let gap = match gap_kind {
        Some("subj") => {
            if let Some(gap_prep_at) = gap_prep_at {
                return err(gap_prep_at, "`(prep …)` requires `:gap pp`");
            }
            if let Some(gap_case_at) = gap_case_at {
                return err(gap_case_at, "`:case` requires `:gap pp`");
            }
            GapRole::Subject
        }
        Some("obj") => {
            if let Some(gap_prep_at) = gap_prep_at {
                return err(gap_prep_at, "`(prep …)` requires `:gap pp`");
            }
            if let Some(gap_case_at) = gap_case_at {
                return err(gap_case_at, "`:case` requires `:gap pp`");
            }
            GapRole::Object
        }
        Some("pp") => GapRole::PpObject {
            preposition: gap_prep.ok_or_else(|| SexprError {
                at,
                msg: "a pp gap needs a `(prep …)`".into(),
            })?,
            case: gap_case.ok_or_else(|| SexprError {
                at,
                msg: "a pp gap needs `:case`".into(),
            })?,
        },
        _ => return err(at, "`(rel …)` needs `:gap subj|obj|pp`"),
    };
    let Some(vp) = vp else {
        return err(at, "`(rel …)` needs a `(vp …)`");
    };
    if gap == GapRole::Subject && subject.is_some() {
        return err(
            at,
            "a subject gap cannot also have an overt subject nominal",
        );
    }
    if gap != GapRole::Subject && subject.is_none() {
        return err(at, "a non-subject gap needs an overt subject nominal");
    }
    if gap == GapRole::Object && vp.object.is_some() {
        return err(at, "an object gap cannot also have a VP object");
    }
    Ok(RelClause {
        gap,
        subject,
        vp,
        tense,
        polarity,
        relativizer,
    })
}

fn compile_pron(items: &[Value], at: usize) -> Result<Nominal, SexprError> {
    let (mut person, mut number, mut gender, mut clitic) = (None, None, None, false);
    for item in &items[1..] {
        let Value::Key(key, key_at) = item else {
            return err(item.at(), "`(pron …)` takes only keywords");
        };
        match key.as_str() {
            "1" | "2" | "3" => {
                if person.is_some() {
                    return err(*key_at, "`(pron …)` takes exactly one person");
                }
                person = Some(match key.as_str() {
                    "1" => Person::First,
                    "2" => Person::Second,
                    _ => Person::Third,
                });
            }
            "sg" | "pl" => {
                if number.is_some() {
                    return err(*key_at, "`(pron …)` takes exactly one number");
                }
                number = Some(if key == "sg" {
                    Number::Singular
                } else {
                    Number::Plural
                });
            }
            "m" | "f" | "n" => {
                if gender.is_some() {
                    return err(*key_at, "`(pron …)` takes at most one gender");
                }
                gender = Some(match key.as_str() {
                    "m" => Gender::Masculine,
                    "f" => Gender::Feminine,
                    _ => Gender::Neuter,
                });
            }
            "clitic" => {
                if clitic {
                    return err(*key_at, "duplicate `:clitic`");
                }
                clitic = true;
            }
            other => return err(*key_at, format!("unknown pron key `:{other}`")),
        }
    }
    match (person, number) {
        (Some(person), Some(number)) => Ok(Nominal::Pron {
            person,
            number,
            gender: gender.unwrap_or(Gender::Masculine),
            clitic,
        }),
        _ => err(
            at,
            "`(pron …)` needs a person (:1/:2/:3) and number (:sg/:pl)",
        ),
    }
}

fn compile_name(items: &[Value], at: usize) -> Result<Nominal, SexprError> {
    let mut text = None;
    let mut gender = None;
    let mut indeclinable = false;
    for item in &items[1..] {
        match item {
            Value::Sym(s, s_at) => {
                if text.is_some() {
                    return err(*s_at, "`(name …)` takes exactly one name");
                }
                text = Some(s.clone());
            }
            Value::Key(key, key_at) => match key.as_str() {
                "m" | "f" | "n" => {
                    if gender.is_some() {
                        return err(*key_at, "`(name …)` takes exactly one gender");
                    }
                    gender = Some(match key.as_str() {
                        "m" => Gender::Masculine,
                        "f" => Gender::Feminine,
                        _ => Gender::Neuter,
                    });
                }
                "indecl" => {
                    if indeclinable {
                        return err(*key_at, "duplicate `:indecl`");
                    }
                    indeclinable = true;
                }
                other => return err(*key_at, format!("unknown name key `:{other}`")),
            },
            other => return err(other.at(), "unexpected value inside `(name …)`"),
        }
    }
    match (text, gender) {
        (Some(text), Some(gender)) => Ok(Nominal::Name {
            text,
            gender,
            indeclinable,
        }),
        _ => err(at, "`(name …)` needs a word and a gender (:m/:f/:n)"),
    }
}

fn compile_coord(items: &[Value], at: usize) -> Result<Nominal, SexprError> {
    let mut rest = items[1..].iter();
    let conjunction = match rest.next() {
        Some(Value::Sym(s, s_at)) => conj_of(s, *s_at)?,
        _ => return err(at, "`(coord …)` needs a conjunction first (i|ili|a|ale)"),
    };
    let mut coord_items = Vec::new();
    for item in rest {
        coord_items.push(compile_nominal(item)?);
    }
    if coord_items.len() < 2 {
        return err(at, "`(coord …)` needs at least two nominals");
    }
    Ok(Nominal::Coord(Coordination {
        conjunction,
        items: coord_items,
    }))
}

fn compile_vp(items: &[Value], at: usize) -> Result<VerbPhrase, SexprError> {
    let mut verb = None;
    let mut object = None;
    let mut adverbs = Vec::new();
    let mut pps = Vec::new();
    for item in &items[1..] {
        let Value::List(child, child_at) = item else {
            return err(item.at(), "unexpected atom inside `(vp …)`");
        };
        match head_of(child, *child_at)? {
            ("v", _) => {
                if verb.is_some() {
                    return err(*child_at, "`(vp …)` takes exactly one `(v …)`");
                }
                let words: Vec<&str> = child[1..]
                    .iter()
                    .map(|value| match value {
                        Value::Sym(s, _) => Ok(s.as_str()),
                        other => err(other.at(), "`(v …)` takes symbols"),
                    })
                    .collect::<Result<_, _>>()?;
                if words.is_empty() {
                    return err(*child_at, "`(v …)` needs a verb");
                }
                verb = Some(words.join(" "));
            }
            ("adv", _) => adverbs.push(sym_arg(child, "adv", *child_at)?),
            ("np" | "pron" | "name" | "coord", _) => {
                if object.is_some() {
                    return err(*child_at, "`(vp …)` takes at most one object");
                }
                object = Some(compile_nominal(item)?);
            }
            ("pp", _) => pps.push(compile_pp(child, *child_at)?),
            (other, other_at) => return err(other_at, format!("unknown vp child `{other}`")),
        }
    }
    let Some(verb) = verb else {
        return err(at, "`(vp …)` needs a `(v …)`");
    };
    Ok(VerbPhrase {
        verb,
        object,
        adverbs,
        pps,
    })
}

fn compile_pp(items: &[Value], at: usize) -> Result<PrepPhrase, SexprError> {
    let mut preposition = None;
    let mut case = None;
    let mut case_at = None;
    let mut object = None;
    let mut rest = items[1..].iter().peekable();
    while let Some(item) = rest.next() {
        match item {
            Value::Key(key, key_at) if key == "case" => {
                mark_once(&mut case_at, *key_at, "`:case`")?;
                let (s, s_at) = key_sym(&mut rest, *key_at, ":case")?;
                case = Some(case_of(s, s_at)?);
            }
            Value::Key(key, key_at) => return err(*key_at, format!("unknown pp key `:{key}`")),
            Value::List(child, child_at) => match head_of(child, *child_at)? {
                ("prep", _) => {
                    if preposition.is_some() {
                        return err(*child_at, "`(pp …)` takes exactly one `(prep …)`");
                    }
                    preposition = Some(sym_arg(child, "prep", *child_at)?);
                }
                ("np" | "pron" | "name" | "coord", _) => {
                    if object.is_some() {
                        return err(*child_at, "`(pp …)` takes exactly one object");
                    }
                    object = Some(compile_nominal(item)?);
                }
                (other, other_at) => return err(other_at, format!("unknown pp child `{other}`")),
            },
            other => return err(other.at(), "unexpected atom inside `(pp …)`"),
        }
    }
    match (preposition, object) {
        (Some(preposition), Some(object)) => Ok(PrepPhrase {
            preposition,
            case,
            object,
        }),
        _ => err(at, "`(pp …)` needs a `(prep …)` and an object"),
    }
}

// ---------------------------------------------------------------------------
// Canonical printer.
// ---------------------------------------------------------------------------

/// Print a clause in canonical form: children in fixed order, only
/// non-default keys emitted, single-item coordinations normalized to
/// their item. The round-trip contract: `compile_clause(parse(print(c)))`
/// equals `c` for every CANONICAL tree, and `print` is idempotent under
/// re-parsing (`print(parse(print(c))) == print(c)`) for every tree —
/// printing canonicalizes.
pub fn print(clause: &Clause) -> String {
    let mut out = String::from("(clause ");
    print_nominal(&clause.subject, &mut out);
    match &clause.core {
        ClauseCore::Verbal(coordination) => {
            for vp in &coordination.items {
                out.push(' ');
                print_vp(vp, &mut out);
            }
            if coordination.conjunction != Conj::I {
                out.push_str(" :conj ");
                out.push_str(coordination.conjunction.word());
            }
        }
        ClauseCore::Copular {
            predicate,
            pred_case,
        } => {
            out.push_str(" (pred ");
            match predicate {
                Predicate::Nominal(np) => print_np(np, &mut out),
                Predicate::Adjectival(adjective) => out.push_str(&format!("(adj {adjective})")),
                Predicate::Participial(infinitive) => out.push_str(&format!("(part {infinitive})")),
            }
            out.push(')');
            if *pred_case == PredCase::Instrumental {
                out.push_str(" :pred-case ins");
            }
        }
    }
    match clause.tense {
        TenseSpec::Present => {}
        TenseSpec::Past => out.push_str(" :tense past"),
        TenseSpec::Future => out.push_str(" :tense future"),
    }
    if clause.polarity == Polarity::Negative {
        out.push_str(" :neg");
    }
    match clause.force {
        Force::Declarative => {}
        Force::LiQuestion => out.push_str(" :force li"),
        Force::CiQuestion => out.push_str(" :force či"),
        Force::IntonationQuestion => out.push_str(" :force intonation"),
        Force::Imperative(addressee) => {
            out.push_str(" :force imp");
            match addressee {
                Addressee::You => {}
                Addressee::We => out.push_str(" :addressee 1pl"),
                Addressee::YouAll => out.push_str(" :addressee 2pl"),
            }
        }
    }
    if clause.mood == Mood::Conditional {
        out.push_str(" :mood cond");
    }
    if clause.voice == Voice::Passive {
        out.push_str(" :voice passive");
    }
    if let Some(topic) = clause.topic {
        out.push_str(" :topic ");
        out.push_str(slot_name(topic));
    }
    if let Some(focus) = clause.focus {
        out.push_str(" :focus ");
        out.push_str(slot_name(focus));
    }
    if clause.prodrop {
        out.push_str(" :prodrop");
    }
    out.push(')');
    out
}

fn slot_name(slot: SlotRef) -> &'static str {
    match slot {
        SlotRef::Subject => "subj",
        SlotRef::Object => "obj",
    }
}

fn print_nominal(nominal: &Nominal, out: &mut String) {
    match nominal {
        Nominal::Np(np) => print_np(np, out),
        Nominal::Pron {
            person,
            number,
            gender,
            clitic,
        } => {
            let p = match person {
                Person::First => "1",
                Person::Second => "2",
                Person::Third => "3",
            };
            let n = match number {
                Number::Singular => "sg",
                Number::Plural => "pl",
            };
            let g = match gender {
                Gender::Masculine => "m",
                Gender::Feminine => "f",
                Gender::Neuter => "n",
            };
            out.push_str(&format!("(pron :{p} :{n} :{g}"));
            if *clitic {
                out.push_str(" :clitic");
            }
            out.push(')');
        }
        Nominal::Name {
            text,
            gender,
            indeclinable,
        } => {
            let g = match gender {
                Gender::Masculine => "m",
                Gender::Feminine => "f",
                Gender::Neuter => "n",
            };
            out.push_str(&format!("(name {text} :{g}"));
            if *indeclinable {
                out.push_str(" :indecl");
            }
            out.push(')');
        }
        Nominal::Coord(coordination) => {
            // Canonicalization: a single-item coordination prints as its
            // item — the reader requires >= 2 conjuncts, and the two
            // trees are surface-indistinguishable anyway. See the
            // round-trip contract on [`print`].
            if let [only] = coordination.items.as_slice() {
                print_nominal(only, out);
                return;
            }
            out.push_str(&format!("(coord {}", coordination.conjunction.word()));
            for item in &coordination.items {
                out.push(' ');
                print_nominal(item, out);
            }
            out.push(')');
        }
    }
}

fn print_np(np: &NounPhrase, out: &mut String) {
    out.push_str("(np");
    if let Some(case) = np.case_override {
        out.push_str(" :case ");
        out.push_str(case_name(case));
    }
    if let Some(entity) = &np.entity {
        out.push_str(&format!(" :entity {entity}"));
    }
    if let Some(det) = &np.determiner {
        out.push_str(&format!(" (det {det})"));
    }
    if let Some(n) = np.count {
        out.push_str(&format!(" (num {n})"));
    }
    for adjective in &np.adjectives {
        out.push_str(&format!(" (adj {adjective})"));
    }
    out.push_str(&format!(" (n {})", np.head));
    if let Some(rel) = &np.relative {
        out.push(' ');
        print_rel(rel, out);
    }
    out.push(')');
}

fn print_rel(rel: &RelClause, out: &mut String) {
    out.push_str("(rel :gap ");
    match &rel.gap {
        GapRole::Subject => out.push_str("subj"),
        GapRole::Object => out.push_str("obj"),
        GapRole::PpObject { preposition, case } => {
            out.push_str(&format!(
                "pp (prep {preposition}) :case {}",
                case_name(*case)
            ));
        }
    }
    if let Some(subject) = &rel.subject {
        out.push(' ');
        print_nominal(subject, out);
    }
    out.push(' ');
    print_vp(&rel.vp, out);
    match rel.tense {
        TenseSpec::Present => {}
        TenseSpec::Past => out.push_str(" :tense past"),
        TenseSpec::Future => out.push_str(" :tense future"),
    }
    if rel.polarity == Polarity::Negative {
        out.push_str(" :neg");
    }
    if rel.relativizer == Relativizer::Iže {
        out.push_str(" :relativizer iže");
    }
    out.push(')');
}

fn print_vp(vp: &VerbPhrase, out: &mut String) {
    out.push_str(&format!("(vp (v {})", vp.verb));
    for adverb in &vp.adverbs {
        out.push_str(&format!(" (adv {adverb})"));
    }
    if let Some(object) = &vp.object {
        out.push(' ');
        print_nominal(object, out);
    }
    for pp in &vp.pps {
        out.push_str(&format!(" (pp (prep {})", pp.preposition));
        if let Some(case) = pp.case {
            out.push_str(" :case ");
            out.push_str(case_name(case));
        }
        out.push(' ');
        print_nominal(&pp.object, out);
        out.push(')');
    }
    out.push(')');
}

/// Parse + compile in one step — the template entry point.
pub fn clause_from_str(input: &str) -> Result<Clause, SexprError> {
    compile_clause(&parse(input)?)
}
