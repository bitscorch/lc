use anyhow::{Context, Result, bail};
use serde::Deserialize;
use std::{fs, path::Path, thread, time::Duration};

const GRAPHQL_URL: &str = "https://leetcode.com/graphql";
const BASE_URL: &str = "https://leetcode.com";
const USER_AGENT: &str = "github.com/bitscorch/lc";

/// A single problem, as returned by LeetCode's GraphQL API.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Question {
    pub question_id: String,
    /// The number shown in the UI, e.g. "70".
    pub question_frontend_id: String,
    pub title: String,
    pub title_slug: String,
    /// Problem statement as HTML. `None` for Premium-locked problems.
    pub content: Option<String>,
    pub difficulty: String,
    /// Public example inputs, newline-separated. NOT the hidden test suite.
    pub example_testcases: String,
    /// JSON describing the function: name, param types, return type.
    pub meta_data: String,
    /// Starter code, one entry per language.
    pub code_snippets: Vec<CodeSnippet>,
    pub topic_tags: Vec<TopicTag>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CodeSnippet {
    pub lang: String,
    /// e.g. "rust", "python3", "cpp" — match against this, not `lang`.
    pub lang_slug: String,
    pub code: String,
}

#[derive(Debug, Deserialize)]
pub struct TopicTag {
    pub name: String,
}

#[derive(Deserialize)]
struct GraphQlResponse {
    data: DataField,
}

#[derive(Deserialize)]
struct DataField {
    question: Option<Question>,
}

const QUERY: &str = r"
query questionData($titleSlug: String!) {
  question(titleSlug: $titleSlug) {
    questionId
    questionFrontendId
    title
    titleSlug
    content
    difficulty
    exampleTestcases
    metaData
    codeSnippets { lang langSlug code }
    topicTags { name }
  }
}
";

/// Fetch a problem by its URL slug, e.g. "two-sum".
///
/// Works anonymously for public problems. Premium problems return a question
/// with `content: None` (you'd need a session cookie to see those — and
/// shouldn't republish them anyway).
pub fn fetch_question(slug: &str) -> Result<Question> {
    let body = serde_json::json!({
        "query": QUERY,
        "variables": { "titleSlug": slug },
    });

    let resp: GraphQlResponse = reqwest::blocking::Client::new()
        .post(GRAPHQL_URL)
        .header(reqwest::header::USER_AGENT, USER_AGENT)
        // LeetCode rejects GraphQL calls without a matching Referer.
        .header(
            reqwest::header::REFERER,
            format!("https://leetcode.com/problems/{slug}/"),
        )
        .json(&body)
        .send()?
        .error_for_status()?
        .json()
        .context("Failed to decode GraphQL response")?;

    resp.data
        .question
        .with_context(|| format!("No problem found for slug '{slug}' (check the URL slug)"))
}

impl Question {
    /// The starter snippet for a given language slug, if LeetCode offers one.
    pub fn snippet(&self, lang_slug: &str) -> Option<&CodeSnippet> {
        self.code_snippets.iter().find(|s| s.lang_slug == lang_slug)
    }

    /// Number of function parameters, parsed from `metaData`. Each example
    /// case consumes this many lines of `example_testcases`. Defaults to 1
    /// (covers scalar problems and unparseable/design-style metadata).
    pub fn param_count(&self) -> usize {
        #[derive(Deserialize)]
        struct Meta {
            params: Vec<serde_json::Value>,
        }
        serde_json::from_str::<Meta>(&self.meta_data)
            .map(|m| m.params.len().max(1))
            .unwrap_or(1)
    }
}

/// LeetCode session credentials. Pulled live from a logged-in Brave session
/// when possible, otherwise from gitignored files at the repo root.
pub struct Credentials {
    /// The `LEETCODE_SESSION` cookie (a long JWT).
    session: String,
    /// The `csrftoken` cookie, also echoed back in the `x-csrftoken` header.
    csrf: String,
}

impl Credentials {
    /// Load credentials: first try reading live cookies straight from Brave,
    /// then fall back to the `.session` / `.session.csrf` files. The browser
    /// path means you never have to copy-paste cookies as long as you're logged
    /// in to leetcode.com in Brave.
    pub fn load(root: &Path) -> Result<Self> {
        match Self::from_brave() {
            Ok(creds) => Ok(creds),
            Err(browser_err) => Self::load_from_files(root).map_err(|file_err| {
                file_err.context(format!("Couldn't read cookies from Brave either: {browser_err}"))
            }),
        }
    }

    /// Read the LeetCode cookies directly from a logged-in Brave session.
    fn from_brave() -> Result<Self> {
        let cookies = rookie::brave(Some(vec!["leetcode.com".into()]))
            .map_err(|e| anyhow::anyhow!("reading Brave cookies failed: {e}"))?;
        let find = |name: &str| cookies.iter().find(|c| c.name == name).map(|c| c.value.clone());
        Ok(Self {
            session: find("LEETCODE_SESSION").context(
                "LEETCODE_SESSION cookie not found in Brave — log in to leetcode.com first",
            )?,
            csrf: find("csrftoken").context("csrftoken cookie not found in Brave")?,
        })
    }

    /// Load from `.session` (LEETCODE_SESSION) and `.session.csrf` (csrftoken).
    fn load_from_files(root: &Path) -> Result<Self> {
        Ok(Self {
            session: read_token(&root.join(".session"), "LEETCODE_SESSION")?,
            csrf: read_token(&root.join(".session.csrf"), "csrftoken")?,
        })
    }

    fn cookie(&self) -> String {
        format!("LEETCODE_SESSION={}; csrftoken={}", self.session, self.csrf)
    }
}

/// Read a single cookie token from `path`, trimming whitespace.
fn read_token(path: &Path, cookie_name: &str) -> Result<String> {
    let raw = fs::read_to_string(path).with_context(|| {
        format!(
            "Could not read '{}'.\n\
             Copy your `{cookie_name}` cookie value into this file \
             (it's gitignored). Find it in your browser's devtools under \
             Application > Cookies for leetcode.com.",
            path.display()
        )
    })?;
    let token = raw.trim().to_string();
    if token.is_empty() {
        bail!("'{}' is empty — paste your `{cookie_name}` cookie value", path.display());
    }
    Ok(token)
}

#[derive(Deserialize)]
struct SubmitResponse {
    submission_id: u64,
}

/// Submit `code` for `slug` to LeetCode's judge. `question_id` is the internal
/// id (`Question::question_id`), not the frontend number. Returns the new
/// submission id, which you then poll with [`poll_submission`].
pub fn submit(
    creds: &Credentials,
    slug: &str,
    question_id: &str,
    lang_slug: &str,
    code: &str,
) -> Result<u64> {
    let body = serde_json::json!({
        "lang": lang_slug,
        "question_id": question_id,
        "typed_code": code,
    });

    let resp = reqwest::blocking::Client::new()
        .post(format!("{BASE_URL}/problems/{slug}/submit/"))
        .header(reqwest::header::COOKIE, creds.cookie())
        .header("x-csrftoken", &creds.csrf)
        .header(reqwest::header::REFERER, format!("{BASE_URL}/problems/{slug}/"))
        .header(reqwest::header::USER_AGENT, USER_AGENT)
        .json(&body)
        .send()?;

    if !resp.status().is_success() {
        let status = resp.status();
        let text = resp.text().unwrap_or_default();
        bail!(
            "Submit rejected ({status}). Your session may have expired — \
             re-copy `.session` and `.session.csrf` from the browser.\n{text}"
        );
    }

    let parsed: SubmitResponse = resp
        .json()
        .context("Unexpected submit response (are the cookies for a logged-in session?)")?;
    Ok(parsed.submission_id)
}

/// The judged result of a submission. Most fields are only present for certain
/// verdicts (e.g. `last_testcase` on Wrong Answer, `compile_error` on a build
/// failure), so everything past `state` is optional.
#[derive(Deserialize)]
pub struct SubmissionResult {
    pub state: String,
    pub status_msg: Option<String>,
    pub total_correct: Option<u32>,
    pub total_testcases: Option<u32>,
    pub status_runtime: Option<String>,
    pub status_memory: Option<String>,
    pub runtime_percentile: Option<f64>,
    pub memory_percentile: Option<f64>,
    pub compile_error: Option<String>,
    pub full_compile_error: Option<String>,
    pub runtime_error: Option<String>,
    pub full_runtime_error: Option<String>,
    pub last_testcase: Option<String>,
    pub expected_output: Option<String>,
    pub code_output: Option<String>,
}

/// Poll LeetCode's check endpoint until the judge finishes (`state == "SUCCESS"`
/// means judging is complete, *not* that the answer was accepted).
pub fn poll_submission(creds: &Credentials, submission_id: u64) -> Result<SubmissionResult> {
    let url = format!("{BASE_URL}/submissions/detail/{submission_id}/check/");

    for _ in 0..60 {
        let resp: serde_json::Value = reqwest::blocking::Client::new()
            .get(&url)
            .header(reqwest::header::COOKIE, creds.cookie())
            .header(reqwest::header::REFERER, format!("{BASE_URL}/"))
            .header(reqwest::header::USER_AGENT, USER_AGENT)
            .send()?
            .error_for_status()?
            .json()
            .context("Failed to decode submission check response")?;

        if resp.get("state").and_then(serde_json::Value::as_str) == Some("SUCCESS") {
            return serde_json::from_value(resp).context("Failed to parse judged result");
        }
        thread::sleep(Duration::from_millis(700));
    }

    bail!("Timed out waiting for LeetCode to judge submission {submission_id}")
}
