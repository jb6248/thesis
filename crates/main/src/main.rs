mod genetic_simulation;
mod distance;
mod util;

use std::sync::Arc;
use music_primitives::Pitch;
use music_turtle_lang::cfg::{GrammarDerivationConfig, GrammarDerivationGenerator, Performer};
use music_turtle_lang::composition::Instrument::Piano;
use music_turtle_lang::composition::Volume;
use music_turtle_lang::grammar_from_file;
use music_turtle_lang::lilypond::{call_lilypond_cli, render_to_lilypond, LilyPondConfig};
use crate::genetic_simulation::{Simulation, GrammarDerivationGenome};
use crate::genetic_simulation::analysis::extract_symbolic_chord_structure;
use crate::genetic_simulation::grammar_derivation_genome2::{AnalysisParams, GrammarDerivationGenome2};

fn main() {
    // we are going to run a simulation
    experiment_2(5);
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
    let log_every         = 10;   // print metrics every N generations
    
    // analysis
    let smooth_weight = 0.1; // weight for smoothing fitness scores based on distance to neighbors

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

    let mut simulation: Simulation<GrammarDerivationGenome2> =
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