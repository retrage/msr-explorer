// SPDX-License-Identifier: MIT
// Copyright (C) 2023 Akira Moroo

//! x86 MSR Explorer
//!
//! This tool is a dumb tool that searches for x86 MSR space by checking if the MSR is readable or not.

use std::{
    error,
    fs::File,
    io::{self, Read, Seek, SeekFrom},
};

use clap::{Args, Parser, Subcommand};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct CpuMsr {
    cpu: usize,
    msrs: Vec<Msr>,
}

impl CpuMsr {
    const fn new(cpu: usize) -> Self {
        Self {
            cpu,
            msrs: Vec::new(),
        }
    }

    fn read(&mut self, addr: u64) -> io::Result<()> {
        let mut msr = Msr::new(addr);
        msr.read(self.cpu)?;
        self.msrs.push(msr);
        Ok(())
    }
}

#[derive(Serialize, Deserialize)]
struct Msr {
    addr: u64,
    value: Option<[u8; 8]>,
}

impl Msr {
    const fn new(addr: u64) -> Self {
        Self { addr, value: None }
    }

    fn read(&mut self, cpu: usize) -> io::Result<()> {
        let mut fd = File::open(format!("/dev/cpu/{}/msr", cpu))?;
        let mut buf = [0u8; 8];
        fd.seek(SeekFrom::Start(self.addr))?;
        match fd.read_exact(&mut buf) {
            Ok(_) => self.value = Some(buf),
            Err(_) => self.value = None,
        };
        Ok(())
    }
}

#[derive(Parser)]
struct Cli {
    /// CPU number
    #[arg(short, long, default_value = "0")]
    cpu: usize,
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Read MSR
    Rdmsr(RdmsrArgs),
    RdmsrRange(RdmsrRangeArgs),
}

#[derive(Args)]
struct RdmsrArgs {
    /// MSR address
    #[arg(required = true)]
    addr: u64,
}

#[derive(Args)]
struct RdmsrRangeArgs {
    /// MSR start address
    #[arg(required = true)]
    addr_start: u64,
    /// MSR end address
    #[arg(required = true)]
    addr_end: u64,
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Rdmsr(args) => {
            let mut msr = Msr::new(args.addr);
            msr.read(cli.cpu)?;
            let json = serde_json::to_string_pretty(&msr)?;
            println!("{}", json);
        }
        Commands::RdmsrRange(args) => {
            let mut cpu_msr = CpuMsr::new(cli.cpu);
            for addr in args.addr_start..=args.addr_end {
                cpu_msr.read(addr)?;
            }
            let json = serde_json::to_string_pretty(&cpu_msr)?;
            println!("{}", json);
        }
    }

    Ok(())
}
