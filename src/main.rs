// QuickBasicEmulator_Decompiler — entry point
//
// v0.0.1-Gates: skeleton placeholder. CLI accepts arguments but does not decompile.
// v0.0.4-Whitten: BRUN-mode prototype.
// v0.4.0-Chien: Stand-alone EXE mode + full signature-DB.

use clap::{Parser, ValueEnum};

#[derive(Parser, Debug)]
#[command(version, about = "Decompile QuickBASIC 4.5 .exe to .bas (skeleton)")]
struct Cli {
    /// Input .exe file
    input: String,

    /// Output file (default: stdout)
    #[arg(short, long)]
    output: Option<String>,

    /// Output format
    #[arg(long, value_enum, default_value_t = OutputFormat::Bas)]
    format: OutputFormat,
}

#[derive(Copy, Clone, Debug, ValueEnum)]
enum OutputFormat {
    Bas,
    Json,
}

fn main() {
    let cli = Cli::parse();
    println!("qbedec v0.0.1-Gates — skeleton placeholder");
    println!("Input: {}", cli.input);
    println!("Format: {:?}", cli.format);
    println!("Decompilation arrives v0.0.4-Whitten (BRUN-mode prototype).");
}
