use music_turtle_lang::cfg::GrammarDerivation;

pub struct GrammarDerivationGenome(GrammarDerivation);

#[cfg(test)]
mod test {
    #[test]
    pub fn test_rng() {
        let mut rng = rand::rng();
    }
}