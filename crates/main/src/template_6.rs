use std::sync::Arc;
use music_turtle_lang::cfg::{GrammarDerivationConfig, GrammarDerivationGenerator};
use music_turtle_lang::grammar_from_file;
use crate::{genetic_simulation, render_grammar_derivation_to_lilypond};
use crate::genetic_simulation::grammar_derivation_genome4::GrammarDerivationGenome4;
use crate::genetic_simulation::Simulation;

pub fn template_6(trials: usize, distance_bias: f64, experiment_location: &str) {
    // this uses hierarchical analysis of prolongational trees
    // --- constants ---
    let grammar_file    = &format!("{experiment_location}/grammar.mt");
    let population_size   = 100;
    let generations       = 1_000;
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

    let analysis_params = genetic_simulation::grammar_derivation_genome4::AnalysisParams4 {
        distance_bias
    };

    let config = Arc::new((generator, analysis_params));

    // --- run trials ---
    let mut trial_best_fitnesses: Vec<f64> = Vec::with_capacity(trials);
    let mut trial_avg_fitnesses: Vec<f64> = Vec::with_capacity(trials);
    let mut trial_all_time_bests: Vec<(GrammarDerivationGenome4, f64)> = Vec::with_capacity(trials);

    for trial in 0..trials {
        println!("\n=== Trial {}/{} ===", trial + 1, trials);

        let mut simulation: Simulation<GrammarDerivationGenome4> =
            Simulation::new(population_size, Arc::clone(&config), p_crossover, p_mutation);

        let mut all_time_best: Option<(GrammarDerivationGenome4, f64)> = None;

        for generation in 0..generations {
            simulation.step();

            if (generation + 1) % log_every == 0 {
                let metrics = simulation.calculate_metrics(1);
                println!("  Generation {:>4}: {}", generation + 1, metrics.summary());

                let (top_genome, top_fitness) = &metrics.best_genomes[0];
                if all_time_best.as_ref().map_or(true, |(_, f)| top_fitness > f) {
                    all_time_best = Some((top_genome.clone(), *top_fitness));
                }
            }
        }

        let final_metrics = simulation.calculate_metrics(top_n);
        println!("  Final: {}", final_metrics.summary());

        // Check final metrics against all-time best
        let (top_genome, top_fitness) = &final_metrics.best_genomes[0];
        if all_time_best.as_ref().map_or(true, |(_, f)| top_fitness > f) {
            all_time_best = Some((top_genome.clone(), *top_fitness));
        }

        // Show chord progressions for the top 5 genomes
        for (rank, (genome, fitness)) in final_metrics.best_genomes.iter().enumerate() {
            println!("  -- Top genome #{} (fitness: {:.4}):", rank + 1, fitness);
            genome.show_overall_branching_split();
            genome.show_prolongational_branching();
        }

        let (best_genome, best_fitness) = all_time_best.expect("population must be non-empty");

        println!("  -- All-time best for trial {} (fitness: {:.4}):", trial + 1, best_fitness);
        best_genome.show_overall_branching_split();
        best_genome.show_prolongational_branching();

        // Render the all-time best genome to LilyPond
        let ly_path = format!("{experiment_location}/trial_{}_best.ly", trial + 1);
        render_grammar_derivation_to_lilypond(&best_genome.0, grammar.time_signature, &ly_path);

        trial_best_fitnesses.push(best_fitness);
        trial_avg_fitnesses.push(final_metrics.average_fitness);
        trial_all_time_bests.push((best_genome, best_fitness));
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
    println!("Experiment 5 Summary ({} trials)", trials);
    println!("============================");
    println!("  Best fitness  — mean: {:.4}, std: {:.4}, min: {:.4}, max: {:.4}",
        mean_best, std_best, min_best, max_best);
    println!("  Average fitness (mean across trials): {:.4}", mean_avg);
    for (i, ((_, best_f), avg)) in trial_all_time_bests.iter().zip(trial_avg_fitnesses.iter()).enumerate() {
        println!("  Trial {:>2}: all-time best = {:.4}, avg = {:.4}", i + 1, best_f, avg);
    }
}