use crate::genetic_simulation::Genome;
use music_turtle_lang::cfg::{
    Grammar, GrammarDerivation, GrammarDerivationConfig, GrammarDerivationGenerator, NonTerminal,
};
use rand::Rng;
use std::sync::Arc;
use crate::distance::pitch_class_space::PitchClassSpace;
use crate::genetic_simulation::analysis::{extract_chord_structure, extract_symbolic_chord_structure};
use crate::genetic_simulation::analysis::lerdahl::{get_maximum_distance, get_total_interchordal_distances};
use crate::genetic_simulation::analysis::prolongational_tree::find_best_tension_relaxation_split_and_score;

pub const CHORD_PREFIX: &str = "#";
pub const INITIAL_CHORD: PitchClassSpace = PitchClassSpace::c_maj();

pub struct AnalysisParams2 {
}

#[derive(Debug, Clone)]
pub struct GrammarDerivationGenome3(pub GrammarDerivation);

impl GrammarDerivationGenome3 {
    pub fn show_chord_progression(&self) {
        let chord_progression = extract_symbolic_chord_structure(&self.0, CHORD_PREFIX);
        println!("{:?}", chord_progression);
    }
}
impl Genome for GrammarDerivationGenome3 {
    type Config = Arc<(GrammarDerivationGenerator, AnalysisParams2)>;

    fn generate(config: &Self::Config, rng: &mut impl Rng) -> Self {
        let derivation = config.0.produce(rng);
        GrammarDerivationGenome3(derivation)
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
                    return GrammarDerivationGenome3(current);
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
        best_score as f64 / total_branchings
    }
}
