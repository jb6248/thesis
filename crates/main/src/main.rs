mod genetic_simulation;
mod distance;
mod util;

use std::sync::Arc;
use music_primitives::{Pitch, TimeSignature};
use music_turtle_lang::cfg::{GrammarDerivation, GrammarDerivationConfig, GrammarDerivationGenerator, Performer};
use music_turtle_lang::composition::Instrument::Piano;
use music_turtle_lang::composition::Volume;
use music_turtle_lang::grammar_from_file;
use music_turtle_lang::lilypond::{call_lilypond_cli, render_to_lilypond, LilyPondConfig};
use crate::genetic_simulation::{Simulation, GrammarDerivationGenome};
use crate::genetic_simulation::analysis::extract_symbolic_chord_structure;
use crate::genetic_simulation::grammar_derivation_genome2::{AnalysisParams, GrammarDerivationGenome2};
use crate::genetic_simulation::grammar_derivation_genome3::{AnalysisParams2, GrammarDerivationGenome3};

fn main() {
    // we are going to run a simulation
    experiment_3(1);
}

/// Renders a GrammarDerivationGenome2 to a LilyPond file and compiles it.
/// `output_path` should be the full path to the .ly file (e.g. "data/experiments/2/trial_1_best.ly").
fn render_grammar_derivation_to_lilypond(derivation: &GrammarDerivation, time_signature: TimeSignature, output_path: &str) {
    let output_dir = std::path::Path::new(output_path)
        .parent()
        .and_then(|p| p.to_str())
        .expect("Invalid output path");
    let performer = Performer {
        instrument: Piano,
        volume: Volume(50),
        pitch: Pitch::middle_c(),
    };
    let music_string = derivation.to_music_string();
    let mut composition = music_string.compose_v2(time_signature, performer).unwrap();
    composition.transpose(24); // to make it easy to read
    let composition = composition;
    render_to_lilypond(
        composition,
        output_path,
        Some(LilyPondConfig {
            write_dynamics: false,
            piano_staff: true,
            ..LilyPondConfig::default()
        })
    ).unwrap();
    call_lilypond_cli(output_path, &format!("{output_dir}/"), false).unwrap();
}

fn experiment_1() {
    const EXPERIMENT_LOCATION: &str = "data/experiments/initial";
    // --- constants ---
    let grammar_file    = &format!("{EXPERIMENT_LOCATION}/grammar.mt");
    let population_size   = 1000;
    let generations       = 100;
    let p_crossover       = 0.7;
    let p_mutation        = 0.2;
    let iterations        = 5;
    let max_depth         = 10;
    let log_every         = 10;   // print metrics every N generations

    // --- setup ---
    let grammar = grammar_from_file(grammar_file)
        .expect("Failed to load grammar file");

    let generator = GrammarDerivationGenerator::new(
        GrammarDerivationConfig {
            iterations,
            panic_on_bad_production: false,
            rounded: true,
            max_depth,
        },
        &grammar,
    );

    let config = Arc::new(generator);

    let mut simulation: Simulation<GrammarDerivationGenome> =
        Simulation::new(population_size, config, p_crossover, p_mutation);

    // --- run ---
    for generation in 0..generations {
        simulation.step();

        if (generation + 1) % log_every == 0 {
            let metrics = simulation.calculate_metrics(1);
            println!("Generation {:>4}: {}", generation + 1, metrics.summary());
        }
    }

    let final_metrics = simulation.calculate_metrics(1);
    println!("\nFinal: {}", final_metrics.summary());

    let (best_genome, fitness) = &final_metrics.best_genomes[0];
    println!("Best genome fitness: {}", fitness);

    // Print chord progression of best genome
    best_genome.show_chord_progression();
    // Output best genome to file by rendering and using lilypond
    let performer = Performer {
        instrument: Piano,
        volume: Volume(50),
        pitch: Pitch::middle_c(),
    };
    let music_string = best_genome.0.to_music_string();
    let mut composition = music_string.compose_v2(grammar.time_signature, performer).unwrap();
    composition.transpose(24); // to make it easy to read
    let composition = composition;
    let lilypond_output_path = format!("{EXPERIMENT_LOCATION}/best_genome.ly");
    render_to_lilypond(
        composition,
        &lilypond_output_path,
        Some(LilyPondConfig {
            write_dynamics: false,
            ..LilyPondConfig::default()
        })
    ).unwrap();
    call_lilypond_cli(&lilypond_output_path, &format!("{EXPERIMENT_LOCATION}/"), true).unwrap();
}

fn experiment_2(trials: usize) {
    // this adds a weight to the smoothness
    const EXPERIMENT_LOCATION: &str = "data/experiments/2";
    // --- constants ---
    let grammar_file    = &format!("{EXPERIMENT_LOCATION}/grammar.mt");
    let population_size   = 1000;
    let generations       = 100;
    let p_crossover       = 0.7;
    let p_mutation        = 0.2;
    let iterations        = 5;
    let max_depth         = 10;

    // analysis
    let smooth_weight = 0.1; // weight for smoothing fitness scores based on distance to neighbors


    // logs
    let log_every = 20; // print metrics every N generations
    let top_n = 5; // how many of the best genomes to show at the end of each trial

    // --- setup ---
    let grammar = grammar_from_file(grammar_file)
        .expect("Failed to load grammar file");

    let generator = GrammarDerivationGenerator::new(
        GrammarDerivationConfig {
            iterations,
            panic_on_bad_production: false,
            rounded: true,
            max_depth,
        },
        &grammar,
    );

    let analysis_params = AnalysisParams {
        smooth_weight,
    };

    let config = Arc::new((generator, analysis_params));

    // --- run trials ---
    let mut trial_best_fitnesses: Vec<f64> = Vec::with_capacity(trials);
    let mut trial_avg_fitnesses: Vec<f64> = Vec::with_capacity(trials);

    for trial in 0..trials {
        println!("\n=== Trial {}/{} ===", trial + 1, trials);

        let mut simulation: Simulation<GrammarDerivationGenome2> =
            Simulation::new(population_size, Arc::clone(&config), p_crossover, p_mutation);

        for generation in 0..generations {
            simulation.step();

            if (generation + 1) % log_every == 0 {
                let metrics = simulation.calculate_metrics(1);
                println!("  Generation {:>4}: {}", generation + 1, metrics.summary());
            }
        }

        let final_metrics = simulation.calculate_metrics(top_n);
        println!("  Final: {}", final_metrics.summary());

        // Show chord progressions for the top 5 genomes
        for (rank, (genome, fitness)) in final_metrics.best_genomes.iter().enumerate() {
            println!("  -- Top genome #{} (fitness: {:.4}):", rank + 1, fitness);
            genome.show_chord_progression();
        }

        // Render the best genome to LilyPond
        let (best_genome, _) = &final_metrics.best_genomes[0];
        let ly_path = format!("{EXPERIMENT_LOCATION}/trial_{}_best.ly", trial + 1);
        render_grammar_derivation_to_lilypond(&best_genome.0, grammar.time_signature, &ly_path);

        trial_best_fitnesses.push(final_metrics.best_fitness);
        trial_avg_fitnesses.push(final_metrics.average_fitness);
    }

    // --- summarize ---
    let n = trials as f64;
    let mean_best = trial_best_fitnesses.iter().sum::<f64>() / n;
    let mean_avg  = trial_avg_fitnesses.iter().sum::<f64>() / n;
    let min_best  = trial_best_fitnesses.iter().cloned().fold(f64::INFINITY, f64::min);
    let max_best  = trial_best_fitnesses.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let std_best  = {
        let variance = trial_best_fitnesses.iter().map(|x| (x - mean_best).powi(2)).sum::<f64>() / n;
        variance.sqrt()
    };

    println!("\n============================");
    println!("Experiment 2 Summary ({} trials)", trials);
    println!("============================");
    println!("  Best fitness  — mean: {:.4}, std: {:.4}, min: {:.4}, max: {:.4}",
        mean_best, std_best, min_best, max_best);
    println!("  Average fitness (mean across trials): {:.4}", mean_avg);
    for (i, (best, avg)) in trial_best_fitnesses.iter().zip(trial_avg_fitnesses.iter()).enumerate() {
        println!("  Trial {:>2}: best = {:.4}, avg = {:.4}", i + 1, best, avg);
    }
}

fn experiment_3(trials: usize) {
    // this uses hierarchical analysis of prolongational trees
    const EXPERIMENT_LOCATION: &str = "data/experiments/3";
    // --- constants ---
    let grammar_file    = &format!("{EXPERIMENT_LOCATION}/grammar.mt");
    let population_size   = 100;
    let generations       = 100;
    let p_crossover       = 0.7;
    let p_mutation        = 0.2;
    let iterations        = 5;
    let max_depth         = 10;

    // logs
    let log_every = 20; // print metrics every N generations
    let top_n = 5; // how many of the best genomes to show at the end of each trial

    // --- setup ---
    let grammar = grammar_from_file(grammar_file)
        .expect("Failed to load grammar file");

    let generator = GrammarDerivationGenerator::new(
        GrammarDerivationConfig {
            iterations,
            panic_on_bad_production: false,
            rounded: true,
            max_depth,
        },
        &grammar,
    );

    let analysis_params = AnalysisParams2 {
    };

    let config = Arc::new((generator, analysis_params));

    // --- run trials ---
    let mut trial_best_fitnesses: Vec<f64> = Vec::with_capacity(trials);
    let mut trial_avg_fitnesses: Vec<f64> = Vec::with_capacity(trials);

    for trial in 0..trials {
        println!("\n=== Trial {}/{} ===", trial + 1, trials);

        let mut simulation: Simulation<GrammarDerivationGenome3> =
            Simulation::new(population_size, Arc::clone(&config), p_crossover, p_mutation);

        for generation in 0..generations {
            simulation.step();

            if (generation + 1) % log_every == 0 {
                let metrics = simulation.calculate_metrics(1);
                println!("  Generation {:>4}: {}", generation + 1, metrics.summary());
            }
        }

        let final_metrics = simulation.calculate_metrics(top_n);
        println!("  Final: {}", final_metrics.summary());

        // Show chord progressions for the top 5 genomes
        for (rank, (genome, fitness)) in final_metrics.best_genomes.iter().enumerate() {
            println!("  -- Top genome #{} (fitness: {:.4}):", rank + 1, fitness);
            genome.show_chord_progression();
        }

        // Render the best genome to LilyPond
        let (best_genome, _) = &final_metrics.best_genomes[0];
        let ly_path = format!("{EXPERIMENT_LOCATION}/trial_{}_best.ly", trial + 1);
        render_grammar_derivation_to_lilypond(&best_genome.0, grammar.time_signature, &ly_path);

        trial_best_fitnesses.push(final_metrics.best_fitness);
        trial_avg_fitnesses.push(final_metrics.average_fitness);
    }

    // --- summarize ---
    let n = trials as f64;
    let mean_best = trial_best_fitnesses.iter().sum::<f64>() / n;
    let mean_avg  = trial_avg_fitnesses.iter().sum::<f64>() / n;
    let min_best  = trial_best_fitnesses.iter().cloned().fold(f64::INFINITY, f64::min);
    let max_best  = trial_best_fitnesses.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let std_best  = {
        let variance = trial_best_fitnesses.iter().map(|x| (x - mean_best).powi(2)).sum::<f64>() / n;
        variance.sqrt()
    };

    println!("\n============================");
    println!("Experiment 2 Summary ({} trials)", trials);
    println!("============================");
    println!("  Best fitness  — mean: {:.4}, std: {:.4}, min: {:.4}, max: {:.4}",
        mean_best, std_best, min_best, max_best);
    println!("  Average fitness (mean across trials): {:.4}", mean_avg);
    for (i, (best, avg)) in trial_best_fitnesses.iter().zip(trial_avg_fitnesses.iter()).enumerate() {
        println!("  Trial {:>2}: best = {:.4}, avg = {:.4}", i + 1, best, avg);
    }
}