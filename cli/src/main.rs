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

fn render_template(template: &str, name: &str, ptype: &str, lang: &str, scope: &str) -> String {
    template
        .replace("{{PROJECT_NAME}}", name)
        .replace("{{PROJECT_TYPE}}", ptype)
        .replace("{{LANGUAGE}}", lang)
        .replace("{{SCOPE_DESCRIPTION}}", scope)
        .replace("{{PHASE}}", "DISCOVER")
}

// ── Scaffold helpers ──────────────────────────────────────────────────────

fn build_command_for(lang: &str) -> &str {
    match lang.to_lowercase().as_str() {
        "rust" | "cargo" => "cargo build --all-features",
        "kotlin" | "kt" | "android" => "./gradlew assembleDebug",
        "typescript" | "ts" | "javascript" | "js" | "node" | "bun" => "npm run build",
        "python" | "py" => "python -m build",
        "go" | "golang" => "go build ./...",
        "csharp" | "c#" | "dotnet" | ".net" => "dotnet build",
        _ => "[your build command]",
    }
}

fn test_command_for(lang: &str) -> &str {
    match lang.to_lowercase().as_str() {
        "rust" | "cargo" => "cargo test",
        "kotlin" | "kt" | "android" => "./gradlew test",
        "typescript" | "ts" | "javascript" | "js" | "node" | "bun" => "npm test",
        "python" | "py" => "python -m pytest",
        "go" | "golang" => "go test ./...",
        "csharp" | "c#" | "dotnet" | ".net" => "dotnet test",
        _ => "[your test command]",
    }
}

fn lint_command_for(lang: &str) -> &str {
    match lang.to_lowercase().as_str() {
        "rust" | "cargo" => "cargo clippy --all-targets",
        "kotlin" | "kt" | "android" => "./gradlew lint",
        "typescript" | "ts" | "javascript" | "js" => "npx biome ci",
        "python" | "py" => "ruff check .",
        "go" | "golang" => "golangci-lint run",
        "csharp" | "c#" | "dotnet" | ".net" => "dotnet build -- TreatWarningsAsErrors",
        _ => "[your lint command]",
    }
}

fn scaffold_editorconfig(dir: &Path) -> io::Result<()> {
    let content = "root = true

[*]
indent_style = space
indent_size = 4
end_of_line = lf
charset = utf-8
trim_trailing_whitespace = true
insert_final_newline = true

[*.md]
trim_trailing_whitespace = false

[*.{yml,yaml}]
indent_size = 2
";
    fs::write(dir.join(".editorconfig"), content)?;
    println!("  Created .editorconfig");
    Ok(())
}

fn scaffold_gitignore(dir: &Path) -> io::Result<()> {
    let content = "# Dependencies
/ vendor/
node_modules/
target/
build/
dist/

# IDE
.idea/
*.iml
.vscode/
*.swp
*.swo

# OS
.DS_Store
Thumbs.db

# Build artifacts
*.log
*.tmp
*.cache
";
    fs::write(dir.join(".gitignore"), content)?;
    println!("  Created .gitignore");
    Ok(())
}

fn scaffold_changelog(dir: &Path, name: &str) -> io::Result<()> {
    let content = format!(
        "# Changelog

All notable changes to {name} will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/),
and this project adheres to [Semantic Versioning](https://semver.org/).

## [Unreleased]

### Added

- Initial project scaffolded via Development Protocol

### Changed

### Fixed

### Removed
"
    );
    fs::write(dir.join("CHANGELOG.md"), content)?;
    println!("  Created CHANGELOG.md");
    Ok(())
}

fn scaffold_ci_workflow(dir: &Path, lang: &str) -> io::Result<()> {
    let workflows_dir = dir.join(".github").join("workflows");
    fs::create_dir_all(&workflows_dir)?;

    let build = build_command_for(lang);
    let test = test_command_for(lang);
    let lint = lint_command_for(lang);

    let content = format!(
        "name: CI

on:
  push:
    branches: [main]
  pull_request:
    branches: [main]

jobs:
  check:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      # Phase compliance: verify RULES.md phase is set
      - name: Check RULES.md phase
        run: |
          if ! grep -q \"Current: WORK\\|Current: PERFECT\\|Current: DISTRIBUTE\\|Current: ITERATE\\|Current: DISCOVER\" RULES.md; then
            echo \"ERROR: RULES.md phase not set.\"
            exit 1
          fi

      - name: Build
        run: {build}

      - name: Lint
        run: {lint}

      - name: Test
        run: {test}
"
    );
    fs::write(workflows_dir.join("ci.yml"), content)?;
    println!("  Created .github/workflows/ci.yml");
    Ok(())
}

fn scaffold_release_workflow(dir: &Path) -> io::Result<()> {
    let workflows_dir = dir.join(".github").join("workflows");
    fs::create_dir_all(&workflows_dir)?;

    let content = r#"name: Release

on:
  push:
    tags: ["v*"]

jobs:
  release:
    runs-on: ubuntu-latest
    permissions:
      contents: write
    steps:
      - uses: actions/checkout@v4
      - name: Build
        run: [your build command]
      - name: Generate release notes
        run: |
          VERSION="${GITHUB_REF_NAME#v}"
          sed -n "/^## \[$VERSION\]/,/^## \[/p" CHANGELOG.md | head -n -2 > release-notes.md
      - name: Create Release
        uses: softprops/action-gh-release@v2
        with:
          body_path: release-notes.md
          files: |
            [path to your build artifact]
"#;
    fs::write(workflows_dir.join("release.yml"), content)?;
    println!("  Created .github/workflows/release.yml");
    Ok(())
}

fn scaffold_docs_glossary(dir: &Path) -> io::Result<()> {
    let docs_dir = dir.join("docs");
    fs::create_dir_all(&docs_dir)?;

    let content = "# Glossary

Key terms and concepts used in this project.

| Term | Definition |
|------|------------|
|      |            |

_Add terms as you encounter them during development._
";
    fs::write(docs_dir.join("glossary.md"), content)?;
    println!("  Created docs/glossary.md");
    Ok(())
}

fn scaffold_what_is_this(dir: &Path, name: &str) -> io::Result<()> {
    let docs_dir = dir.join("docs");
    fs::create_dir_all(&docs_dir)?;

    let content = format!("# What Is {name}?

**{name}** is a {{{{PROJECT_TYPE}}}} project built with {{{{LANGUAGE}}}}.

## Problem

_What gap does this project fill? Why does it exist?_

## Architecture

_How is the project structured? What are the main components?_

## Status

_Current phase, what's working, what's next._

---

_This file should answer \"what is this project?\" for anyone — including future you — in under 2 minutes._
");
    fs::write(docs_dir.join("what-is-this.md"), content)?;
    println!("  Created docs/what-is-this.md");
    Ok(())
}

fn scaffold_scope_warp_log(dir: &Path) -> io::Result<()> {
    let content = "# Scope Warp Log

Track conscious scope expansions here. Max 3 warps per project.

## Warp 1

- **Feature added:**
- **Rationale:**
- **Cost:**
- **Deferred from V2:**
- **Date:**

---

## Warp 2

- **Feature added:**
- **Rationale:**
- **Cost:**
- **Deferred from V2:**
- **Date:**

---

## Warp 3

- **Feature added:**
- **Rationale:**
- **Cost:**
- **Deferred from V2:**
- **Date:**

---

*If you need more than 3 warps, force-enter PERFECT phase.*
";
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
        #[arg(short, long)]
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
    fs::write(current_dir.join("CLAUDE.md"), &claude)?;
    println!("  Created CLAUDE.md");

    // Scaffold supporting files
    scaffold_editorconfig(&current_dir)?;
    scaffold_gitignore(&current_dir)?;
    scaffold_changelog(&current_dir, &pname)?;
    scaffold_ci_workflow(&current_dir, &lang)?;
    scaffold_release_workflow(&current_dir)?;
    scaffold_docs_glossary(&current_dir)?;
    scaffold_what_is_this(&current_dir, &pname)?;
    scaffold_scope_warp_log(&current_dir)?;

    println!("\nProject scaffolded. Next steps:");
    println!("  1. Edit RULES.md: set your V1 scope, Constitution, and AI persona");
    println!("  2. Set the phase: `project-kit phase --set work`");
    println!("  3. Initialize git: `git init && git add -A && git commit -m \"chore: initial scaffold\"`");
    println!("  4. Start your AI session by having it read RULES.md");

    Ok(())
}

fn cmd_publish(message: &str, tag: Option<&str>) -> io::Result<()> {
    // Verify we're in a git repo
    if !Path::new(".git").exists() {
        eprintln!("FAIL: No .git directory found. Run `git init` first.");
        return Ok(());
    }

    // Save current branch name
    let branch_name = String::from_utf8_lossy(
        &Command::new("git")
            .args(["rev-parse", "--abbrev-ref", "HEAD"])
            .output()
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?
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
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    if !status.success() {
        eprintln!("FAIL: Could not create orphan branch.");
        return Ok(());
    }

    // Add all files
    let status = Command::new("git")
        .args(["add", "-A"])
        .status()
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    if !status.success() {
        eprintln!("FAIL: Could not add files.");
        return Ok(());
    }

    // Commit squashed
    let status = Command::new("git")
        .args(["commit", "-m", message])
        .status()
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    if !status.success() {
        eprintln!("FAIL: Could not create commit. Check git config (user.name, user.email).");
        // Go back to original branch
        let _ = Command::new("git")
            .args(["checkout", &branch_name])
            .status();
        let _ = Command::new("git")
            .args(["branch", "-D", &temp_branch])
            .status();
        return Ok(());
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
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    if !status.success() {
        eprintln!("FAIL: Push failed. Is 'origin' set up correctly?");
        let _ = Command::new("git")
            .args(["checkout", &branch_name])
            .status();
        let _ = Command::new("git")
            .args(["branch", "-D", &temp_branch])
            .status();
        return Ok(());
    }

    // Tag if requested
    if let Some(tag_name) = tag {
        let status = Command::new("git")
            .args(["tag", tag_name])
            .status()
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        if status.success() {
            let _ = Command::new("git")
                .args(["push", "origin", tag_name])
                .status();
            println!("  Tagged: {}", tag_name);
        }
    }

    // Return to original branch and clean up
    let _ = Command::new("git")
        .args(["checkout", &branch_name])
        .status();
    let _ = Command::new("git")
        .args(["branch", "-D", &temp_branch])
        .status();

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
                &content
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
        println!("\nTip: Run the Phase Exit Checklist (Section 9 in RULES.md) if you're leaving a completed phase.");
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
