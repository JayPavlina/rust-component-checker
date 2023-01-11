//! Finds missing rust components for a specific nightly version.

use std::collections::{HashSet};
use chrono::NaiveDate;
use rustup_available_packages::{AvailabilityData, Downloader};

/// The packages to check
const PACKAGES: [&str; 15] = [
    "cargo",
    "clippy",
    "llvm-tools",
    "miri",
    "reproducible-artifacts",
    "rls",
    "rust",
    "rust-analysis",
    "rust-analyzer",
    "rust-docs",
    "rust-docs-json",
    "rust-std",
    "rustc-dev",
    "rustc-docs",
    "rustfmt",
];

fn main() {
    let target = "x86_64-unknown-linux-gnu";
    let date = NaiveDate::from_ymd_opt(2022, 9, 20).unwrap();
    let manifest = Downloader::with_default_source("nightly").get_manifest(date).unwrap();
    let mut availability = AvailabilityData::default();
    availability.add_manifest(manifest);

    let mut missing_packages = HashSet::new();

    for package in PACKAGES {
        let row = availability.get_availability_row(target, package, vec![date]);
        if !row.map(|x| *x.availability_list.first().unwrap_or(&false)).unwrap_or_default() {
            missing_packages.insert(package);
        }
    }

    if missing_packages.is_empty() {
        println!("No missing packages")
    } else {
        println!("Missing Packages:\n===================");
        for package in missing_packages {
            println!("{package}")
        }
    }
}
