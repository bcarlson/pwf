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

    /// Convert between PWF and other formats (FIT, TCX, GPX)
    Convert {
        /// Input format (fit, tcx, gpx, pwf)
        #[arg(long)]
        from: String,

        /// Output format (pwf, fit, tcx, gpx, csv)
        #[arg(long)]
        to: String,

        /// Input file path
        input: PathBuf,

        /// Output file path
        output: PathBuf,

        /// Summary only (skip time-series data for smaller files)
        #[arg(long)]
        summary_only: bool,

        /// Verbose output (show conversion warnings)
        #[arg(short, long)]
        verbose: bool,
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
        Commands::Convert {
            from,
            to,
            input,
            output,
            summary_only,
            verbose,
        } => convert_file(&from, &to, &input, &output, summary_only, verbose),
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

fn convert_file(
    from: &str,
    to: &str,
    input: &PathBuf,
    output: &PathBuf,
    summary_only: bool,
    verbose: bool,
) -> ExitCode {
    // Validate format combinations
    let from_lower = from.to_lowercase();
    let to_lower = to.to_lowercase();

    if from_lower == to_lower {
        eprintln!(
            "{}: Input and output formats are the same: {}",
            "error".red(),
            from
        );
        return ExitCode::FAILURE;
    }

    // Check input file exists
    if !input.exists() {
        eprintln!(
            "{}: Input file not found: {}",
            "error".red(),
            input.display()
        );
        return ExitCode::FAILURE;
    }

    // Check output file doesn't exist
    if output.exists() {
        eprintln!(
            "{}: Output file already exists: {}",
            "error".red(),
            output.display()
        );
        return ExitCode::FAILURE;
    }

    // Perform conversion based on formats
    match (from_lower.as_str(), to_lower.as_str()) {
        ("fit", "pwf") => convert_fit_to_pwf(input, output, summary_only, verbose),
        ("tcx", "pwf") => convert_tcx_to_pwf(input, output, summary_only, verbose),
        ("gpx", "pwf") => convert_gpx_to_pwf(input, output, summary_only, verbose),
        ("pwf", "tcx") => convert_pwf_to_tcx(input, output, verbose),
        ("pwf", "gpx") => convert_pwf_to_gpx(input, output, verbose),
        ("pwf", "csv") => convert_pwf_to_csv(input, output, verbose),
        ("pwf", "fit") => {
            // Special error message for FIT export
            eprintln!("{}: FIT export is not currently supported", "error".red());
            eprintln!();
            eprintln!("{}", "Reason:".bold());
            eprintln!("  No production-ready Rust library for FIT file writing is available.");
            eprintln!("  Current libraries are either read-only or experimental/undocumented.");
            eprintln!();
            eprintln!("{}", "Recommended alternative:".bold());
            eprintln!("  Export to TCX format instead:");
            eprintln!(
                "  {}",
                "pwf convert --from pwf --to tcx workout.yaml output.tcx".cyan()
            );
            eprintln!();
            eprintln!("  TCX files are accepted by:");
            eprintln!("  • Garmin Connect");
            eprintln!("  • Strava");
            eprintln!("  • TrainingPeaks");
            eprintln!("  • Most fitness platforms");
            eprintln!();
            eprintln!("{}", "Workaround (if FIT is required):".bold());
            eprintln!(
                "  1. Export to TCX: pwf convert --from pwf --to tcx workout.yaml output.tcx"
            );
            eprintln!("  2. Use Garmin FitCSVTool to convert TCX → FIT:");
            eprintln!(
                "     {}",
                "https://developer.garmin.com/fit/fitcsvtool/".underline()
            );
            eprintln!();
            eprintln!("For more details, see: {}", "FIT_EXPORT_ANALYSIS.md".cyan());
            ExitCode::FAILURE
        }
        (from, to) => {
            eprintln!(
                "{}: Conversion from {} to {} is not yet implemented",
                "error".red(),
                from,
                to
            );
            eprintln!();
            eprintln!("Currently supported conversions:");
            eprintln!("  {} → {}", "fit".green(), "pwf".green());
            eprintln!("  {} → {}", "tcx".green(), "pwf".green());
            eprintln!("  {} → {}", "gpx".green(), "pwf".green());
            eprintln!("  {} → {}", "pwf".green(), "tcx".green());
            eprintln!("  {} → {}", "pwf".green(), "gpx".green());
            eprintln!("  {} → {}", "pwf".green(), "csv".green());
            ExitCode::FAILURE
        }
    }
}

fn convert_fit_to_pwf(
    input: &PathBuf,
    output: &PathBuf,
    summary_only: bool,
    verbose: bool,
) -> ExitCode {
    println!("{} Converting {} to PWF...", "→".cyan(), input.display());

    if verbose {
        println!("  {} Reading FIT file...", "→".dimmed());
    }

    // Open input file
    let file = match fs::File::open(input) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("{}: Failed to open input file: {}", "error".red(), e);
            return ExitCode::FAILURE;
        }
    };

    // Get file size for progress indication
    let file_size = file.metadata().map(|m| m.len()).unwrap_or(0);
    if verbose && file_size > 1_000_000 {
        println!(
            "  {} Large file detected ({:.1} MB), this may take a moment...",
            "ℹ".dimmed(),
            file_size as f64 / 1_000_000.0
        );
    }

    if verbose {
        println!("  {} Parsing FIT records...", "→".dimmed());
    }

    // Convert using pwf-converters library
    let result = match pwf_converters::fit_to_pwf(file, summary_only) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("{}: Conversion failed: {}", "error".red(), e);
            return ExitCode::FAILURE;
        }
    };

    if verbose {
        println!("  {} Converting to PWF structure...", "→".dimmed());
    }

    // Show warnings if verbose
    if verbose && !result.warnings.is_empty() {
        println!();
        println!("{} Conversion warnings:", "⚠".yellow());
        for warning in &result.warnings {
            println!("  {} {}", "⚠".yellow(), warning.to_string().yellow());
        }
        println!();
    }

    if verbose {
        // Show conversion statistics
        let line_count = result.pwf_yaml.lines().count();
        let size_kb = result.pwf_yaml.len() as f64 / 1024.0;
        println!(
            "  {} Generated PWF YAML: {} lines, {:.1} KB",
            "✓".dimmed(),
            line_count,
            size_kb
        );
        println!("  {} Writing output file...", "→".dimmed());
    }

    // Write output file
    match fs::write(output, &result.pwf_yaml) {
        Ok(_) => {
            println!("{} Converted to {}", "✓".green(), output.display());

            if !verbose && result.has_warnings() {
                println!(
                    "  {} warnings (use {} to see details)",
                    result.warnings.len().to_string().yellow(),
                    "--verbose".cyan()
                );
            }

            println!();
            println!("Next steps:");
            println!(
                "  Validate: {}",
                format!("pwf history {}", output.display()).cyan()
            );

            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("{}: Failed to write output file: {}", "error".red(), e);
            ExitCode::FAILURE
        }
    }
}

fn convert_tcx_to_pwf(
    input: &PathBuf,
    output: &PathBuf,
    summary_only: bool,
    verbose: bool,
) -> ExitCode {
    println!("{} Converting {} to PWF...", "→".cyan(), input.display());

    if verbose {
        println!("  {} Reading TCX file...", "→".dimmed());
    }

    // Open input file
    let file = match fs::File::open(input) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("{}: Failed to open input file: {}", "error".red(), e);
            return ExitCode::FAILURE;
        }
    };

    // Get file size for progress indication
    let file_size = file.metadata().map(|m| m.len()).unwrap_or(0);
    if verbose && file_size > 1_000_000 {
        println!(
            "  {} Large file detected ({:.1} MB), this may take a moment...",
            "ℹ".dimmed(),
            file_size as f64 / 1_000_000.0
        );
    }

    if verbose {
        println!("  {} Parsing TCX activities...", "→".dimmed());
    }

    // Convert using pwf-converters library
    let result: pwf_converters::ConversionResult =
        match pwf_converters::tcx_to_pwf(file, summary_only) {
            Ok(r) => r,
            Err(e) => {
                eprintln!("{}: Conversion failed: {}", "error".red(), e);
                return ExitCode::FAILURE;
            }
        };

    if verbose {
        println!("  {} Converting to PWF structure...", "→".dimmed());
    }

    // Show warnings if verbose
    if verbose && !result.warnings.is_empty() {
        println!();
        println!("{} Conversion warnings:", "⚠".yellow());
        for warning in &result.warnings {
            println!("  {} {}", "⚠".yellow(), warning.to_string().yellow());
        }
        println!();
    }

    if verbose {
        // Show conversion statistics
        let line_count = result.pwf_yaml.lines().count();
        let size_kb = result.pwf_yaml.len() as f64 / 1024.0;
        println!(
            "  {} Generated PWF YAML: {} lines, {:.1} KB",
            "✓".dimmed(),
            line_count,
            size_kb
        );
        println!("  {} Writing output file...", "→".dimmed());
    }

    // Write output file
    match fs::write(output, &result.pwf_yaml) {
        Ok(_) => {
            println!("{} Converted to {}", "✓".green(), output.display());

            if !verbose && result.has_warnings() {
                println!(
                    "  {} warnings (use {} to see details)",
                    result.warnings.len().to_string().yellow(),
                    "--verbose".cyan()
                );
            }

            println!();
            println!("Next steps:");
            println!(
                "  Validate: {}",
                format!("pwf history {}", output.display()).cyan()
            );

            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("{}: Failed to write output file: {}", "error".red(), e);
            ExitCode::FAILURE
        }
    }
}

fn convert_gpx_to_pwf(
    input: &PathBuf,
    output: &PathBuf,
    summary_only: bool,
    verbose: bool,
) -> ExitCode {
    println!("{} Converting {} to PWF...", "→".cyan(), input.display());

    if verbose {
        println!("  {} Reading GPX file...", "→".dimmed());
    }

    // Open input file
    let file = match fs::File::open(input) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("{}: Failed to open input file: {}", "error".red(), e);
            return ExitCode::FAILURE;
        }
    };

    if verbose {
        println!("  {} Parsing GPX tracks...", "→".dimmed());
    }

    // Convert GPX to PWF
    let result: pwf_converters::ConversionResult =
        match pwf_converters::gpx_to_pwf(file, summary_only) {
            Ok(r) => r,
            Err(e) => {
                eprintln!("{}: Conversion failed: {}", "error".red(), e);
                return ExitCode::FAILURE;
            }
        };

    if verbose {
        println!("  {} Converting to PWF structure...", "→".dimmed());
    }

    // Show warnings if verbose
    if verbose && !result.warnings.is_empty() {
        println!();
        println!("{} Conversion warnings:", "⚠".yellow());
        for warning in &result.warnings {
            println!("  {} {}", "⚠".yellow(), warning.to_string().yellow());
        }
        println!();
    }

    if verbose {
        let line_count = result.pwf_yaml.lines().count();
        let size_kb = result.pwf_yaml.len() as f64 / 1024.0;
        println!(
            "  {} Generated PWF YAML: {} lines, {:.1} KB",
            "✓".dimmed(),
            line_count,
            size_kb
        );
        println!("  {} Writing output file...", "→".dimmed());
    }

    // Write output file
    match fs::write(output, &result.pwf_yaml) {
        Ok(_) => {
            println!("{} Converted to {}", "✓".green(), output.display());

            if !verbose && result.has_warnings() {
                println!(
                    "  {} warnings (use {} to see details)",
                    result.warnings.len().to_string().yellow(),
                    "--verbose".cyan()
                );
            }

            println!();
            println!("Next steps:");
            println!(
                "  Validate: {}",
                format!("pwf history {}", output.display()).cyan()
            );

            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("{}: Failed to write output file: {}", "error".red(), e);
            ExitCode::FAILURE
        }
    }
}

fn convert_pwf_to_gpx(input: &PathBuf, output: &PathBuf, verbose: bool) -> ExitCode {
    println!("{} Exporting {} to GPX...", "→".cyan(), input.display());

    if verbose {
        println!("  {} Reading PWF history file...", "→".dimmed());
    }

    // Read PWF history file
    let content = match fs::read_to_string(input) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("{}: Failed to read input file: {}", "error".red(), e);
            return ExitCode::FAILURE;
        }
    };

    if verbose {
        println!("  {} Parsing PWF history...", "→".dimmed());
    }

    // Parse PWF history
    let history: pwf_core::history::WpsHistory = match pwf_core::history::parse(&content) {
        Ok(h) => h,
        Err(e) => {
            eprintln!("{}: Failed to parse PWF history: {}", "error".red(), e);
            eprintln!();
            eprintln!("Hint: Validate your PWF file first:");
            eprintln!("  {}", format!("pwf history {}", input.display()).cyan());
            return ExitCode::FAILURE;
        }
    };

    if verbose {
        println!(
            "  {} Converting {} workouts to GPX format...",
            "→".dimmed(),
            history.workouts.len()
        );
    }

    // Convert to GPX using pwf-converters library
    let result = match pwf_converters::pwf_to_gpx(&history) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("{}: Export failed: {}", "error".red(), e);
            return ExitCode::FAILURE;
        }
    };

    // Show warnings if verbose
    if verbose && !result.warnings.is_empty() {
        println!();
        println!("{} Export warnings:", "⚠".yellow());
        for warning in &result.warnings {
            println!("  {} {}", "⚠".yellow(), warning.to_string().yellow());
        }
        println!();
    }

    if verbose {
        // Show export statistics
        let line_count = result.gpx_xml.lines().count();
        let size_kb = result.gpx_xml.len() as f64 / 1024.0;
        println!(
            "  {} Generated GPX XML: {} lines, {:.1} KB",
            "✓".dimmed(),
            line_count,
            size_kb
        );
        println!("  {} Writing output file...", "→".dimmed());
    }

    // Write output file
    match fs::write(output, &result.gpx_xml) {
        Ok(_) => {
            println!("{} Exported to {}", "✓".green(), output.display());

            if !verbose && result.has_warnings() {
                println!(
                    "  {} warnings (use {} to see details)",
                    result.warnings.len().to_string().yellow(),
                    "--verbose".cyan()
                );
            }

            println!();
            println!("Next steps:");
            println!("  Upload to GPS platforms:");
            println!("    • Garmin Connect");
            println!("    • Strava");
            println!("    • Komoot");
            println!("    • AllTrails");
            println!("    • Most GPS/mapping platforms");

            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("{}: Failed to write output file: {}", "error".red(), e);
            ExitCode::FAILURE
        }
    }
}

fn convert_pwf_to_csv(input: &PathBuf, output: &PathBuf, verbose: bool) -> ExitCode {
    println!("{} Exporting {} to CSV...", "→".cyan(), input.display());

    if verbose {
        println!("  {} Reading PWF history file...", "→".dimmed());
    }

    // Read PWF history file
    let content = match fs::read_to_string(input) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("{}: Failed to read input file: {}", "error".red(), e);
            return ExitCode::FAILURE;
        }
    };

    if verbose {
        println!("  {} Parsing PWF history...", "→".dimmed());
    }

    // Parse PWF history
    let history: pwf_core::history::WpsHistory = match pwf_core::history::parse(&content) {
        Ok(h) => h,
        Err(e) => {
            eprintln!("{}: Failed to parse PWF history: {}", "error".red(), e);
            eprintln!();
            eprintln!("Hint: Validate your PWF file first:");
            eprintln!("  {}", format!("pwf history {}", input.display()).cyan());
            return ExitCode::FAILURE;
        }
    };

    if verbose {
        println!(
            "  {} Extracting time-series telemetry data...",
            "→".dimmed()
        );
    }

    // Export to CSV using pwf-converters library
    let options = pwf_converters::CsvExportOptions::default();
    let result = match pwf_converters::export_telemetry_to_csv(&history, &options) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("{}: Export failed: {}", "error".red(), e);
            eprintln!();
            eprintln!("{}", "Note:".bold());
            eprintln!(
                "  CSV export requires workouts with second-by-second time-series telemetry data."
            );
            eprintln!("  Only summary metrics (avg/max values) are not sufficient.");
            eprintln!();
            eprintln!("{}", "Example of compatible PWF history:".bold());
            eprintln!("  exercises:");
            eprintln!("    - name: Cycling");
            eprintln!("      sets:");
            eprintln!("        - telemetry:");
            eprintln!("            time_series:");
            eprintln!("              timestamps: [\"2025-01-15T14:30:00Z\", ...]");
            eprintln!("              heart_rate: [145, 147, ...]");
            eprintln!("              power: [200, 205, ...]");
            return ExitCode::FAILURE;
        }
    };

    // Show warnings if verbose
    if verbose && !result.warnings.is_empty() {
        println!();
        println!("{} Export warnings:", "⚠".yellow());
        for warning in &result.warnings {
            println!("  {} {}", "⚠".yellow(), warning.to_string().yellow());
        }
        println!();
    }

    if verbose {
        // Show export statistics
        let line_count = result.csv_data.lines().count();
        let size_kb = result.csv_data.len() as f64 / 1024.0;
        println!(
            "  {} Generated CSV: {} lines, {:.1} KB",
            "✓".dimmed(),
            line_count,
            size_kb
        );
        println!(
            "  {} Exported {} data points from {} workout(s)",
            "✓".dimmed(),
            result.data_points,
            result.workouts_processed
        );
        println!("  {} Writing output file...", "→".dimmed());
    }

    // Write output file
    match fs::write(output, &result.csv_data) {
        Ok(_) => {
            println!("{} Exported to {}", "✓".green(), output.display());

            if !verbose && result.has_warnings() {
                println!(
                    "  {} warnings (use {} to see details)",
                    result.warnings.len().to_string().yellow(),
                    "--verbose".cyan()
                );
            }

            println!();
            println!("Export summary:");
            println!("  Data points:  {}", result.data_points.to_string().cyan());
            println!(
                "  Workouts:     {}",
                result.workouts_processed.to_string().cyan()
            );
            println!();
            println!("Next steps:");
            println!("  Open in Excel, Google Sheets, or any spreadsheet application");
            println!("  Analyze trends, create charts, or perform statistical analysis");

            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("{}: Failed to write output file: {}", "error".red(), e);
            ExitCode::FAILURE
        }
    }
}

fn convert_pwf_to_tcx(input: &PathBuf, output: &PathBuf, verbose: bool) -> ExitCode {
    println!("{} Exporting {} to TCX...", "→".cyan(), input.display());

    if verbose {
        println!("  {} Reading PWF history file...", "→".dimmed());
    }

    // Read PWF history file
    let content = match fs::read_to_string(input) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("{}: Failed to read input file: {}", "error".red(), e);
            return ExitCode::FAILURE;
        }
    };

    if verbose {
        println!("  {} Parsing PWF history...", "→".dimmed());
    }

    // Parse PWF history
    let history: pwf_core::history::WpsHistory = match pwf_core::history::parse(&content) {
        Ok(h) => h,
        Err(e) => {
            eprintln!("{}: Failed to parse PWF history: {}", "error".red(), e);
            eprintln!();
            eprintln!("Hint: Validate your PWF file first:");
            eprintln!("  {}", format!("pwf history {}", input.display()).cyan());
            return ExitCode::FAILURE;
        }
    };

    if verbose {
        println!(
            "  {} Converting {} workouts to TCX format...",
            "→".dimmed(),
            history.workouts.len()
        );
    }

    // Convert to TCX using pwf-converters library
    let result = match pwf_converters::pwf_to_tcx(&history) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("{}: Export failed: {}", "error".red(), e);
            return ExitCode::FAILURE;
        }
    };

    // Show warnings if verbose
    if verbose && !result.warnings.is_empty() {
        println!();
        println!("{} Export warnings:", "⚠".yellow());
        for warning in &result.warnings {
            println!("  {} {}", "⚠".yellow(), warning.to_string().yellow());
        }
        println!();
    }

    if verbose {
        // Show export statistics
        let line_count = result.tcx_xml.lines().count();
        let size_kb = result.tcx_xml.len() as f64 / 1024.0;
        println!(
            "  {} Generated TCX XML: {} lines, {:.1} KB",
            "✓".dimmed(),
            line_count,
            size_kb
        );
        println!("  {} Writing output file...", "→".dimmed());
    }

    // Write output file
    match fs::write(output, &result.tcx_xml) {
        Ok(_) => {
            println!("{} Exported to {}", "✓".green(), output.display());

            if !verbose && result.has_warnings() {
                println!(
                    "  {} warnings (use {} to see details)",
                    result.warnings.len().to_string().yellow(),
                    "--verbose".cyan()
                );
            }

            println!();
            println!("Next steps:");
            println!("  Upload to fitness platforms:");
            println!("    • Garmin Connect");
            println!("    • Strava");
            println!("    • TrainingPeaks");
            println!("    • Most other fitness platforms");

            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("{}: Failed to write output file: {}", "error".red(), e);
            ExitCode::FAILURE
        }
    }
}
