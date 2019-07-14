// Copyright 2019 Barret Rennie.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT
// or http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use std::collections::HashMap;
use std::fmt;

use serde::de::{Deserializer, Error, MapAccess, Unexpected, Visitor};
use serde::Deserialize;

/// A response from Searchfox.
#[derive(Debug, Deserialize, Eq, PartialEq)]
pub struct Response {
    /// The query that was searched for.
    #[serde(rename = "*title*")]
    pub title: String,

    /// Whether or not the reques timed out.
    ///
    /// If true, the results will be incomplete.
    #[serde(rename = "*timedout*")]
    pub timedout: bool,

    /// Matches for the query that occur in code.
    pub normal: Option<Matches>,

    /// Matches for the query that occur in test code.
    pub test: Option<Matches>,

    /// Matches for the query that occur in generated code.
    pub generated: Option<Matches>,
}

/// A collection of matches for a query.
#[derive(Debug, Default, Eq, PartialEq)]
pub struct Matches {
    /// Declarations that match the query.
    pub declarations: FuzzyMatches,

    /// Definitions that match the query.
    pub definitions: FuzzyMatches,

    /// File names that match the query.
    pub files: Vec<String>,

    /// Plain text that matches the query.
    ///
    /// This includes strings, comments, and plain text files.
    pub text_matches: FileMatches,

    /// Code use that match the query.
    pub uses: FuzzyMatches,
}

/// Fuzzily found matches.
///
/// When searching for a query, results may be returned that do not match the
/// query exactly.
pub type FuzzyMatches = HashMap<String, FileMatches>;

/// A mapping of file names to query matches found within those files.
pub type FileMatches = HashMap<String, Vec<LineMatch>>;

/// A single match.
// #[derive(Debug, Deserialize, Eq, PartialEq)]
// pub struct FileMatches {
//     /// The path that contained the matches.
//     pub path: String,

//     /// Individual matches in the file.
//     pub lines: Vec<Match>,
// }

/// Context about a match.
#[derive(Debug, Deserialize, Eq, PartialEq)]
pub struct MatchContext {
    /// The context where the match was found.
    ///
    /// This is e.g. the name of the function that the match was found inside of.
    pub context: String,

    /// The symbol that Searchfox has generated for this context.
    ///
    /// This is a unique identifier for the context.
    #[serde(rename = "contextsym")]
    pub symbol: String,
}

/// A match for the query.
#[derive(Debug, Deserialize, Eq, PartialEq)]
pub struct LineMatch {
    /// The contents of the line.
    pub line: String,

    /// The line number in the file.
    #[serde(rename = "lno")]
    pub number: usize,

    /// The position in the line where the symbol was found.
    pub bounds: (usize, usize),

    /// Lines before and/or after the symbol that may provide some context.
    ///
    /// This will likely be a comment describing the match.
    #[serde(
        default,
        rename = "peekLines",
        deserialize_with = "deserialize_optional_string"
    )]
    pub peek_lines: Option<String>,

    /// Context for where the match was found.
    ///
    /// For example, this may be the name of the function where the match was
    /// found.
    #[serde(flatten, default, deserialize_with = "deserialize_optional_context")]
    pub context: Option<MatchContext>,
}

#[derive(Debug, Deserialize, Eq, PartialEq)]
struct RawMatch {
    lines: Vec<LineMatch>,
    path: String,
}

impl RawMatch {
    fn into_hashmap(matches: Vec<RawMatch>) -> FileMatches {
        matches.into_iter().map(|m| (m.path, m.lines)).collect()
    }

    fn into_paths(matches: Vec<RawMatch>) -> Vec<String> {
        for m in &matches {
            debug_assert!(m.lines.len() == 0);
        }

        matches.into_iter().map(|m| m.path).collect()
    }
}

static EXPECTED_FIELDS: &'static [&'static str] = &[
    "Files",
    "Textual Occurrences",
    "Definitions (...)",
    "Declarations (...)",
    "Uses (...)",
];

struct MatchesVisitor;
impl<'de> Visitor<'de> for MatchesVisitor {
    type Value = Matches;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a map of matches")
    }

    fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
    where
        M: MapAccess<'de>,
    {
        let mut matches = Matches::default();

        while let Some((key, value)) = access.next_entry::<String, Vec<RawMatch>>()? {
            if key == "Files" {
                matches.files = RawMatch::into_paths(value);
            } else if key == "Textual Occurrences" {
                matches.text_matches = RawMatch::into_hashmap(value);
            } else if key.ends_with(")") {
                let key = &key[..key.len() - 1];

                if key.starts_with("Definitions (") {
                    let name = &key["Definitions (".len()..];

                    matches
                        .definitions
                        .insert(name.into(), RawMatch::into_hashmap(value));
                } else if key.starts_with("Declarations (") {
                    let name = &key["Declarations (".len()..];
                    matches
                        .declarations
                        .insert(name.into(), RawMatch::into_hashmap(value));
                } else if key.starts_with("Uses (") {
                    let name = &key["Uses (".len()..];
                    matches
                        .uses
                        .insert(name.into(), RawMatch::into_hashmap(value));
                } else {
                    return Err(M::Error::unknown_field(&key, EXPECTED_FIELDS));
                }
            } else {
                return Err(M::Error::unknown_field(&key, EXPECTED_FIELDS));
            }
        }

        Ok(matches)
    }
}

impl<'de> Deserialize<'de> for Matches {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_map(MatchesVisitor)
    }
}

/// Deserialize an `Option<String>` where the empty string is `None`.
fn deserialize_optional_string<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    match <Option<String> as Deserialize>::deserialize(deserializer)? {
        Some(ref s) if s.len() == 0 => Ok(None),
        Some(s) => Ok(Some(s)),
        None => Ok(None),
    }
}

fn deserialize_optional_context<'de, D>(deserializer: D) -> Result<Option<MatchContext>, D::Error>
where
    D: Deserializer<'de>,
{
    match <Option<MatchContext> as Deserialize>::deserialize(deserializer)? {
        Some(ctx) => match (ctx.context.len(), ctx.symbol.len()) {
            (0, 0) => Ok(None),
            (0, _) => Err(D::Error::invalid_value(
                Unexpected::Str(&ctx.symbol),
                &"an empty symbol",
            )),
            (_, 0) => Err(D::Error::invalid_value(
                Unexpected::Str(&ctx.symbol),
                &"a non-empty symbol",
            )),
            (_, _) => Ok(Some(ctx)),
        },
        None => Ok(None),
    }
}
