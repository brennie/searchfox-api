use failure::{format_err, Error};
use reqwest::header::ACCEPT;
use reqwest::{Client, Url};
use searchfox_api::Response;
use structopt::StructOpt;

static BASE_URL: &'static str = "https://searchfox.org";
static TRUE: &'static str = "true";
static FALSE: &'static str = "false";

fn bool_to_str(b: bool) -> &'static str {
    if b {
        TRUE
    } else {
        FALSE
    }
}

macro_rules! repo {
    ( $($variant:ident => $default_name:expr $(,$name:expr)*);+ ) => {
        #[derive(Debug, Eq, PartialEq)]
        enum Repo {
            $($variant),+
        }

        impl ::std::fmt::Display for Repo {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                match self {
                    $(Repo::$variant => write!(f, "{}", $default_name) ),+
                }
            }
        }

        impl ::std::str::FromStr for Repo {
            type Err = String;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                $(
                    if s.eq_ignore_ascii_case($default_name) $(|| s.eq_ignore_ascii_case($name))* {
                        Ok(Repo::$variant)
                    }
                ) else +
                else {
                    Err(String::from("dafuq"))
                }
            }
        }

        impl Repo {
            pub fn values() -> Vec<&'static str> {
                vec![
                    $(
                        $default_name $(, $name)*
                    ),+
                ]
            }
        }
    }
}

repo! {
    MozillaCentral => "mozilla-central", "central", "m-c", "mc";
    MozillaMobile => "mozilla-mobile", "mobile";
    CommCentral => "comm-central", "comm";
    Nss => "nss";
    WhatWgHtml => "what-wg-html", "what-wg", "what", "html";
    MozillaBeta => "mozilla-beta", "beta";
    Mozillarelease => "mozilla-release", "release";
    MozillaEsr60 => "mozilla-esr60", "esr60"
}

/// Make a query to Searchfox and dump the parsed results.
#[derive(Debug, Eq, PartialEq, StructOpt)]
struct Options {
    /// Perform a case-sensitive search.
    #[structopt(long = "case-sensitive")]
    case_sensitive: bool,

    /// A path to limit the query to.
    #[structopt(long = "path", default_value = "")]
    path: String,

    /// Perform a regular expression search.
    #[structopt(long = "regex")]
    regex: bool,

    /// The repository to use on searchfox.org
    #[structopt(
        long = "repo",
        raw(possible_values = "&Repo::values()"),
        next_line_help = true,
        case_insensitive = true,
        default_value = "Repo::MozillaCentral"
    )]
    repository: Repo,

    /// The search query.
    ///
    /// The query must be at least three characters long.
    #[structopt(name = "query", parse(try_from_str = "parse_query"))]
    query: String,
}

fn parse_query(s: &str) -> Result<String, String> {
    if s.len() >= 3 {
        Ok(s.into())
    } else {
        Err("Queries must be at least three characters.".into())
    }
}

fn build_url(opts: &Options) -> Url {
    let mut url = Url::parse(&format!(
        "{}/{}/search",
        BASE_URL,
        &opts.repository.to_string()
    ))
    .unwrap();

    url.query_pairs_mut()
        .append_pair("q", &opts.query)
        .append_pair("case", bool_to_str(opts.case_sensitive))
        .append_pair("regex", bool_to_str(opts.regex))
        .append_pair("path", &opts.path);

    url
}

fn main() -> Result<(), Error> {
    let opts = Options::from_args();
    let url = build_url(&opts);
    let client = Client::new();

    let mut rsp = client.get(url).header(ACCEPT, "application/json").send()?;

    if !rsp.status().is_success() {
        return Err(format_err!(
            "GET {} returned {:?}",
            rsp.url().to_string(),
            rsp.status()
        ));
    }

    let api_rsp: Response = rsp.json()?;

    println!("{:#?}", api_rsp);

    Ok(())
}
