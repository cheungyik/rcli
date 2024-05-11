use std::fs;

use anyhow::{Ok, Result};
use clap::Parser;
use rcli::{
    process_csv, process_decode, process_encode, process_generate, process_genpass,
    process_text_sign, process_text_verify, Base64SubCommand, Opts, SubCommand, TextSignFormat,
    TextSubCommand,
};
use zxcvbn::zxcvbn;

fn main() -> Result<()> {
    let opts = Opts::parse();
    match opts.cmd {
        SubCommand::Csv(opts) => {
            let output = opts.output.unwrap_or(format!("output.{}", opts.format));
            process_csv(&opts.input, &output, opts.format)?;
        }
        SubCommand::GenPass(opts) => {
            let pwd = process_genpass(
                opts.length,
                opts.uppercase,
                opts.lowercase,
                opts.number,
                opts.symbol,
            )?;
            println!("{}", pwd);
            let estimate = zxcvbn(&pwd, &[])?;
            eprintln!("Password strength: {}", estimate.score());
        }
        SubCommand::Base64(subcmd) => match subcmd {
            Base64SubCommand::Encode(opts) => {
                let encoded = process_encode(&opts.input, opts.format)?;
                println!("{}", encoded);
            }
            Base64SubCommand::Decode(opts) => {
                let decoded = process_decode(&opts.input, opts.format)?;
                let decoded = String::from_utf8(decoded)?;
                println!("{}", decoded);
            }
        },
        SubCommand::Text(subcmd) => match subcmd {
            TextSubCommand::Sign(opts) => {
                let signed = process_text_sign(&opts.input, &opts.key, opts.format)?;
                println!("{}", signed);
            }
            TextSubCommand::Verify(opts) => {
                let verified = process_text_verify(&opts.input, &opts.key, opts.format, &opts.sig)?;
                println!("{}", verified);
            }
            TextSubCommand::Generate(opts) => {
                let keys = process_generate(opts.format)?;
                match opts.format {
                    TextSignFormat::Blake3 => {
                        let name = &opts.output.join("blake3.txt");
                        fs::write(name, &keys[0])?;
                    }
                    TextSignFormat::Ed25519 => {
                        let name = &opts.output;
                        fs::write(name.join("ed25519.sk"), &keys[0])?;
                        fs::write(name.join("ed25519.pk"), &keys[1])?;
                    }
                }
            }
        },
    }
    Ok(())
}
