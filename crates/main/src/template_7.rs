use std::sync::Arc;
use std::process::Command;
use rayon::prelude::*;
use music_turtle_lang::cfg::{GrammarDerivationConfig, GrammarDerivationGenerator};
use music_turtle_lang::grammar_from_file;
use crate::{genetic_simulation, render_grammar_derivation_to_lilypond};
use crate::genetic_simulation::grammar_derivation_genome4::GrammarDerivationGenome4;
use crate::genetic_simulation::grammar_derivation_genome5::{AnalysisParams5, GrammarDerivationGenome5};
use crate::genetic_simulation::{Genome, Simulation};

fn find_or_copy_soundfont(experiment_location: &str) -> String {
    let soundfont_dest = format!("{experiment_location}/soundfont.sf2");
    if std::path::Path::new(&soundfont_dest).exists() {
        return soundfont_dest;
    }
    let output = Command::new("locate")
        .arg("--existing")
        .arg("--limit").arg("1")
        .arg(".sf2")
        .output()
        .expect("Failed to run locate to find a soundfont");
    let stdout = String::from_utf8_lossy(&output.stdout);
    let found = stdout.lines().next()
        .expect("No .sf2 soundfont found via locate; install one (e.g. fluid-soundfont-gm)")
        .trim();
    println!("Copying soundfont from {} to {}", found, soundfont_dest);
    std::fs::copy(found, &soundfont_dest).expect("Failed to copy soundfont");
    soundfont_dest
}

pub fn template_7(trials: usize, distance_bias: f64, experiment_location: &str) {
    // this uses hierarchical analysis of prolongational trees
    // --- constants ---
    let grammar_file    = &format!("{experiment_location}/grammar.mt");
    let population_size   = 200;
    let generations       = 1_000;
    let p_crossover       = 0.5;
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

    let analysis_params = AnalysisParams5 {
        distance_bias
    };

    let config = Arc::new((generator, analysis_params));

    // --- run trials ---
    let mut trial_best_fitnesses: Vec<f64> = Vec::with_capacity(trials);
    let mut trial_avg_fitnesses: Vec<f64> = Vec::with_capacity(trials);
    let mut trial_all_time_bests: Vec<(GrammarDerivationGenome5, f64)> = Vec::with_capacity(trials);
    let mut last_simulation: Option<Simulation<GrammarDerivationGenome5>> = None;

    for trial in 0..trials {
        println!("\n=== Trial {}/{} ===", trial + 1, trials);

        let mut simulation: Simulation<GrammarDerivationGenome5> =
            Simulation::new(population_size, Arc::clone(&config), p_crossover, p_mutation);

        let mut all_time_best: Option<(GrammarDerivationGenome5, f64)> = None;

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
        last_simulation = Some(simulation);
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

    // --- Render entire final population and produce MP3 samples ---
    let simulation = last_simulation.expect("at least one trial must have run");
    let lily_dir    = format!("{experiment_location}/lilypond_output");
    let samples_dir = format!("{experiment_location}/samples");
    std::fs::create_dir_all(&lily_dir).expect("Failed to create lilypond_output/");
    let soundfont = find_or_copy_soundfont(experiment_location);
    let time_sig = grammar.time_signature;

    // Sort population by fitness descending so index 0 = best
    let mut ranked: Vec<(&_, f64)> = simulation.get_population()
        .par_iter()
        .map(|genome| (genome, genome.fitness(&config)))
        .collect();
    ranked.sort_by(|(_, a), (_, b)| b.partial_cmp(a).unwrap());

    println!("\nRendering {} genomes...", ranked.len());
    ranked.par_iter().enumerate().for_each(|(index, (genome, _))| {
        // 1. Render .ly; lilypond writes .midi + .pdf into lily_dir
        let ly_path = format!("{lily_dir}/genome_{index}.ly");
        render_grammar_derivation_to_lilypond(&genome.0, time_sig, &ly_path);

        // 2. Render .midi -> .mp3 via fluidsynth into samples/
        let midi_path = format!("{lily_dir}/genome_{index}.midi");
        let mp3_path  = format!("{samples_dir}/genome_{index}.mp3");
        let status = Command::new("fluidsynth")
            .args(["-ni", "-r", "44100", "-F", &mp3_path, &soundfont, &midi_path])
            .status()
            .unwrap_or_else(|e| panic!("Failed to launch fluidsynth for genome {index}: {e}"));

        if !status.success() {
            eprintln!("fluidsynth exited with non-zero status for genome {index}");
        }
    });

    let csv_contents: String = ranked.iter().enumerate()
        .map(|(i, (_, f))| format!("{i},{f}"))
        .collect::<Vec<_>>()
        .join("\n");
    std::fs::write(format!("{experiment_location}/fitness.csv"), csv_contents)
        .expect("Failed to write fitness.csv");

    println!("Done. Lilypond output -> {lily_dir}/  |  MP3 -> {samples_dir}/  |  fitness.csv written");
}