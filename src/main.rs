// qbedec — QuickBASIC 4.5 decompiler CLI
//
// v0.0.4-Whitten — BRUN-mode prototype.
// - Leest input .exe
// - Classifieert (DOS-MZ vs Windows-PE vs invalid)
// - Detecteert BRUN-mode-stub via heuristic
// - Output: minimaal placeholder .bas met watermark + diagnostics
//   OF JSON met AST + verdict + evidence

use clap::{Parser, ValueEnum};
use qbedec::{ast::Program, brun::detect_brun, pe::{classify, PeKind}, watermark_header};
use std::fs;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(version, about = "Decompile QuickBASIC 4.5 .exe to .bas")]
struct Cli {
    /// Input .exe file
    input: PathBuf,

    /// Output file (default: stdout)
    #[arg(short, long)]
    output: Option<PathBuf>,

    /// Output format
    #[arg(long, value_enum, default_value_t = OutputFormat::Bas)]
    format: OutputFormat,
}

#[derive(Copy, Clone, Debug, ValueEnum)]
enum OutputFormat {
    Bas,
    Json,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let bytes = fs::read(&cli.input)?;
    let kind = classify(&bytes);
    let verdict = detect_brun(&bytes);

    let source_filename = cli
        .input
        .file_name()
        .map(|s| s.to_string_lossy().to_string())
        .unwrap_or_else(|| "<unknown>".to_string());
    let date_iso = "2026-06-01".to_string(); // For v0.0.4 we keep deterministic; real date in v0.0.5+

    let output_text: String = match cli.format {
        OutputFormat::Bas => {
            let mut s = watermark_header(&source_filename, &date_iso);
            s.push_str(&format!("' Classification: {:?}\n", kind));
            s.push_str(&format!("' BRUN verdict: {:?}\n", verdict.confidence));
            for ev in &verdict.evidence {
                s.push_str(&format!("' Evidence: {}\n", ev));
            }
            s.push('\n');
            if !matches!(kind, PeKind::DosMz) {
                s.push_str("' [Decompiler requires DOS-MZ executable produced by QuickBASIC 4.5]\n");
                s.push_str("' [Modern Windows PE detected — no decompilation attempted]\n");
            } else if !verdict.is_brun {
                s.push_str("' [BRUN-mode not detected — Stand-alone EXE mode arrives in v0.4.0-Chien]\n");
            } else {
                s.push_str("' [BRUN-mode detected — actual statement reconstruction arrives in v0.2.0-Weiland]\n");
                s.push_str("' [v0.0.4-Whitten provides architecture-skeleton only]\n");
            }
            let program = Program::default();
            s.push_str(&program.to_bas());
            s.push_str("END\n");
            s
        }
        OutputFormat::Json => {
            let result = serde_json::json!({
                "decompiler_version": qbedec::VERSION,
                "codename": qbedec::CODENAME,
                "input_filename": source_filename,
                "classification": format!("{:?}", kind),
                "brun_verdict": verdict,
                "program": Program::default(),
                "watermark": "Reconstructed source — see LEGAL.md",
            });
            serde_json::to_string_pretty(&result)?
        }
    };

    match cli.output {
        Some(p) => fs::write(p, output_text)?,
        None => print!("{}", output_text),
    }
    Ok(())
}
