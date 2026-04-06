use std::fmt::Display;
use crate::genetic_simulation::Genome;
use music_turtle_lang::cfg::{
    Grammar, GrammarDerivation, GrammarDerivationConfig, GrammarDerivationGenerator, NonTerminal,
};
use rand::Rng;
use std::sync::Arc;
use display_tree::println_tree;
use crate::distance::pitch_class_space::PitchClassSpace;
use crate::genetic_simulation::analysis::{extract_chord_structure, extract_symbolic_chord_structure};
use crate::genetic_simulation::analysis::lerdahl::{get_maximum_distance, get_total_interchordal_distances};
use crate::genetic_simulation::analysis::prolongational_tree::find_best_tension_relaxation_split_and_score;

pub const CHORD_PREFIX: &str = "#";
pub const INITIAL_CHORD: PitchClassSpace = PitchClassSpace::c_maj();

pub struct AnalysisParams4 {
    /// How much to influence distance of farthest chord in fitness
    pub(crate) distance_bias: f64
}

#[derive(Debug, Clone)]
pub struct GrammarDerivationGenome4(pub GrammarDerivation);

fn print_vec_plain<T: Display>(vec: Vec<T>, delimiter: &str) {

}

impl GrammarDerivationGenome4 {
    pub fn show_chord_progression(&self) {
        let chord_progression = extract_symbolic_chord_structure(&self.0, CHORD_PREFIX);
        println!("{:?}", chord_progression.join(" "));
    }

    /// Determine the best prolongational branching and show where it splits!
    pub fn show_overall_branching_split(&self) {
        let symbols = extract_symbolic_chord_structure(&self.0, CHORD_PREFIX);
        let chord_progression = extract_chord_structure(&self.0, CHORD_PREFIX);
        let (score, left, right) = find_best_tension_relaxation_split_and_score(&chord_progression, &INITIAL_CHORD);
        let left_size = left.count_leaves();
        assert_eq!(left.count_leaves() + right.count_leaves(), symbols.len(), "Leaves should add up to total symbols");
        println!("{} | {}", (&symbols[..left_size]).join(" "), (&symbols[left_size..]).join(" "));
    }

    pub fn show_prolongational_branching(&self) {
        let symbols = extract_symbolic_chord_structure(&self.0, CHORD_PREFIX);
        let chord_progression = extract_chord_structure(&self.0, CHORD_PREFIX);
        let (score, left, right) = find_best_tension_relaxation_split_and_score(&chord_progression, &INITIAL_CHORD);
        println!("left:");
        println_tree!(left);
        println!("right:");
        println_tree!(right);
    }
}
impl Genome for GrammarDerivationGenome4 {
    type Config = Arc<(GrammarDerivationGenerator, AnalysisParams4)>;

    fn generate(config: &Self::Config, rng: &mut impl Rng) -> Self {
        let derivation = config.0.produce(rng);
        GrammarDerivationGenome4(derivation)
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
                    return GrammarDerivationGenome4(current);
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
        let (best_score, left_tree, right_tree) = find_best_tension_relaxation_split_and_score(
            &chord_progression,
            &INITIAL_CHORD);
        let total_branchings = chord_progression.len() as f64 - 2.;
        let tension_score = best_score as f64 / total_branchings;
        let ending_tonic_penalty = chord_progression.last()
            .map(|last| INITIAL_CHORD.total_distance(last))
            .unwrap_or(0) as f64;
        
        let farthest_chord_dist = chord_progression.iter()
            .map(|chord| INITIAL_CHORD.total_distance(chord))
            .max()
            .unwrap_or(0) as f64;
        
        // we like distance, so we'll add it to the tension score
        tension_score + farthest_chord_dist * pms.distance_bias - ending_tonic_penalty
    }
}
