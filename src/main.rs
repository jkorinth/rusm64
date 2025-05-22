use clap::{Parser, Subcommand};
use rusm::{parse_source /*, assemble, assemble_verbose*/};
use std::fs;
use std::path::PathBuf;
use std::process;

#[derive(Parser)]
#[command(name = "rusm")]
#[command(about = "A Rust-based 6502 assembler for the Commodore 64")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Assemble a source file to binary
    Assemble {
        /// Input assembly file
        #[arg(required = true)]
        input: PathBuf,

        /// Output binary file [default: input filename with .prg extension]
        #[arg(short, long)]
        output: Option<PathBuf>,

        /// Enable verbose output
        #[arg(short, long)]
        verbose: bool,
    },
    /// Parse a source file and print the AST (for debugging)
    Parse {
        /// Input assembly file
        #[arg(required = true)]
        input: PathBuf,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Assemble {
            input,
            output,
            verbose,
        } => {
            let output_path = output.unwrap_or_else(|| {
                let mut path = input.clone();
                path.set_extension("prg");
                path
            });

            match assemble_file(&input, &output_path, verbose) {
                Ok(_) => {
                    println!(
                        "Successfully assembled {} to {}",
                        input.display(),
                        output_path.display()
                    );
                }
                Err(e) => {
                    eprintln!("Error assembling file: {}", e);
                    process::exit(1);
                }
            }
        }
        Commands::Parse { input } => match parse_file(&input) {
            Ok(_) => {
                println!("Successfully parsed {}", input.display());
            }
            Err(e) => {
                eprintln!("Error parsing file: {}", e);
                process::exit(1);
            }
        },
    }
}

fn assemble_file(input_path: &PathBuf, output_path: &PathBuf, verbose: bool) -> rusm::Result<()> {
    let source = fs::read_to_string(input_path)?;
    let ast = parse_source(&source)?;

    if verbose {
        println!("Parsed AST:");
        println!("{:#?}", ast);
    }
    /*

    let binary = if verbose {
        assemble_verbose(&ast)?
    } else {
        assemble(&ast)?
    };

    if verbose {
        println!("Generated {} bytes of machine code", binary.len());
        print_binary_dump(&binary, 16);
    }

    fs::write(output_path, binary)?;*/
    Ok(())
}

fn print_binary_dump(data: &[u8], bytes_per_line: usize) {
    for (i, chunk) in data.chunks(bytes_per_line).enumerate() {
        print!("{:04X}: ", i * bytes_per_line);

        // Print hex values
        for (j, byte) in chunk.iter().enumerate() {
            print!("{:02X} ", byte);
            if j == 7 {
                print!(" "); // Extra space in the middle
            }
        }

        // Pad if line is shorter than bytes_per_line
        for _ in chunk.len()..bytes_per_line {
            print!("   ");
        }

        // Print ASCII representation
        print!(" |");
        for byte in chunk {
            let c = if *byte >= 32 && *byte <= 126 {
                *byte as char
            } else {
                '.'
            };
            print!("{}", c);
        }
        println!("|");
    }
}

fn parse_file(input_path: &PathBuf) -> rusm::Result<()> {
    let source = fs::read_to_string(input_path)?;
    let ast = parse_source(&source)?;
    println!("{:#?}", ast);
    Ok(())
}
