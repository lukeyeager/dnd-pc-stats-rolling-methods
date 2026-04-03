mod methods;

use clap::{Parser, Subcommand};
use colored::Colorize;
use methods::{METHOD_NAMES, roll_method};
use std::collections::HashMap;

// --- CLI ---

#[derive(Parser)]
#[command(about = "Analyze D&D ability score rolling methods via simulation")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// List available rolling methods
    List,
    /// Roll stats once using the given method
    Once { method: String },
    /// Simulate all methods and print statistics
    Stats {
        #[arg(short, long, default_value_t = 10_000)]
        iters: u32,
    },
}

// --- Color helpers ---
// IMPORTANT: always pass an already-padded string so ANSI codes don't affect alignment.

/// Heat-map color for a percentage, relative to the column's peak value.
/// dim → blue → cyan → yellow → bright red as the value rises toward the column peak.
fn colorize_pct(s: &str, ratio: f64) -> String {
    if ratio < 0.2 {
        s.dimmed().to_string()
    } else if ratio < 0.4 {
        s.blue().to_string()
    } else if ratio < 0.6 {
        s.cyan().to_string()
    } else if ratio < 0.8 {
        s.yellow().to_string()
    } else {
        s.bright_red().to_string()
    }
}

/// Color an average value relative to the range [min, max] across all methods.
fn colorize_avg(s: &str, ratio: f64) -> String {
    if ratio < 0.2 {
        s.blue().to_string()
    } else if ratio < 0.4 {
        s.cyan().to_string()
    } else if ratio < 0.6 {
        s.white().to_string()
    } else if ratio < 0.8 {
        s.yellow().to_string()
    } else {
        s.bright_red().to_string()
    }
}

// --- Actions ---

fn action_list() {
    for name in METHOD_NAMES {
        println!("{}", name);
    }
}

fn action_once(method: &str) {
    let mut rng = rand::rng();
    let stats = roll_method(method, &mut rng);
    let parts: Vec<String> = stats.iter().map(|s| format!("{:2}", s)).collect();
    println!("{}", parts.join(" "));
}

/// Per-method stats: [all, top1, top2, top3, top4, top5, top6]
struct MethodStats {
    name: &'static str,
    counters: [HashMap<u32, u64>; 7],
}

const SEP: &str = "  ";
// Percentages are at most "100.00" (6 chars); we use that as the minimum column width.
const PCT_WIDTH: usize = 6;

fn action_stats(iters: u32) {
    let mut rng = rand::rng();

    let mut all_stats: Vec<MethodStats> = METHOD_NAMES
        .iter()
        .map(|&name| MethodStats {
            name,
            counters: std::array::from_fn(|_| HashMap::new()),
        })
        .collect();

    for ms in &mut all_stats {
        for _ in 0..iters {
            let mut stats = roll_method(ms.name, &mut rng);
            stats.sort_unstable_by(|a, b| b.cmp(a)); // descending
            for (i, &s) in stats.iter().enumerate() {
                *ms.counters[0].entry(s).or_insert(0) += 1;
                *ms.counters[i + 1].entry(s).or_insert(0) += 1;
            }
        }
    }

    let field_names = ["all", "top1", "top2", "top3", "top4", "top5", "top6"];
    let method_names: Vec<&str> = all_stats.iter().map(|ms| ms.name).collect();

    // Column width = max(method name length, PCT_WIDTH) so headers and values align.
    let col_widths: Vec<usize> = method_names.iter().map(|n| n.len().max(PCT_WIDTH)).collect();

    // Collect raw averages for the summary table (must stay uncolored until print time).
    let mut summary_avgs: Vec<Vec<f64>> = Vec::new();

    for (fi, &field) in field_names.iter().enumerate() {
        let totals: Vec<u64> = all_stats
            .iter()
            .map(|ms| ms.counters[fi].values().sum())
            .collect();

        // Pre-compute all pct values so we can find per-column maxes for normalization.
        let pct_grid: Vec<Vec<f64>> = (3u32..=18)
            .map(|s| {
                all_stats
                    .iter()
                    .enumerate()
                    .map(|(i, ms)| {
                        let count = ms.counters[fi].get(&s).copied().unwrap_or(0);
                        count as f64 * 100.0 / totals[i] as f64
                    })
                    .collect()
            })
            .collect();

        let col_maxes: Vec<f64> = (0..all_stats.len())
            .map(|i| pct_grid.iter().map(|row| row[i]).fold(0.0_f64, f64::max))
            .collect();

        // Header
        let header_cols: Vec<String> = method_names
            .iter()
            .zip(&col_widths)
            .map(|(name, &w)| format!("{:>w$}", name))
            .collect();
        println!(">>> {}", field);
        println!("{:4}{}{}", "stat", SEP, header_cols.join(SEP));

        for (row_idx, s) in (3u32..=18).enumerate() {
            let values: Vec<String> = pct_grid[row_idx]
                .iter()
                .enumerate()
                .map(|(i, &pct)| {
                    let ratio = if col_maxes[i] > 0.0 { pct / col_maxes[i] } else { 0.0 };
                    // Pad first, then colorize so ANSI codes don't affect column width.
                    let padded = format!("{:>width$.2}", pct, width = col_widths[i]);
                    colorize_pct(&padded, ratio)
                })
                .collect();
            println!("{:4}{}{}", s, SEP, values.join(SEP));
        }
        println!();

        // Averages
        let avg_values: Vec<f64> = all_stats
            .iter()
            .map(|ms| {
                let counter = &ms.counters[fi];
                let n: u64 = counter.values().sum();
                let s: u64 = counter.iter().map(|(&k, &v)| k as u64 * v).sum();
                s as f64 / n as f64
            })
            .collect();

        let avg_min = avg_values.iter().cloned().fold(f64::MAX, f64::min);
        let avg_max = avg_values.iter().cloned().fold(f64::MIN, f64::max);
        let avg_range = avg_max - avg_min;

        println!("> averages");
        for (name, &v) in method_names.iter().zip(&avg_values) {
            let ratio = if avg_range > 0.0 { (v - avg_min) / avg_range } else { 0.5 };
            println!("{}: {}", name, colorize_avg(&format!("{:.2}", v), ratio));
        }
        println!();

        summary_avgs.push(avg_values);
    }

    // Summary table — re-color from raw values so alignment is correct.
    let field_col_width = field_names.iter().map(|n| n.len()).max().unwrap();
    let header_cols: Vec<String> = method_names
        .iter()
        .zip(&col_widths)
        .map(|(name, &w)| format!("{:>w$}", name))
        .collect();
    println!(">>> summary avgs");
    println!("{:width$}{}{}", "stat", SEP, header_cols.join(SEP), width = field_col_width);

    for (fi, &field) in field_names.iter().enumerate() {
        let avgs = &summary_avgs[fi];
        let avg_min = avgs.iter().cloned().fold(f64::MAX, f64::min);
        let avg_max = avgs.iter().cloned().fold(f64::MIN, f64::max);
        let avg_range = avg_max - avg_min;

        let values: Vec<String> = avgs
            .iter()
            .zip(&col_widths)
            .map(|(&v, &w)| {
                let ratio = if avg_range > 0.0 { (v - avg_min) / avg_range } else { 0.5 };
                let padded = format!("{:>w$.2}", v);
                colorize_avg(&padded, ratio)
            })
            .collect();
        println!("{:width$}{}{}", field, SEP, values.join(SEP), width = field_col_width);
    }
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Command::List => action_list(),
        Command::Once { method } => action_once(&method),
        Command::Stats { iters } => action_stats(iters),
    }
}
