mod methods;

use clap::{Parser, Subcommand};
use methods::{METHOD_NAMES, roll_method};
use std::collections::HashMap;
use yansi::Paint;

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

/// Viridis-tracing xterm-256 indices, dark purple → blue → teal → green → yellow.
fn viridis(ratio: f64) -> u8 {
    const STOPS: &[u8] = &[
        53,  // (95,   0,  95) dark purple
        55,  // (95,   0, 175) purple
        61,  // (95,  95, 175) blue-purple
        25,  // ( 0,  95, 175) blue
        31,  // ( 0, 135, 175) blue-teal
        30,  // ( 0, 135, 135) teal
        36,  // ( 0, 175, 135) teal-green
        71,  // (95, 175,  95) green
        76,  // (95, 215,   0) yellow-green
        148, // (175,215,   0) lime
        220, // (255,215,   0) yellow
    ];
    let i = (ratio.clamp(0.0, 1.0) * (STOPS.len() - 1) as f64).round() as usize;
    STOPS[i.min(STOPS.len() - 1)]
}

fn colorize_pct(s: &str, ratio: f64) -> String {
    s.fixed(viridis(ratio)).to_string()
}

fn colorize_avg(s: &str, ratio: f64) -> String {
    s.fixed(viridis(ratio)).to_string()
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
    let col_widths: Vec<usize> = method_names
        .iter()
        .map(|n| n.len().max(PCT_WIDTH))
        .collect();

    // Collect raw averages for the summary table (must stay uncolored until print time).
    let mut summary_avgs: Vec<Vec<f64>> = Vec::new();

    for (fi, &field) in field_names.iter().enumerate() {
        let totals: Vec<u64> = all_stats
            .iter()
            .map(|ms| ms.counters[fi].values().sum())
            .collect();

        // Pre-compute all pct values so we can find the global max for normalization.
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

        let grid_max: f64 = pct_grid
            .iter()
            .flat_map(|row| row.iter())
            .cloned()
            .fold(0.0_f64, f64::max);

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
                .zip(&col_widths)
                .map(|(&pct, &w)| {
                    let ratio = if grid_max > 0.0 { pct / grid_max } else { 0.0 };
                    // Pad first, then colorize so ANSI codes don't affect column width.
                    let padded = format!("{:>w$.2}", pct);
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

        let max_name_len = method_names.iter().map(|n| n.len()).max().unwrap_or(0);
        println!("> averages");
        for (name, &v) in method_names.iter().zip(&avg_values) {
            let ratio = if avg_range > 0.0 {
                (v - avg_min) / avg_range
            } else {
                0.5
            };
            let padded = format!("{:5.2}", v);
            println!(
                "{name}:{:width$}{}",
                "",
                colorize_avg(&padded, ratio),
                width = max_name_len - name.len() + 2
            );
        }
        println!();

        summary_avgs.push(avg_values);
    }

    // Summary table (transposed) — rows=methods, cols=fields, per-row normalization.
    let method_col_width = method_names.iter().map(|n| n.len()).max().unwrap();
    let field_col_widths: Vec<usize> = field_names.iter().map(|n| n.len().max(PCT_WIDTH)).collect();
    let header_cols: Vec<String> = field_names
        .iter()
        .zip(&field_col_widths)
        .map(|(name, &w)| format!("{:>w$}", name))
        .collect();
    println!(">>> summary avgs");
    println!(
        "{:width$}{}{}",
        "method",
        SEP,
        header_cols.join(SEP),
        width = method_col_width
    );

    let col_mins: Vec<f64> = (0..field_names.len())
        .map(|fi| summary_avgs[fi].iter().cloned().fold(f64::MAX, f64::min))
        .collect();
    let col_maxs: Vec<f64> = (0..field_names.len())
        .map(|fi| summary_avgs[fi].iter().cloned().fold(f64::MIN, f64::max))
        .collect();

    for (mi, &method) in method_names.iter().enumerate() {
        let values: Vec<String> = (0..field_names.len())
            .zip(&field_col_widths)
            .map(|(fi, &w)| {
                let v = summary_avgs[fi][mi];
                let range = col_maxs[fi] - col_mins[fi];
                let ratio = if range > 0.0 {
                    (v - col_mins[fi]) / range
                } else {
                    0.5
                };
                let padded = format!("{:>w$.2}", v);
                colorize_avg(&padded, ratio)
            })
            .collect();
        println!(
            "{:width$}{}{}",
            method,
            SEP,
            values.join(SEP),
            width = method_col_width
        );
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
