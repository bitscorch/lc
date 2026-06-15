mod leetcode;

use anyhow::{Result, bail};
use clap::{Parser, Subcommand, ValueEnum};
use regex::Regex;
use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
};

#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Scaffold a solution file (and description) for a problem.
    New { slug: String, language: Language },
    /// Print a problem's description as markdown (caching it under descriptions/).
    Desc { slug: String },
    /// Submit a solved problem to LeetCode and report the verdict.
    Submit {
        slug: String,
        #[arg(value_enum, default_value_t = Language::Rust)]
        language: Language,
    },
}

#[derive(Copy, Clone, PartialEq, Eq, ValueEnum, Debug)]
enum Language {
    Rust,
    Python,
}

impl Language {
    /// LeetCode's `langSlug` for this language.
    fn lang_slug(self) -> &'static str {
        match self {
            Language::Rust => "rust",
            Language::Python => "python3",
        }
    }

    fn ext(self) -> &'static str {
        match self {
            Language::Rust => "rs",
            Language::Python => "py",
        }
    }
}

fn find_project_root() -> Result<PathBuf> {
    let mut current = std::env::current_dir()?;
    loop {
        if current.join("lc.toml").exists() {
            return Ok(current);
        }
        if !current.pop() {
            bail!(
                "Could not find 'lc.toml' root marker.\n\
                 Are you inside your LeetCode repository?\n\
                 (Run 'touch lc.toml' at your repo root to mark it)"
            );
        }
    }
}

/// LeetCode slugs use hyphens; turn them into a valid Rust file/bin stem.
fn file_stem(slug: &str) -> String {
    slug.replace('-', "_")
}

fn clean_md(md: &str) -> String {
    // html2md leaves runs of 3+ blank lines; collapse to one blank line.
    let re_newlines = Regex::new(r"\n{3,}").unwrap();
    re_newlines.replace_all(md.trim(), "\n\n").to_string()
}

/// Word-wrap regular prose to `width`, leaving structure untouched: code
/// blocks (```), list items, headings, blockquotes, and tables stay verbatim
/// so we don't mangle the example inputs/outputs or list formatting.
fn wrap_prose(md: &str, width: usize) -> String {
    let mut out = Vec::new();
    let mut in_code = false;

    for line in md.lines() {
        let trimmed = line.trim_start();

        if trimmed.starts_with("```") {
            in_code = !in_code;
            out.push(line.to_string());
            continue;
        }

        // Only real bullets (`* `/`- `/`+ ` with a space) count as list items;
        // `**bold**` prose like `**Follow-up:**` should wrap, not be left long.
        let is_list_or_block = trimmed.starts_with("* ")
            || trimmed.starts_with("- ")
            || trimmed.starts_with("+ ")
            || trimmed.starts_with(['>', '#', '|']);
        let is_prose = !in_code
            && !line.is_empty()
            && !is_list_or_block
            && !trimmed.chars().next().is_some_and(|c| c.is_ascii_digit());

        if is_prose {
            out.extend(wrap_line(line, width));
        } else {
            out.push(line.to_string());
        }
    }

    out.join("\n")
}

/// Greedily wrap a single line at word boundaries to at most `width` chars.
fn wrap_line(line: &str, width: usize) -> Vec<String> {
    let mut lines = Vec::new();
    let mut current = String::new();

    for word in line.split_whitespace() {
        if !current.is_empty() && current.len() + 1 + word.len() > width {
            lines.push(std::mem::take(&mut current));
        }
        if !current.is_empty() {
            current.push(' ');
        }
        current.push_str(word);
    }
    if !current.is_empty() {
        lines.push(current);
    }
    lines
}

fn cmd_desc(root: &Path, slug: &str) -> Result<()> {
    let desc_dir = root.join("descriptions");
    let desc_path = desc_dir.join(format!("{slug}.md"));

    if desc_path.exists() {
        println!("{}", fs::read_to_string(&desc_path)?);
        return Ok(());
    }

    let question = leetcode::fetch_question(slug)?;
    let Some(content) = question.content else {
        bail!("Problem '{slug}' has no public content (Premium-locked?)");
    };

    let md = clean_md(&html2md::parse_html(&content));
    fs::create_dir_all(&desc_dir)?;
    fs::write(&desc_path, &md)?;
    println!("{md}");
    Ok(())
}

/// Emit a single `#[rstest]` function with one `#[case(...)]` per example.
/// Inputs come from `example_testcases`, expected values from the description's
/// `Output:` lines, all run through [`rustify`]. Param names/types and the
/// return type are read from the starter signature so cases are strongly typed.
/// Unparsed outputs become a `_` that won't compile, forcing a manual fix.
fn generate_tests(question: &leetcode::Question, snippet: &str) -> String {
    let outputs = question
        .content
        .as_deref()
        .map(parse_outputs)
        .unwrap_or_default();

    let lines: Vec<&str> = question.example_testcases.lines().collect();

    let Some(sig) = parse_signature(snippet) else {
        return fallback_tests(snippet, &lines, &outputs, question.param_count());
    };

    let arity = sig.params.len().max(1);
    let cases = lines
        .chunks(arity)
        .enumerate()
        .map(|(i, args)| {
            let mut vals: Vec<String> = args
                .iter()
                .zip(&sig.params)
                .map(|(a, (_, ty))| coerce(rustify(a), ty))
                .collect();
            vals.push(
                outputs
                    .get(i)
                    .map(|o| coerce(rustify(o), &sig.ret))
                    .unwrap_or_else(|| "_".to_string()),
            );
            format!("    #[case({})]", vals.join(", "))
        })
        .collect::<Vec<_>>()
        .join("\n");

    let mut fn_params: Vec<String> = sig
        .params
        .iter()
        .map(|(name, ty)| format!("#[case] {name}: {ty}"))
        .collect();
    fn_params.push(format!("#[case] expected: {}", sig.ret));

    let call_args = sig
        .params
        .iter()
        .map(|(name, _)| name.as_str())
        .collect::<Vec<_>>()
        .join(", ");

    format!(
        "    #[rstest]\n{cases}\n    fn cases({}) {{\n        \
         assert_eq!(expected, Solution::{}({call_args}));\n    }}",
        fn_params.join(", "),
        sig.name,
    )
}

/// Per-case `#[test]` fns, used when the starter signature can't be parsed for
/// the parametrized form (e.g. exotic linked-list / tree types).
fn fallback_tests(snippet: &str, lines: &[&str], outputs: &[String], param_count: usize) -> String {
    let fn_name = Regex::new(r"fn\s+(\w+)")
        .unwrap()
        .captures(snippet)
        .map_or_else(|| "solve".to_string(), |c| c[1].to_string());

    lines
        .chunks(param_count.max(1))
        .enumerate()
        .map(|(i, args)| {
            let args = args.iter().map(|a| rustify(a)).collect::<Vec<_>>().join(", ");
            let expected = outputs
                .get(i)
                .map(|o| rustify(o))
                .unwrap_or_else(|| "_".to_string());
            format!(
                "    #[test]\n    fn case_{}() {{\n        \
                 assert_eq!({expected}, Solution::{fn_name}({args}));\n    }}",
                i + 1,
            )
        })
        .collect::<Vec<_>>()
        .join("\n\n")
}

/// The starter function's name, parameters (`name`, `type`), and return type.
struct Signature {
    name: String,
    params: Vec<(String, String)>,
    ret: String,
}

/// Parse `fn name(p: T, ...) -> Ret {` out of the starter snippet. Returns
/// `None` if there's no `-> Ret` (e.g. an unparseable / unit-returning fn).
fn parse_signature(snippet: &str) -> Option<Signature> {
    let caps = Regex::new(r"fn\s+(\w+)\s*\(([^)]*)\)\s*->\s*([^{]+?)\s*\{")
        .unwrap()
        .captures(snippet)?;
    Some(Signature {
        name: caps[1].to_string(),
        params: split_params(caps[2].trim()),
        ret: caps[3].trim().to_string(),
    })
}

/// Split a parameter list on top-level commas (commas inside `<...>` are part of
/// a generic type, not separators), yielding `(name, type)` pairs.
fn split_params(s: &str) -> Vec<(String, String)> {
    let mut params = Vec::new();
    let mut depth = 0i32;
    let mut current = String::new();
    for c in s.chars() {
        match c {
            '<' | '(' | '[' => {
                depth += 1;
                current.push(c);
            }
            '>' | ')' | ']' => {
                depth -= 1;
                current.push(c);
            }
            ',' if depth == 0 => {
                push_param(&mut params, &current);
                current.clear();
            }
            _ => current.push(c),
        }
    }
    push_param(&mut params, &current);
    params
}

fn push_param(params: &mut Vec<(String, String)>, raw: &str) {
    if let Some((name, ty)) = raw.trim().split_once(':') {
        params.push((name.trim().to_string(), ty.trim().to_string()));
    }
}

/// Rewrite a LeetCode literal into Rust syntax. Currently the only fixup is
/// array literals: `[1,2]` -> `vec![1,2]`, including nested arrays. Characters
/// inside string literals are left untouched. Extend the `match` as new types
/// come up (e.g. turning string literals into `.to_string()`).
fn rustify(literal: &str) -> String {
    let mut out = String::with_capacity(literal.len() + 8);
    let mut in_string = false;
    for c in literal.chars() {
        match c {
            '"' => {
                in_string = !in_string;
                out.push(c);
            }
            '[' if !in_string => out.push_str("vec!["),
            _ => out.push(c),
        }
    }
    out
}

/// Type-directed fixups on a rustified literal. When the target type involves
/// `String`, append `.to_string()` to every `"..."` literal — so a `String`
/// param gets `"abc".to_string()` and a `Vec<String>` gets
/// `vec!["a".to_string(), "b".to_string()]`. Other types pass through unchanged.
fn coerce(value: String, ty: &str) -> String {
    if ty.contains("String") {
        Regex::new(r#""[^"]*""#)
            .unwrap()
            .replace_all(&value, |c: &regex::Captures| format!("{}.to_string()", &c[0]))
            .into_owned()
    } else {
        value
    }
}

/// Pull the `Output:` value out of each worked example in the (HTML) problem
/// description, in order, to use as expected values in the generated tests.
fn parse_outputs(content: &str) -> Vec<String> {
    let md = html2md::parse_html(content);
    Regex::new(r"(?im)^\s*\**\s*output\**\s*:\**\s*(.+?)\s*$")
        .unwrap()
        .captures_iter(&md)
        .map(|c| c[1].trim().trim_matches('`').trim().to_string())
        .collect()
}

fn cmd_new(root: &Path, slug: &str, language: Language) -> Result<()> {
    let question = leetcode::fetch_question(slug)?;

    let Some(snippet) = question.snippet(language.lang_slug()) else {
        bail!(
            "No {:?} starter code for '{slug}' (available: {})",
            language,
            question
                .code_snippets
                .iter()
                .map(|s| s.lang_slug.as_str())
                .collect::<Vec<_>>()
                .join(", ")
        );
    };

    // For now only Rust has a directory layout wired up.
    let (lang_dir, bin_dir) = match language {
        Language::Rust => {
            let lang_dir = root.join("rust");
            let bin_dir = lang_dir.join("src").join("bin");
            if !lang_dir.join("Cargo.toml").exists() {
                fs::create_dir_all(&lang_dir)?;
                Command::new("cargo")
                    .args(["init", "--lib", "--name", "lc", "--vcs", "none"])
                    .current_dir(&lang_dir)
                    .status()?;
                // rstest powers the parametrized test cases we generate.
                Command::new("cargo")
                    .args(["add", "rstest", "--dev"])
                    .current_dir(&lang_dir)
                    .status()?;
                fs::create_dir_all(&bin_dir)?;
            }
            (lang_dir, bin_dir)
        }
        Language::Python => bail!("Python scaffolding not implemented yet"),
    };
    let _ = lang_dir;

    let file_path = bin_dir.join(format!("{}.{}", file_stem(slug), language.ext()));
    if file_path.exists() {
        println!("{} already exists", file_path.display());
        return Ok(());
    }

    let tags = question
        .topic_tags
        .iter()
        .map(|t| t.name.as_str())
        .collect::<Vec<_>>()
        .join(" | ");

    // Render the HTML description as markdown, then prefix every line as a
    // file-level doc comment (blank lines become a bare `//!`).
    let description = match &question.content {
        Some(html) => wrap_prose(&clean_md(&html2md::parse_html(html)), 76)
            .lines()
            .map(|line| {
                if line.is_empty() {
                    "//!".to_string()
                } else {
                    format!("//! {line}")
                }
            })
            .collect::<Vec<_>>()
            .join("\n"),
        None => "//! (no public description)".to_string(),
    };

    let tests = generate_tests(&question, &snippet.code);

    let template = include_str!("../templates/rust.rs")
        .replace("{{number}}", &question.question_frontend_id)
        .replace("{{title}}", &question.title)
        .replace("{{difficulty}}", &question.difficulty)
        .replace("{{tags}}", &tags)
        .replace("{{slug}}", &question.title_slug)
        .replace("{{description}}", &description)
        .replace("{{snippet}}", &snippet.code)
        .replace("{{tests}}", &tests);

    fs::write(&file_path, template)?;
    println!("Created {}", file_path.display());
    Ok(())
}

/// Carve the LeetCode-submittable code out of a scaffolded file: everything
/// between the `struct Solution;` line (LeetCode supplies its own) and the
/// `fn main()` harness, dropping the doc-comment header and the test module.
/// Falls back to the whole file if the markers aren't found.
fn extract_solution(source: &str) -> String {
    const STRUCT: &str = "struct Solution;";
    let after_struct = match source.find(STRUCT) {
        Some(i) => &source[i + STRUCT.len()..],
        None => source,
    };
    let body = match after_struct.find("fn main(") {
        Some(i) => &after_struct[..i],
        None => after_struct,
    };
    body.trim().to_string()
}

fn cmd_submit(root: &Path, slug: &str, language: Language) -> Result<()> {
    let file_path = match language {
        Language::Rust => root.join("rust").join("src").join("bin").join(format!(
            "{}.{}",
            file_stem(slug),
            language.ext()
        )),
        Language::Python => bail!("Python submit not implemented yet"),
    };
    if !file_path.exists() {
        bail!(
            "No solution at {} — run `lc new {slug} {}` first",
            file_path.display(),
            language.lang_slug(),
        );
    }

    let source = fs::read_to_string(&file_path)?;
    let code = extract_solution(&source);

    let creds = leetcode::Credentials::load(root)?;
    let question = leetcode::fetch_question(slug)?;

    println!("Submitting {slug} ({})...", language.lang_slug());
    let id = leetcode::submit(
        &creds,
        slug,
        &question.question_id,
        language.lang_slug(),
        &code,
    )?;

    let result = leetcode::poll_submission(&creds, id)?;
    let correct = result.total_correct.unwrap_or(0);
    let total = result.total_testcases.unwrap_or(0);
    let status = result.status_msg.as_deref().unwrap_or("Unknown");

    println!();
    match status {
        "Accepted" => {
            println!("✅ Accepted — {correct}/{total} test cases passed");
            if let Some(rt) = &result.status_runtime {
                match result.runtime_percentile {
                    Some(p) => println!("   Runtime: {rt} (beats {p:.1}%)"),
                    None => println!("   Runtime: {rt}"),
                }
            }
            if let Some(mem) = &result.status_memory {
                match result.memory_percentile {
                    Some(p) => println!("   Memory:  {mem} (beats {p:.1}%)"),
                    None => println!("   Memory:  {mem}"),
                }
            }
        }
        "Wrong Answer" => {
            println!("❌ Wrong Answer — {correct}/{total} test cases passed");
            if let Some(tc) = &result.last_testcase {
                println!("   Input:    {}", tc.replace('\n', ", "));
            }
            if let Some(out) = &result.code_output {
                println!("   Output:   {out}");
            }
            if let Some(exp) = &result.expected_output {
                println!("   Expected: {exp}");
            }
        }
        "Compile Error" => {
            println!("❌ Compile Error");
            if let Some(err) = result.full_compile_error.or(result.compile_error) {
                println!("{err}");
            }
        }
        "Runtime Error" => {
            println!("❌ Runtime Error — {correct}/{total} test cases passed");
            if let Some(err) = result.full_runtime_error.or(result.runtime_error) {
                println!("{err}");
            }
        }
        other => {
            println!("❌ {other} — {correct}/{total} test cases passed");
        }
    }

    println!("\n   https://leetcode.com/submissions/detail/{id}/");
    Ok(())
}

fn main() -> Result<()> {
    let root = find_project_root()?;
    match Cli::parse().command {
        Commands::New { slug, language } => cmd_new(&root, &slug, language),
        Commands::Desc { slug } => cmd_desc(&root, &slug),
        Commands::Submit { slug, language } => cmd_submit(&root, &slug, language),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("[1,2,3]", "vec![1,2,3]")]
    #[case("[[1],[2,3]]", "vec![vec![1],vec![2,3]]")]
    #[case("9", "9")]
    // brackets inside a string literal are left alone
    #[case("\"a[b]\"", "\"a[b]\"")]
    fn rustify_arrays_to_vecs(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(rustify(input), expected);
    }

    #[rstest]
    #[case("\"abc\"", "String", "\"abc\".to_string()")]
    #[case("vec![\"a\",\"b\"]", "Vec<String>", "vec![\"a\".to_string(),\"b\".to_string()]")]
    // non-String types are untouched
    #[case("vec![1,2]", "Vec<i32>", "vec![1,2]")]
    #[case("5", "i32", "5")]
    fn coerce_string_types(#[case] value: &str, #[case] ty: &str, #[case] expected: &str) {
        assert_eq!(coerce(value.to_string(), ty), expected);
    }

    #[test]
    fn parse_outputs_in_order() {
        let html = "<pre><strong>Input:</strong> n = 2\n\
                    <strong>Output:</strong> 2</pre>\
                    <pre><strong>Output:</strong> [0,1,1]</pre>";
        assert_eq!(parse_outputs(html), vec!["2", "[0,1,1]"]);
    }

    #[test]
    fn signature_with_generic_params() {
        let snippet = "impl Solution {\n    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {\n    }\n}";
        let sig = parse_signature(snippet).unwrap();
        assert_eq!(sig.name, "two_sum");
        assert_eq!(sig.ret, "Vec<i32>");
        assert_eq!(
            sig.params,
            vec![
                ("nums".to_string(), "Vec<i32>".to_string()),
                ("target".to_string(), "i32".to_string()),
            ]
        );
    }
}
