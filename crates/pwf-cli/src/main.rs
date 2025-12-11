use clap::{Parser, Subcommand, ValueEnum};
use colored::*;
use std::fs;
use std::path::PathBuf;
use std::process::ExitCode;

#[derive(Parser)]
#[command(name = "pwf")]
#[command(author, version, about = "Portable Workout Format validator", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Validate PWF plan files
    Validate {
        /// Plan files to validate
        #[arg(required = true)]
        files: Vec<PathBuf>,

        /// Output format
        #[arg(short, long, value_enum, default_value = "pretty")]
        format: OutputFormat,

        /// Treat warnings as errors
        #[arg(short, long)]
        strict: bool,

        /// Only show errors (suppress warnings)
        #[arg(short, long)]
        quiet: bool,
    },

    /// Validate PWF history export files
    History {
        /// History files to validate
        #[arg(required = true)]
        files: Vec<PathBuf>,

        /// Output format
        #[arg(short, long, value_enum, default_value = "pretty")]
        format: OutputFormat,

        /// Treat warnings as errors
        #[arg(short, long)]
        strict: bool,
    },

    /// Show specification version info
    Info,

    /// Generate a new plan from template
    Init {
        /// Output file path
        #[arg(default_value = "plan.yaml")]
        output: PathBuf,

        /// Generate a history export template instead
        #[arg(long)]
        history: bool,
    },
}

#[derive(Copy, Clone, PartialEq, Eq, ValueEnum)]
enum OutputFormat {
    Pretty,
    Json,
    Compact,
}

fn main() -> ExitCode {
    let cli = Cli::parse();

    match cli.command {
        Commands::Validate {
            files,
            format,
            strict,
            quiet,
        } => validate_plans(&files, format, strict, quiet),
        Commands::History {
            files,
            format,
            strict,
        } => validate_history(&files, format, strict),
        Commands::Info => {
            println!("{}", "PWF - Portable Workout Format".bold());
            println!();
            println!("Specification version: {}", "1.0".cyan());
            println!(
                "Validator version:     {}",
                env!("CARGO_PKG_VERSION").cyan()
            );
            println!();
            println!("{}", "Supported formats:".bold());
            println!("  {} - Workout plan templates", "plan".green());
            println!("  {} - Workout history exports", "history".green());
            println!();
            println!("{}", "Modalities:".bold());
            println!("  {} - Sets × reps training", "strength".yellow());
            println!("  {} - Fixed duration timer", "countdown".yellow());
            println!("  {} - Open-ended timing", "stopwatch".yellow());
            println!("  {} - Repeating work periods", "interval".yellow());
            println!();
            println!("Documentation: {}", "https://pwf.dev".underline());
            ExitCode::SUCCESS
        }
        Commands::Init { output, history } => {
            if history {
                init_history(&output)
            } else {
                init_plan(&output)
            }
        }
    }
}

fn validate_plans(files: &[PathBuf], format: OutputFormat, strict: bool, quiet: bool) -> ExitCode {
    let mut all_valid = true;
    let mut results = Vec::new();

    for path in files {
        let content = match fs::read_to_string(path) {
            Ok(c) => c,
            Err(e) => {
                eprintln!("{}: {}", path.display().to_string().red(), e);
                all_valid = false;
                continue;
            }
        };

        let result = pwf_core::plan::validate(&content);
        let is_valid = result.valid && (!strict || result.warnings.is_empty());

        if !is_valid {
            all_valid = false;
        }

        results.push((path.clone(), result));
    }

    output_plan_results(&results, format, strict, quiet);

    if all_valid {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}

fn validate_history(files: &[PathBuf], format: OutputFormat, strict: bool) -> ExitCode {
    let mut all_valid = true;
    let mut results = Vec::new();

    for path in files {
        let content = match fs::read_to_string(path) {
            Ok(c) => c,
            Err(e) => {
                eprintln!("{}: {}", path.display().to_string().red(), e);
                all_valid = false;
                continue;
            }
        };

        let result = pwf_core::history::validate(&content);
        let is_valid = result.valid && (!strict || result.warnings.is_empty());

        if !is_valid {
            all_valid = false;
        }

        results.push((path.clone(), result));
    }

    output_history_results(&results, format, strict);

    if all_valid {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}

fn output_plan_results(
    results: &[(PathBuf, pwf_core::plan::ValidationResult)],
    format: OutputFormat,
    strict: bool,
    quiet: bool,
) {
    match format {
        OutputFormat::Json => {
            let output: Vec<_> = results
                .iter()
                .map(|(path, result)| {
                    serde_json::json!({
                        "file": path.display().to_string(),
                        "type": "plan",
                        "valid": result.valid && (!strict || result.warnings.is_empty()),
                        "errors": result.errors,
                        "warnings": result.warnings,
                        "statistics": result.statistics,
                    })
                })
                .collect();
            println!("{}", serde_json::to_string_pretty(&output).unwrap());
        }
        OutputFormat::Compact => {
            for (path, result) in results {
                let status = if result.valid && (!strict || result.warnings.is_empty()) {
                    "✓".green()
                } else {
                    "✗".red()
                };
                println!("{} {}", status, path.display());
            }
        }
        OutputFormat::Pretty => {
            for (path, result) in results {
                let is_valid = result.valid && (!strict || result.warnings.is_empty());

                if is_valid {
                    println!("{} {}", "✓".green().bold(), path.display());

                    if let Some(ref stats) = result.statistics {
                        println!(
                            "  {} days, {} exercises",
                            stats.total_days.to_string().cyan(),
                            stats.total_exercises.to_string().cyan()
                        );
                    }

                    if !quiet {
                        for warning in &result.warnings {
                            println!(
                                "  {} {}: {}",
                                "⚠".yellow(),
                                warning.path.dimmed(),
                                warning.message.yellow()
                            );
                        }
                    }
                } else {
                    println!("{} {}", "✗".red().bold(), path.display());

                    for error in &result.errors {
                        println!(
                            "  {} {}: {}",
                            "✗".red(),
                            if error.path.is_empty() {
                                "(root)".to_string()
                            } else {
                                error.path.clone()
                            }
                            .dimmed(),
                            error.message.red()
                        );
                    }

                    if strict {
                        for warning in &result.warnings {
                            println!(
                                "  {} {}: {}",
                                "⚠".yellow(),
                                warning.path.dimmed(),
                                warning.message.yellow()
                            );
                        }
                    }
                }
                println!();
            }
        }
    }
}

fn output_history_results(
    results: &[(PathBuf, pwf_core::history::ValidationResult)],
    format: OutputFormat,
    strict: bool,
) {
    match format {
        OutputFormat::Json => {
            let output: Vec<_> = results
                .iter()
                .map(|(path, result)| {
                    serde_json::json!({
                        "file": path.display().to_string(),
                        "type": "history",
                        "valid": result.valid && (!strict || result.warnings.is_empty()),
                        "errors": result.errors,
                        "warnings": result.warnings,
                        "statistics": result.statistics,
                    })
                })
                .collect();
            println!("{}", serde_json::to_string_pretty(&output).unwrap());
        }
        OutputFormat::Compact => {
            for (path, result) in results {
                let status = if result.valid && (!strict || result.warnings.is_empty()) {
                    "✓".green()
                } else {
                    "✗".red()
                };
                println!("{} {}", status, path.display());
            }
        }
        OutputFormat::Pretty => {
            for (path, result) in results {
                let is_valid = result.valid && (!strict || result.warnings.is_empty());

                if is_valid {
                    println!("{} {}", "✓".green().bold(), path.display());

                    if let Some(ref stats) = result.statistics {
                        println!(
                            "  {} workouts, {} sets, {:.0} kg total volume",
                            stats.total_workouts.to_string().cyan(),
                            stats.total_sets.to_string().cyan(),
                            stats.total_volume_kg
                        );
                        if let (Some(start), Some(end)) =
                            (&stats.date_range_start, &stats.date_range_end)
                        {
                            println!("  Date range: {} to {}", start.cyan(), end.cyan());
                        }
                    }

                    for warning in &result.warnings {
                        println!(
                            "  {} {}: {}",
                            "⚠".yellow(),
                            warning.path.dimmed(),
                            warning.message.yellow()
                        );
                    }
                } else {
                    println!("{} {}", "✗".red().bold(), path.display());

                    for error in &result.errors {
                        println!(
                            "  {} {}: {}",
                            "✗".red(),
                            if error.path.is_empty() {
                                "(root)".to_string()
                            } else {
                                error.path.clone()
                            }
                            .dimmed(),
                            error.message.red()
                        );
                    }
                }
                println!();
            }
        }
    }
}

fn init_plan(output: &PathBuf) -> ExitCode {
    let template = r#"# PWF Plan v1
# Documentation: https://pwf.dev/docs/SPECIFICATION

plan_version: 1

meta:
  title: "My Training Plan"
  description: "A brief description of this plan"
  author: "Your Name"
  equipment: [dumbbells]
  daysPerWeek: 3
  tags: [strength]

cycle:
  notes: "Coaching notes for the entire cycle"

  days:
    - focus: "Day 1"
      target_session_length_min: 45
      exercises:
        - name: "Exercise Name"
          modality: strength
          target_sets: 3
          target_reps: 10
          target_notes: "Form cues go here"

    - focus: "Day 2"
      exercises:
        - name: "Another Exercise"
          modality: strength
          target_sets: 3
          target_reps: 8
"#;

    write_template(output, template)
}

fn init_history(output: &PathBuf) -> ExitCode {
    let template = r#"# PWF History Export v1
# Documentation: https://pwf.dev/docs/blocks/history

history_version: 1
exported_at: "2025-01-15T10:30:00Z"

export_source:
  app_name: "Your App"
  app_version: "1.0.0"

units:
  weight: kg
  distance: meters

workouts:
  - date: "2025-01-15"
    title: "Push Day"
    started_at: "2025-01-15T09:00:00Z"
    ended_at: "2025-01-15T10:00:00Z"
    duration_sec: 3600
    exercises:
      - name: "Bench Press"
        modality: strength
        sets:
          - set_number: 1
            set_type: warmup
            reps: 10
            weight_kg: 60
          - set_number: 2
            set_type: working
            reps: 5
            weight_kg: 100
            rpe: 8
          - set_number: 3
            set_type: working
            reps: 5
            weight_kg: 100
            rpe: 8.5
            is_pr: true

personal_records:
  - exercise_name: "Bench Press"
    record_type: max_weight
    value: 100
    unit: kg
    achieved_at: "2025-01-15"

body_measurements:
  - date: "2025-01-15"
    weight_kg: 85.5
    body_fat_percent: 15.0
"#;

    write_template(output, template)
}

fn write_template(output: &PathBuf, template: &str) -> ExitCode {
    if output.exists() {
        eprintln!(
            "{}: File already exists: {}",
            "error".red(),
            output.display()
        );
        return ExitCode::FAILURE;
    }

    match fs::write(output, template) {
        Ok(_) => {
            println!("{} Created {}", "✓".green(), output.display());
            println!();
            println!("Next steps:");
            println!("  1. Edit {} to add your data", output.display());
            println!(
                "  2. Run {} to validate",
                format!("pwf validate {}", output.display()).cyan()
            );
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("{}: {}", "error".red(), e);
            ExitCode::FAILURE
        }
    }
}
