//! Central nominal-feature resolution.
//!
//! Inflection, finite agreement, and discourse reference are related but
//! not identical. This module is the only place that decides those
//! numbers, so realization and discourse cannot drift into contradictory
//! policies.

use crate::ast::{Nominal, ReferentialForm};
use interslavic::{Animacy, Gender, Number, Person, noun_info};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct NominalProfile {
    pub person: Person,
    pub gender: Gender,
    pub animacy: Animacy,
    pub inflection_number: Number,
    pub agreement_number: Number,
    pub agreement_gender: Gender,
    pub referent_number: Number,
}

pub(crate) fn nominal_profile(nominal: &Nominal) -> NominalProfile {
    match nominal {
        Nominal::Pron {
            person,
            number,
            gender,
            ..
        } => NominalProfile {
            person: *person,
            gender: *gender,
            animacy: Animacy::Animate,
            inflection_number: *number,
            agreement_number: *number,
            agreement_gender: *gender,
            referent_number: *number,
        },
        Nominal::Name { gender, .. } => NominalProfile {
            person: Person::Third,
            gender: *gender,
            animacy: Animacy::Animate,
            inflection_number: Number::Singular,
            agreement_number: Number::Singular,
            agreement_gender: *gender,
            referent_number: Number::Singular,
        },
        Nominal::Np(np) => {
            let info = noun_info(&np.head);
            let (inflection_number, agreement_number, agreement_gender, referent_number) = if info
                .plural_only
            {
                // A plural-only lexeme remains grammatically and
                // referentially plural even under an explicit count
                // of one. This avoids "1 noviny leži ... one ležęt".
                (Number::Plural, Number::Plural, info.gender, Number::Plural)
            } else {
                match np.count {
                    // An explicit grammatical number applies only when no
                    // numeral is present; a numeral computes its own.
                    None if np.number == Some(Number::Plural) => {
                        (Number::Plural, Number::Plural, info.gender, Number::Plural)
                    }
                    None | Some(1) => (
                        Number::Singular,
                        Number::Singular,
                        info.gender,
                        Number::Singular,
                    ),
                    Some(2..=4) => (Number::Plural, Number::Plural, info.gender, Number::Plural),
                    Some(_) => (
                        Number::Plural,
                        Number::Singular,
                        Gender::Neuter,
                        Number::Plural,
                    ),
                }
            };
            let (agreement_number, agreement_gender, inflection_number) =
                if np.referential == ReferentialForm::Full {
                    (agreement_number, agreement_gender, inflection_number)
                } else {
                    // A planned pronoun/clitic agrees as a pronoun with
                    // the referent, not with a quantified surface phrase.
                    (referent_number, info.gender, referent_number)
                };
            NominalProfile {
                person: Person::Third,
                gender: info.gender,
                animacy: info.animacy,
                inflection_number,
                agreement_number,
                agreement_gender,
                referent_number,
            }
        }
        Nominal::Coord(coordination) => {
            let profiles: Vec<_> = coordination.items.iter().map(nominal_profile).collect();
            let first = profiles.first().copied().unwrap_or(NominalProfile {
                person: Person::Third,
                gender: Gender::Masculine,
                animacy: Animacy::Inanimate,
                inflection_number: Number::Plural,
                agreement_number: Number::Plural,
                agreement_gender: Gender::Masculine,
                referent_number: Number::Plural,
            });
            let person = profiles
                .iter()
                .map(|profile| profile.person)
                .min_by_key(|person| match person {
                    Person::First => 0,
                    Person::Second => 1,
                    Person::Third => 2,
                })
                .unwrap_or(Person::Third);
            let gender = if profiles
                .iter()
                .all(|profile| profile.gender == first.gender)
            {
                first.gender
            } else {
                Gender::Masculine
            };
            let animacy = if profiles
                .iter()
                .any(|profile| profile.animacy == Animacy::Animate)
            {
                Animacy::Animate
            } else {
                Animacy::Inanimate
            };
            let plural = coordination.items.len() > 1
                || profiles
                    .iter()
                    .any(|profile| profile.referent_number == Number::Plural);
            let number = if plural {
                Number::Plural
            } else {
                first.referent_number
            };
            NominalProfile {
                person,
                gender,
                animacy,
                inflection_number: number,
                agreement_number: number,
                agreement_gender: gender,
                referent_number: number,
            }
        }
    }
}
