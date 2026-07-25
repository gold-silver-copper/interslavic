//! Hierarchical surface plans.
//!
//! Nested nominals and relatives remain opaque nodes while each verb
//! domain places its own clitics. The tree is flattened exactly once by
//! [`ClausePlan::stringify`].

use crate::ast::Force;
use crate::profile::NominalProfile;
use crate::resolve::CaseSource;
use interslavic::Case;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum SurfaceNode {
    Word(String),
    Punct(char),
    Nominal(Box<NominalPlan>),
    Relative(Box<RelativePlan>),
    /// A subordinate clause, already fully planned including its own
    /// clitic placement. It is opaque to the parent for the same reason a
    /// relative is: the parent must not be able to reach inside and move
    /// anything out.
    Subordinate(Box<SubordinatePlan>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct NominalPlan {
    pub body: Vec<SurfaceNode>,
    /// Present only when this nominal itself—not any descendant—resolved
    /// as a clitic pronoun.
    pub direct_clitic: Option<String>,
    pub profile: NominalProfile,
    pub case: Case,
    pub case_source: CaseSource,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct RelativePlan {
    pub body: Vec<SurfaceNode>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct SubordinatePlan {
    pub body: Vec<SurfaceNode>,
}

#[derive(Debug, Clone)]
pub(crate) struct VerbDomainPlan {
    pub complex: Vec<SurfaceNode>,
    pub cluster: Vec<String>,
    pub recipient: Option<NominalPlan>,
    pub object: Option<NominalPlan>,
    pub object_case: Option<Case>,
    pub adjuncts: Vec<Vec<SurfaceNode>>,
    /// The verb's finite complement clause, already sealed. It follows
    /// every other complement, so it is held separately rather than in
    /// `adjuncts`.
    pub complement_clause: Option<Vec<SurfaceNode>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum SlotKind {
    Subject,
    Verb(usize),
    Recipient(usize),
    Object(usize),
    QuestionParticle(QuestionParticle),
    InitialAdjunct(usize),
    Fixed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum QuestionParticle {
    Li,
    Ci,
}

#[derive(Debug, Clone)]
pub(crate) struct Constituent {
    pub slot: SlotKind,
    pub nodes: Vec<SurfaceNode>,
}

#[derive(Debug, Clone)]
pub(crate) struct ClausePlan {
    pub lead_in: Option<String>,
    pub constituents: Vec<Constituent>,
    pub force: Force,
    pub sentence: bool,
}

pub(crate) fn word(text: impl Into<String>) -> SurfaceNode {
    SurfaceNode::Word(text.into())
}

impl NominalPlan {
    pub fn into_surface(self) -> Vec<SurfaceNode> {
        if let Some(clitic) = self.direct_clitic {
            vec![word(clitic)]
        } else {
            vec![SurfaceNode::Nominal(Box::new(self))]
        }
    }
}

impl ClausePlan {
    pub fn stringify(self) -> String {
        let mut flat = Vec::new();
        if let Some(lead_in) = self.lead_in {
            flat.push(FlatToken::Word(lead_in));
        }
        for constituent in self.constituents {
            for node in constituent.nodes {
                flatten(node, &mut flat);
            }
        }
        join_flat(&flat, self.force, self.sentence)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum FlatToken {
    Word(String),
    Punct(char),
}

fn flatten(node: SurfaceNode, out: &mut Vec<FlatToken>) {
    match node {
        SurfaceNode::Word(text) => out.push(FlatToken::Word(text)),
        SurfaceNode::Punct(mark) => {
            // Boundaries can coincide: a relative's closing comma may
            // also be the delimiter before the next coordination item.
            // They are one orthographic boundary, not two marks.
            if out.last() != Some(&FlatToken::Punct(mark)) {
                out.push(FlatToken::Punct(mark));
            }
        }
        SurfaceNode::Nominal(plan) => {
            debug_assert!(
                plan.direct_clitic.is_none(),
                "a direct clitic must be placed by its owning verb domain"
            );
            for child in plan.body {
                flatten(child, out);
            }
        }
        SurfaceNode::Relative(plan) => {
            for child in plan.body {
                flatten(child, out);
            }
        }
        SurfaceNode::Subordinate(plan) => {
            for child in plan.body {
                flatten(child, out);
            }
        }
    }
}

fn join_flat(tokens: &[FlatToken], force: Force, sentence: bool) -> String {
    let mut out = String::new();
    let mut trailing_comma = false;
    for token in tokens {
        match token {
            FlatToken::Punct(mark) => {
                out.push(*mark);
                trailing_comma = *mark == ',';
            }
            FlatToken::Word(text) => {
                if !out.is_empty() {
                    out.push(' ');
                }
                out.push_str(text);
                trailing_comma = false;
            }
        }
    }
    if sentence {
        if trailing_comma {
            out.pop();
        }
        let mut chars = out.chars();
        if let Some(first) = chars.next() {
            out = first.to_uppercase().collect::<String>() + chars.as_str();
        }
        out.push(match force {
            Force::Declarative => '.',
            Force::Imperative(_) | Force::Optative => '!',
            _ => '?',
        });
    } else {
        while out.ends_with([',', ' ']) {
            out.pop();
        }
    }
    out
}
