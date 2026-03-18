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

#[derive(Debug, Clone)]
pub struct GrammarDerivationGenome(pub GrammarDerivation);

impl GrammarDerivationGenome {
    pub fn show_chord_progression(&self) {
        let chord_progression = extract_symbolic_chord_structure(&self.0, CHORD_PREFIX);
        println!("{:?}", chord_progression);
    }
}
impl Genome for GrammarDerivationGenome {
    type Config = Arc<GrammarDerivationGenerator>;

    fn generate(config: &Self::Config, rng: &mut impl Rng) -> Self {
        let derivation = config.produce(rng);
        GrammarDerivationGenome(derivation)
    }

    fn mutate(&mut self, config: &Self::Config, rng: &mut impl Rng) {
        config.re_expand_random_nt(&mut self.0, rng)
    }

    fn crossover(&self, other: &Self, rng: &mut impl Rng) -> Self {
        let mut current = self.0.clone();
        let mut other = other.0.clone();
        loop {
            if let Some((self_derivation, self_depth)) =
                current.pick_random_nt_mut(rng, None::<fn(&NonTerminal) -> bool>)
            {
                let self_nt_root = self_derivation
                    .get_nt_root()
                    .expect("Dev error: Picked a non-NT root");
                let check = |dev: &NonTerminal| self_nt_root == dev;
                if let Some((other_derivation, other_depth)) =
                    other.pick_random_nt_mut(rng, Some(check))
                {
                    std::mem::swap(self_derivation, other_derivation);
                    return GrammarDerivationGenome(current);
                }
                // otherwise... choose something else (this seems like it could take a while)
                // todo: intersect NTs for each and then pick from intersection
            } else {
                // no non-terminals??
                return self.clone(); // idk man
            }
        }
    }

    fn fitness(&self) -> f64 {
        let chord_progression = extract_chord_structure(&self.0, CHORD_PREFIX);
        let max_dist = get_maximum_distance(&chord_progression, &INITIAL_CHORD);
        let horizontal_dist = get_total_interchordal_distances(&chord_progression, &INITIAL_CHORD);
        
        // want to maximize max_dist and minimize horizontal_dist, so I'll subtract them for now.
        max_dist as f64 - horizontal_dist as f64
    }
}
#[cfg(test)]
mod test {
    use super::*;
    use music_primitives::{Duration, Pitch, TimeSignature};
    use music_turtle_lang::cfg::TerminalNote::Note;
    use music_turtle_lang::cfg::{
        Grammar, MusicPrimitive, MusicString, NonTerminal, Performer, Production, Symbol, Terminal,
    };
    use num::rational::Ratio;

    #[test]
    fn test_grammar_derivation_genome() {
        let ts = TimeSignature::common();
        let grammar = Grammar {
            start: NonTerminal::Custom("S".to_string()),
            time_signature: ts,
            productions: vec![Production(
                NonTerminal::Custom("S".to_string()),
                MusicString(vec![MusicPrimitive::Simple(Symbol::T(
                    Terminal::AbsoluteSound {
                        note: Note {
                            pitch: Pitch::new(4, 0),
                        },
                        duration: Duration::from_beats_with_ts(Ratio::from_integer(1), ts),
                    },
                ))]),
            )],
        };
        let generator = GrammarDerivationGenerator {
            config: GrammarDerivationConfig {
                iterations: 4,
                panic_on_bad_production: true,
                rounded: true,
                max_depth: 4,
            },
            grammar: Arc::new(grammar),
        };
        let config = Arc::new(generator);
        let mut rng = rand::rng();
        let genome = GrammarDerivationGenome::generate(&config, &mut rng);
        println!("{:?}", genome);
    }
}
