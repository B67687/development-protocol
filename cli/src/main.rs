use clap::{Parser, Subcommand};
use std::fs;
use std::io::{self, Write};
use std::path::Path;
use std::process::Command;

/// Check if RULES.md has a heading matching `name`, handling numbered sections.
/// Matches `## Phase Definitions`, `## 3. Phase Definitions`, etc.
fn has_heading(content: &str, name: &str) -> bool {
    content.lines().any(|line| {
        let t = line.trim();
        if !t.starts_with("## ") {
            return false;
        }
        let text = t[3..].trim();
        // Strip optional number prefix like "3. "
        let text = text
            .strip_prefix(|c: char| c.is_ascii_digit())
            .and_then(|s| s.strip_prefix(". "))
            .unwrap_or(text);
        text.contains(name)
    })
}

fn render(template: &str, pairs: &[(&str, &str)]) -> String {
    pairs
        .iter()
        .fold(template.to_string(), |acc, (key, value)| {
            acc.replace(key, value)
        })
}

fn render_template(template: &str, name: &str, ptype: &str, lang: &str, scope: &str) -> String {
    render(
        template,
        &[
            ("{{PROJECT_NAME}}", name),
            ("{{PROJECT_TYPE}}", ptype),
            ("{{LANGUAGE}}", lang),
            ("{{SCOPE_DESCRIPTION}}", scope),
            ("{{PHASE}}", "DISCOVER"),
        ],
    )
}

/// Load a scaffold template from the repo `template/` dir at runtime, falling
/// back to the compile-time embedded copy (mirrors the RULES.md pattern).
fn load_template(name: &str) -> io::Result<String> {
    let template_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("template")
        .join(name);
    if template_path.exists() {
        fs::read_to_string(&template_path)
    } else {
        let fallback = match name {
            "editorconfig.tmpl" => include_str!("../../template/editorconfig.tmpl"),
            "gitignore.tmpl" => include_str!("../../template/gitignore.tmpl"),
            "changelog.tmpl" => include_str!("../../template/changelog.tmpl"),
            "ci.yml.tmpl" => include_str!("../../template/ci.yml.tmpl"),
            "release.yml.tmpl" => include_str!("../../template/release.yml.tmpl"),
            "glossary.tmpl" => include_str!("../../template/glossary.tmpl"),
            "what-is-this.tmpl" => include_str!("../../template/what-is-this.tmpl"),
            "scope-warp-log.tmpl" => include_str!("../../template/scope-warp-log.tmpl"),
            other => {
                return Err(io::Error::new(
                    io::ErrorKind::NotFound,
                    format!("Unknown template: {other}"),
                ));
            }
        };
        Ok(fallback.to_string())
    }
}

// ── Scaffold helpers ──────────────────────────────────────────────────────

struct LangCmd {
    aliases: &'static [&'static str],
    build: &'static str,
    test: &'static str,
    lint: &'static str,
}

const LANG_CMDS: &[LangCmd] = &[
    LangCmd {
        aliases: &["rust", "cargo"],
        build: "cargo build --all-features",
        test: "cargo test",
        lint: "cargo clippy --all-targets",
    },
    LangCmd {
        aliases: &["kotlin", "kt", "android"],
        build: "./gradlew assembleDebug",
        test: "./gradlew test",
        lint: "./gradlew lint",
    },
    LangCmd {
        aliases: &["typescript", "ts", "javascript", "js", "node", "bun"],
        build: "npm run build",
        test: "npm test",
        lint: "npx biome ci",
    },
    LangCmd {
        aliases: &["python", "py"],
        build: "python -m build",
        test: "python -m pytest",
        lint: "ruff check .",
    },
    LangCmd {
        aliases: &["go", "golang"],
        build: "go build ./...",
        test: "go test ./...",
        lint: "golangci-lint run",
    },
    LangCmd {
        aliases: &["csharp", "c#", "dotnet", ".net"],
        build: "dotnet build",
        test: "dotnet test",
        lint: "dotnet build -- TreatWarningsAsErrors",
    },
];

fn lang_cmds(lang: &str) -> &'static LangCmd {
    let l = lang.to_lowercase();
    LANG_CMDS
        .iter()
        .find(|c| c.aliases.contains(&l.as_str()))
        .unwrap_or(&LangCmd {
            aliases: &[],
            build: "[your build command]",
            test: "[your test command]",
            lint: "[your lint command]",
        })
}

fn scaffold_editorconfig(dir: &Path) -> io::Result<()> {
    let content = load_template("editorconfig.tmpl")?;
    fs::write(dir.join(".editorconfig"), content)?;
    println!("  Created .editorconfig");
    Ok(())
}

fn scaffold_gitignore(dir: &Path) -> io::Result<()> {
    let content = load_template("gitignore.tmpl")?;
    fs::write(dir.join(".gitignore"), content)?;
    println!("  Created .gitignore");
    Ok(())
}

fn scaffold_changelog(dir: &Path, name: &str) -> io::Result<()> {
    let content = render(&load_template("changelog.tmpl")?, &[("{{NAME}}", name)]);
    fs::write(dir.join("CHANGELOG.md"), content)?;
    println!("  Created CHANGELOG.md");
    Ok(())
}

fn scaffold_ci_workflow(dir: &Path, lang: &str) -> io::Result<()> {
    let workflows_dir = dir.join(".github").join("workflows");
    fs::create_dir_all(&workflows_dir)?;

    let cmds = lang_cmds(lang);
    let content = render(
        &load_template("ci.yml.tmpl")?,
        &[
            ("{{BUILD}}", cmds.build),
            ("{{TEST}}", cmds.test),
            ("{{LINT}}", cmds.lint),
        ],
    );
    fs::write(workflows_dir.join("ci.yml"), content)?;
    println!("  Created .github/workflows/ci.yml");
    Ok(())
}

fn scaffold_release_workflow(dir: &Path) -> io::Result<()> {
    let workflows_dir = dir.join(".github").join("workflows");
    fs::create_dir_all(&workflows_dir)?;

    let content = load_template("release.yml.tmpl")?;
    fs::write(workflows_dir.join("release.yml"), content)?;
    println!("  Created .github/workflows/release.yml");
    Ok(())
}

fn scaffold_docs_glossary(dir: &Path) -> io::Result<()> {
    let docs_dir = dir.join("docs");
    fs::create_dir_all(&docs_dir)?;

    let content = load_template("glossary.tmpl")?;
    fs::write(docs_dir.join("glossary.md"), content)?;
    println!("  Created docs/glossary.md");
    Ok(())
}

fn scaffold_what_is_this(dir: &Path, name: &str, ptype: &str, lang: &str) -> io::Result<()> {
    let docs_dir = dir.join("docs");
    fs::create_dir_all(&docs_dir)?;

    let content = render(
        &load_template("what-is-this.tmpl")?,
        &[
            ("{{NAME}}", name),
            ("{{PROJECT_TYPE}}", ptype),
            ("{{LANGUAGE}}", lang),
        ],
    );
    fs::write(docs_dir.join("what-is-this.md"), content)?;
    println!("  Created docs/what-is-this.md");
    Ok(())
}

fn scaffold_scope_warp_log(dir: &Path) -> io::Result<()> {
    let content = load_template("scope-warp-log.tmpl")?;
    fs::write(dir.join("scope-warp-log.md"), content)?;
    println!("  Created scope-warp-log.md");
    Ok(())
}

// ── CLI ───────────────────────────────────────────────────────────────────

/// Project Bootstrap Protocol CLI
#[derive(Parser)]
#[command(
    name = "project-kit",
    version,
    about = "Bootstrap, govern, and evolve projects with the Development Protocol"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize a new project with RULES.md governance
    Init {
        /// Project name (default: current directory name)
        #[arg(short = 'n', long)]
        name: Option<String>,
        /// Project type: standard, discover-first, ux-first, port, explore-only, maintenance
        #[arg(short, long)]
        project_type: Option<String>,
        /// Language/tech stack
        #[arg(short, long)]
        language: Option<String>,
        /// V1 scope description
        #[arg(short, long)]
        scope: Option<String>,
    },
    /// Transition to a new phase
    Phase {
        /// Target phase: discover, work, iterate, perfect, distribute
        #[arg(short, long)]
        set: Option<String>,
        /// Check current phase without changing
        #[arg(long)]
        status: bool,
    },
    /// Validate RULES.md is complete and correct
    Check,
    /// Squash working tree into a single commit and force-push to GitHub main
    Publish {
        /// Commit message for the squashed release
        #[arg(short, long)]
        message: String,
        /// Optional version tag (e.g. "v1.0.0")
        #[arg(short, long)]
        tag: Option<String>,
    },
}

fn main() -> io::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init {
            name,
            project_type,
            language,
            scope,
        } => {
            cmd_init(name, project_type, language, scope)?;
        }
        Commands::Phase { set, status } => {
            cmd_phase(set, status)?;
        }
        Commands::Check => {
            cmd_check()?;
        }
        Commands::Publish { message, tag } => {
            cmd_publish(&message, tag.as_deref())?;
        }
    }

    Ok(())
}

fn prompt(prompt_text: &str, default: Option<&str>) -> String {
    let mut input = String::new();
    match default {
        Some(d) => print!("{} [{}]: ", prompt_text, d),
        None => print!("{}: ", prompt_text),
    }
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    let trimmed = input.trim().to_string();
    if trimmed.is_empty() {
        default.unwrap_or("").to_string()
    } else {
        trimmed
    }
}

fn cmd_init(
    name: Option<String>,
    project_type: Option<String>,
    language: Option<String>,
    scope: Option<String>,
) -> io::Result<()> {
    let current_dir = std::env::current_dir()?;
    let rules_path = current_dir.join("RULES.md");

    if rules_path.exists() {
        eprintln!(
            "RULES.md already exists in this directory. Use `project-kit phase` to manage phases."
        );
        return Ok(());
    }

    println!("=== Project Bootstrap ===");

    // Derive project name from directory if not provided
    let dir_name = current_dir
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_else(|| "my-project".to_string());
    let pname = name.unwrap_or_else(|| prompt("Project name", Some(&dir_name)));
    let ptype = project_type.unwrap_or_else(|| {
        prompt(
            "Project type (standard / discover-first / ux-first / port / explore-only / maintenance)",
            Some("standard"),
        )
    });
    let lang = language.unwrap_or_else(|| prompt("Language / tech stack", Some("rust")));
    let scope_description =
        scope.unwrap_or_else(|| prompt("V1 scope (short description)", Some("my project")));

    // Read the template RULES.md and render placeholders
    let template_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("template")
        .join("RULES.md");
    let template = if template_path.exists() {
        fs::read_to_string(&template_path)?
    } else {
        include_str!("../../RULES.md").to_string()
    };
    let rendered = render_template(&template, &pname, &ptype, &lang, &scope_description);

    fs::write(&rules_path, &rendered)?;
    println!("  Created RULES.md");

    // Create AGENTS.md
    let agents = format!(
        "# AGENTS.md\n\n\
         This project follows the **Development Protocol** defined in `RULES.md`.\n\
         Every AI must read that file first before starting any work.\n\n\
         ## Project\n\n\
         - **Name:** {}\n\
         - **Type:** {}\n\
         - **Language:** {}\n\
         - **V1 Scope:** {}\n\n\
         ## Startup\n\n\
         ```\n\
         Read RULES.md.\n\
         State current phase, scope, and Constitution principles.\n\
         Check stop rules.\n\
         Proceed.\n\
         ```\n",
        pname, ptype, lang, scope_description
    );
    fs::write(current_dir.join("AGENTS.md"), &agents)?;
    println!("  Created AGENTS.md");

    // Create CLAUDE.md
    let claude = "@AGENTS.md\n@RULES.md\n\n# Claude-specific additions\n";
    fs::write(current_dir.join("CLAUDE.md"), claude)?;
    println!("  Created CLAUDE.md");

    // Scaffold supporting files
    scaffold_editorconfig(&current_dir)?;
    scaffold_gitignore(&current_dir)?;
    scaffold_changelog(&current_dir, &pname)?;
    scaffold_ci_workflow(&current_dir, &lang)?;
    scaffold_release_workflow(&current_dir)?;
    scaffold_docs_glossary(&current_dir)?;
    scaffold_what_is_this(&current_dir, &pname, &ptype, &lang)?;
    scaffold_scope_warp_log(&current_dir)?;

    println!("\nProject scaffolded. Next steps:");
    println!("  1. Edit RULES.md: set your V1 scope, Constitution, and AI persona");
    println!("  2. Set the phase: `project-kit phase --set work`");
    println!(
        "  3. Initialize git: `git init && git add -A && git commit -m \"chore: initial scaffold\"`"
    );
    println!("  4. Start your AI session by having it read RULES.md");

    Ok(())
}

fn restore_branch(branch: &str, temp_branch: &str) {
    let _ = Command::new("git").args(["checkout", branch]).status();
    let _ = Command::new("git")
        .args(["branch", "-D", temp_branch])
        .status();
}

fn cmd_publish(message: &str, tag: Option<&str>) -> io::Result<()> {
    // Verify we're in a git repo
    if !Path::new(".git").exists() {
        eprintln!("FAIL: No .git directory found. Run `git init` first.");
        return Err(io::Error::other(
            "FAIL: No .git directory found. Run `git init` first.",
        ));
    }

    // Save current branch name
    let branch_name = String::from_utf8_lossy(
        &Command::new("git")
            .args(["rev-parse", "--abbrev-ref", "HEAD"])
            .output()
            .map_err(io::Error::other)?
            .stdout,
    )
    .trim()
    .to_string();

    println!("Publishing from branch: {}", branch_name);

    // Create orphan branch
    let temp_branch = format!("publish-temp-{}", std::process::id());
    let status = Command::new("git")
        .args(["checkout", "--orphan", &temp_branch])
        .status()
        .map_err(io::Error::other)?;
    if !status.success() {
        eprintln!("FAIL: Could not create orphan branch.");
        return Err(io::Error::other("FAIL: Could not create orphan branch."));
    }

    // Add all files
    let status = Command::new("git")
        .args(["add", "-A"])
        .status()
        .map_err(io::Error::other)?;
    if !status.success() {
        eprintln!("FAIL: Could not add files.");
        return Err(io::Error::other("FAIL: Could not add files."));
    }

    // Commit squashed
    let status = Command::new("git")
        .args(["commit", "-m", message])
        .status()
        .map_err(io::Error::other)?;
    if !status.success() {
        eprintln!("FAIL: Could not create commit. Check git config (user.name, user.email).");
        // Go back to original branch
        restore_branch(&branch_name, &temp_branch);
        return Err(io::Error::other(
            "FAIL: Could not create commit. Check git config (user.name, user.email).",
        ));
    }

    // Force-with-lease push to main
    println!("Force-pushing to origin/main...");
    let status = Command::new("git")
        .args([
            "push",
            "origin",
            &format!("{}:main", temp_branch),
            "--force-with-lease",
        ])
        .status()
        .map_err(io::Error::other)?;
    if !status.success() {
        eprintln!("FAIL: Push failed. Is 'origin' set up correctly?");
        restore_branch(&branch_name, &temp_branch);
        return Err(io::Error::other(
            "FAIL: Push failed. Is 'origin' set up correctly?",
        ));
    }

    // Tag if requested
    if let Some(tag_name) = tag {
        let status = Command::new("git")
            .args(["tag", tag_name])
            .status()
            .map_err(io::Error::other)?;
        if status.success() {
            let _ = Command::new("git")
                .args(["push", "origin", tag_name])
                .status();
            println!("  Tagged: {}", tag_name);
        }
    }

    // Return to original branch and clean up
    restore_branch(&branch_name, &temp_branch);

    println!("Published! GitHub main now has 1 commit.");
    println!("  Message: {}", message);
    println!(
        "  Local branch '{}' restored with full history.",
        branch_name
    );
    Ok(())
}

fn cmd_phase(set: Option<String>, status: bool) -> io::Result<()> {
    let current_dir = std::env::current_dir()?;
    let rules_path = current_dir.join("RULES.md");

    if !rules_path.exists() {
        eprintln!("No RULES.md found. Run `project-kit init` first.");
        return Ok(());
    }

    let content = fs::read_to_string(&rules_path)?;

    if status || set.is_none() {
        // Show current phase
        if let Some(line) = content.lines().find(|l| l.contains("**Current:")) {
            println!("{}", line.trim());
        } else {
            println!("Phase not set in RULES.md. Edit the `## Phase` section.");
        }
        return Ok(());
    }

    if let Some(new_phase) = set {
        let phase_upper = new_phase.to_uppercase();
        let valid = ["DISCOVER", "WORK", "ITERATE", "PERFECT", "DISTRIBUTE"];

        if !valid.contains(&phase_upper.as_str()) {
            eprintln!(
                "Invalid phase: {}. Valid: discover, work, iterate, perfect, distribute",
                new_phase
            );
            return Ok(());
        }

        // Update the phase line (simple replace for v0.1.0)
        let new_content = if content.contains("**Current:") {
            content.replace(
                content
                    .lines()
                    .find(|l| l.contains("**Current:"))
                    .unwrap_or("**Current:** `WORK`"),
                &format!("**Current:** `{}`", phase_upper),
            )
        } else {
            // Add phase line after the first heading
            content.replace(
                "Read this at the START of every AI session.",
                &format!(
                    "Read this at the START of every AI session.\n\n**Current:** `{}`",
                    phase_upper
                ),
            )
        };

        fs::write(&rules_path, &new_content)?;
        println!("Phase set to: {}", phase_upper);

        // Run phase exit reflection if moving *out* of a phase
        println!(
            "\nTip: Run the Phase Exit Checklist (Section 9 in RULES.md) if you're leaving a completed phase."
        );
    }

    Ok(())
}

fn cmd_check() -> io::Result<()> {
    let current_dir = std::env::current_dir()?;
    let rules_path = current_dir.join("RULES.md");

    if !rules_path.exists() {
        eprintln!("FAIL: No RULES.md found. Run `project-kit init` first.");
        return Ok(());
    }

    let content = fs::read_to_string(&rules_path)?;
    let mut issues = Vec::new();

    // Check each required section using flexible heading matching
    if !has_heading(&content, "Phase") {
        issues.push("Missing Phase section");
    }
    if !content.contains("**Current:") {
        issues.push("Missing current phase marker (**Current: ...)");
    }
    if !has_heading(&content, "Constitution") {
        issues.push("Missing Constitution section");
    }
    if !has_heading(&content, "V1 Scope") && !has_heading(&content, "Scope") {
        issues.push("Missing V1 Scope section");
    }
    if !content.contains("IN SCOPE") {
        issues.push("Missing IN SCOPE list");
    }
    if !content.contains("OUT OF SCOPE") {
        issues.push("Missing OUT OF SCOPE list");
    }
    if !has_heading(&content, "Stop Rules") {
        issues.push("Missing Stop Rules section");
    }
    if !has_heading(&content, "AI Persona") && !has_heading(&content, "Persona") {
        issues.push("Missing AI Persona section");
    }
    if !has_heading(&content, "Verification Gates") {
        issues.push("Missing Verification Gates section");
    }
    if !has_heading(&content, "Test Philosophy") {
        issues.push("Missing Test Philosophy section");
    }

    if issues.is_empty() {
        println!("PASS: RULES.md is complete and well-formed.");
    } else {
        println!("FAIL: {} issue(s) found:", issues.len());
        for issue in &issues {
            println!("  - {}", issue);
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn has_heading_matches_phase_sections() {
        assert!(has_heading("## Phase Definitions\n", "Phase Definitions"));
        assert!(has_heading(
            "## 3. Phase Definitions\n",
            "Phase Definitions"
        ));
        assert!(!has_heading("### Sub\n", "Sub"));
    }

    #[test]
    fn render_replaces_known_markers_and_keeps_unknown() {
        let out = render("a {{X}} b {{Y}} c", &[("{{X}}", "1"), ("{{Y}}", "2")]);
        assert_eq!(out, "a 1 b 2 c");
        let out = render("a {{X}} b {{Y}}", &[("{{X}}", "1")]);
        assert_eq!(out, "a 1 b {{Y}}");
    }

    #[test]
    fn lang_cmds_lookup() {
        let rust = lang_cmds("rust");
        assert_eq!(rust.build, "cargo build --all-features");
        assert_eq!(rust.test, "cargo test");
        assert_eq!(rust.lint, "cargo clippy --all-targets");
        let py = lang_cmds("python");
        assert_eq!(py.test, "python -m pytest");
        assert_eq!(py.lint, "ruff check .");
        let node = lang_cmds("node");
        assert_eq!(node.lint, "npx biome ci");
        let unknown = lang_cmds("cobol");
        assert_eq!(unknown.build, "[your build command]");
        assert_eq!(unknown.test, "[your test command]");
        assert_eq!(unknown.lint, "[your lint command]");
        let upper = lang_cmds("Rust");
        assert_eq!(upper.build, "cargo build --all-features");
    }

    #[test]
    fn ci_workflow_uses_fixed_phase_grep() {
        let rendered = render(
            &load_template("ci.yml.tmpl").unwrap(),
            &[("{{BUILD}}", "b"), ("{{TEST}}", "t"), ("{{LINT}}", "l")],
        );
        assert!(rendered.contains(r"`(DISCOVER|WORK|ITERATE|PERFECT|DISTRIBUTE)`"));
        assert!(!rendered.contains(r"Current: WORK\|"));
    }

    #[test]
    fn what_is_this_renders_all_markers() {
        let rendered = render(
            &load_template("what-is-this.tmpl").unwrap(),
            &[
                ("{{NAME}}", "demo"),
                ("{{PROJECT_TYPE}}", "standard"),
                ("{{LANGUAGE}}", "rust"),
            ],
        );
        assert!(!rendered.contains("{{"));
        assert!(rendered.contains("# What Is demo?"));
        assert!(rendered.contains("**demo** is a standard project built with rust."));
    }
}
