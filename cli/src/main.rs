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

        let is_prose = !in_code
            && !line.is_empty()
            && !trimmed.starts_with(['*', '-', '>', '#', '|'])
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

/// Build one `#[test] fn case_N()` per example. Inputs come from
/// `example_testcases` (grouped by param count); the expected value is left as
/// a `0` placeholder for the user to fill from the description's `Output:` lines.
fn generate_tests(question: &leetcode::Question, snippet: &str) -> String {
    let fn_name = Regex::new(r"fn\s+(\w+)")
        .unwrap()
        .captures(snippet)
        .map_or_else(|| "solve".to_string(), |c| c[1].to_string());

    let lines: Vec<&str> = question.example_testcases.lines().collect();

    lines
        .chunks(question.param_count())
        .enumerate()
        .map(|(i, args)| {
            format!(
                "    #[test]\n    fn case_{}() {{\n        \
                 assert_eq!(0, Solution::{fn_name}({}));\n    }}",
                i + 1,
                args.join(", "),
            )
        })
        .collect::<Vec<_>>()
        .join("\n\n")
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
        Language::Rust => root
            .join("rust")
            .join("src")
            .join("bin")
            .join(format!("{}.{}", file_stem(slug), language.ext())),
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
