use crate::genetic_simulation::Genome;
use music_turtle_lang::cfg::{
    Grammar, GrammarDerivation, GrammarDerivationConfig, GrammarDerivationGenerator, NonTerminal,
};
use rand::Rng;
use std::sync::Arc;
use crate::distance::pitch_class_space::PitchClassSpace;
use crate::genetic_simulation::analysis::{extract_chord_structure, extract_symbolic_chord_structure};
use crate::genetic_simulation::analysis::lerdahl::{get_maximum_distance, get_total_interchordal_distances};

pub const CHORD_PREFIX: &str = "#";
pub const INITIAL_CHORD: PitchClassSpace = PitchClassSpace::c_maj();

pub struct AnalysisParams {
    pub smooth_weight: f64,
}

#[derive(Debug, Clone)]
pub struct GrammarDerivationGenome2(pub GrammarDerivation);

impl GrammarDerivationGenome2 {
    pub fn show_chord_progression(&self) {
        let chord_progression = extract_symbolic_chord_structure(&self.0, CHORD_PREFIX);
        println!("{:?}", chord_progression);
    }
}
impl Genome for GrammarDerivationGenome2 {
    type Config = Arc<(GrammarDerivationGenerator, AnalysisParams)>;

    fn generate(config: &Self::Config, rng: &mut impl Rng) -> Self {
        let derivation = config.0.produce(rng);
        GrammarDerivationGenome2(derivation)
    }

    fn mutate(&mut self, config: &Self::Config, rng: &mut impl Rng) {
        config.0.re_expand_random_nt(&mut self.0, rng);
        // Prune after mutation in case re-expansion pushed nodes beyond max_depth.
        config.0.prune(&mut self.0);
    }

    fn crossover(&self, other: &Self, config: &Self::Config, rng: &mut impl Rng) -> Self {
        let mut current = self.0.clone();
        let mut other = other.0.clone();
        loop {
            if let Some((self_derivation, _self_depth)) =
                current.pick_random_nt_mut(rng, None::<fn(&NonTerminal) -> bool>)
            {
                let self_nt_root = self_derivation
                    .get_nt_root()
                    .expect("Dev error: Picked a non-NT root");
                let check = |dev: &NonTerminal| self_nt_root == dev;
                if let Some((other_derivation, _other_depth)) =
                    other.pick_random_nt_mut(rng, Some(check))
                {
                    std::mem::swap(self_derivation, other_derivation);
                    // Prune to enforce max_depth after the subtree swap.
                    config.0.prune(&mut current);
                    return GrammarDerivationGenome2(current);
                }
                // otherwise... choose something else (this seems like it could take a while)
                // todo: intersect NTs for each and then pick from intersection
            } else {
                // no non-terminals??
                return self.clone(); // idk man
            }
        }
    }

    fn fitness(&self, config: &Self::Config) -> f64 {
        let pms = &config.1;
        let chord_progression = extract_chord_structure(&self.0, CHORD_PREFIX);
        let max_dist = get_maximum_distance(&chord_progression, &INITIAL_CHORD) as f64;
        let horizontal_dist = get_total_interchordal_distances(&chord_progression, &INITIAL_CHORD) as f64;

        // want to maximize max_dist and minimize horizontal_dist, so I'll subtract them for now.
        max_dist - horizontal_dist * pms.smooth_weight
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use music_turtle_lang::cfg::{GrammarDerivationConfig, GrammarDerivationGenerator};
    use music_turtle_lang::grammar_from_file;
    use crate::genetic_simulation::Genome;

    fn make_config() -> Arc<(GrammarDerivationGenerator, AnalysisParams)> {
        let grammar = grammar_from_file("data/experiments/2/grammar.mt")
            .expect("grammar file must exist");
        let generator = GrammarDerivationGenerator::new(
            GrammarDerivationConfig {
                iterations: 5,
                panic_on_bad_production: false,
                rounded: true,
                max_depth: 10,
            },
            &grammar,
        );
        Arc::new((generator, AnalysisParams { smooth_weight: 0.1 }))
    }

    fn chord_count(genome: &GrammarDerivationGenome2) -> usize {
        extract_symbolic_chord_structure(&genome.0, CHORD_PREFIX).len()
    }

    /// Freshly generated genomes should always have exactly 4 chords.
    #[test]
    fn test_generate_always_produces_4_chords() {
        let config = make_config();
        let mut rng = rand::rng();
        for _ in 0..50 {
            let genome = GrammarDerivationGenome2::generate(&config, &mut rng);
            assert_eq!(chord_count(&genome), 4, "generated genome must have 4 chords");
        }
    }

    /// Crossover must preserve the 4-chord structure.
    /// Before the filter fix in pick_random_nt_mut, crossover could insert the
    /// entire S subtree of one parent into a single chord slot of the other,
    /// producing 7 chords instead of 4.
    #[test]
    fn test_crossover_preserves_chord_count() {
        let config = make_config();
        let mut rng = rand::rng();
        for _ in 0..200 {
            let p1 = GrammarDerivationGenome2::generate(&config, &mut rng);
            let p2 = GrammarDerivationGenome2::generate(&config, &mut rng);
            let child = p1.crossover(&p2, &config, &mut rng);
            assert_eq!(
                chord_count(&child), 4,
                "crossover child must have 4 chords; got {:?}",
                extract_symbolic_chord_structure(&child.0, CHORD_PREFIX)
            );
        }
    }

    /// Mutation must preserve the 4-chord structure.
    #[test]
    fn test_mutation_preserves_chord_count() {
        let config = make_config();
        let mut rng = rand::rng();
        for _ in 0..200 {
            let mut genome = GrammarDerivationGenome2::generate(&config, &mut rng);
            genome.mutate(&config, &mut rng);
            assert_eq!(
                chord_count(&genome), 4,
                "mutated genome must have 4 chords; got {:?}",
                extract_symbolic_chord_structure(&genome.0, CHORD_PREFIX)
            );
        }
    }

    /// Applying crossover and mutation repeatedly must never escape 4 chords.
    #[test]
    fn test_chord_count_stable_across_many_generations() {
        let config = make_config();
        let mut rng = rand::rng();
        let mut population: Vec<GrammarDerivationGenome2> = (0..20)
            .map(|_| GrammarDerivationGenome2::generate(&config, &mut rng))
            .collect();

        for _gen in 0..20 {
            let mut next = Vec::with_capacity(population.len());
            for i in 0..population.len() {
                let j = (i + 1) % population.len();
                let mut child = population[i].crossover(&population[j], &config, &mut rng);
                child.mutate(&config, &mut rng);
                let n = chord_count(&child);
                assert_eq!(n, 4, "genome after crossover+mutation in generation has {} chords", n);
                next.push(child);
            }
            population = next;
        }
    }
}
