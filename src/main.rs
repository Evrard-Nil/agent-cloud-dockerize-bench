use clap::Parser;
use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;
use std::io::{self, Write};
use std::process::{Command, Stdio};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Command to run the auto-dockerizer tool
    #[clap(short, long)]
    dockerizer_command: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct Repository {
    name: String,
    url: String,
    tags: Vec<String>,
}

const REPOS: &str = include_str!("../repositories.json");

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let repositories: Vec<Repository> = serde_json::from_str(&REPOS)?;

    let mut total_benchmarked = 0;
    let mut successful_dockerizations = 0;
    let mut failed_dockerizations = 0;
    let mut tag_results: HashMap<String, (usize, usize)> = HashMap::new(); // (successful, failed)

    println!("--- Auto-Dockerizer Benchmark Report ---");

    for repo in repositories {
        println!("\nBenchmarking repository: {}", repo.name);
        total_benchmarked += 1;

        // Execute the dockerizer command
        let cmd = format!("{} {}", args.dockerizer_command, repo.url);
        println!("Executing dockerizer command: {}", cmd,);
        let _dockerizer_output = Command::new("sh")
            .arg("-c")
            .arg(&cmd)
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .output()?;

        // Prompt for success/failure
        let mut input = String::new();
        print!("Was dockerization successful for {}? (y/n): ", repo.name);
        io::stdout().flush()?;
        io::stdin().read_line(&mut input)?;
        let success = input.trim().to_lowercase() == "y";

        if success {
            successful_dockerizations += 1;
        } else {
            failed_dockerizations += 1;
        }

        // Update tag results
        for tag in &repo.tags {
            let (mut s, mut f) = *tag_results.entry(tag.clone()).or_insert((0, 0));
            if success {
                s += 1;
            } else {
                f += 1;
            }
            tag_results.insert(tag.clone(), (s, f));
        }
    }

    // Generate and print report
    let overall_success_rate = if total_benchmarked > 0 {
        (successful_dockerizations as f64 / total_benchmarked as f64) * 100.0
    } else {
        0.0
    };

    println!("\n--- Auto-Dockerizer Benchmark Report ---");
    println!("\nOverall Performance:");
    println!("  Total Repositories Benchmarked: {}", total_benchmarked);
    println!("  Successful Dockerizations: {}", successful_dockerizations);
    println!("  Failed Dockerizations: {}", failed_dockerizations);
    println!("  Overall Success Rate: {:.2}%", overall_success_rate);

    println!("\nBreakdown by Tag:");
    let mut sorted_tags: Vec<_> = tag_results.keys().collect();
    sorted_tags.sort();

    for tag in sorted_tags {
        let (s, f) = tag_results[tag];
        let tag_total = s + f;
        let tag_success_rate = if tag_total > 0 {
            (s as f64 / tag_total as f64) * 100.0
        } else {
            0.0
        };
        println!("  {}:", tag);
        println!("    Successful: {}", s);
        println!("    Failed: {}", f);
        println!("    Success Rate: {:.2}%", tag_success_rate);
    }

    println!("\n--- End Report ---");

    Ok(())
}
