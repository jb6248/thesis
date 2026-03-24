use std::path::Path;
use rand::Rng;
use crate::cfg::{ComposeError, Grammar, MusicPrimitive, MusicString, GrammarDerivationConfig, Performer};
use crate::composition::Composition;
use crate::scan::ScanError;

pub mod scan;
pub mod cfg;
pub mod composition;
pub mod lilypond;

macro_rules! into_custom_error(
    ($err_type:ident, $variant:ident, $variant_err_type:ty) => {
        impl From<$variant_err_type> for $err_type {
            fn from(err: $variant_err_type) -> Self {
                $err_type::$variant(err)
            }
        }
    };
);

#[derive(Debug)]
pub enum RenderError {
    IoError(std::io::Error),
    ScanError(ScanError),
    ComposeError(ComposeError),
}
into_custom_error!(RenderError, IoError, std::io::Error);
into_custom_error!(RenderError, ScanError, ScanError);
into_custom_error!(RenderError, ComposeError, ComposeError);

pub fn compose_from_grammar(grammar_filename: &str, config: GrammarDerivationConfig, rng: &mut impl Rng) -> Result<Composition, RenderError> {
    let contents = std::fs::read_to_string(grammar_filename)?;
    let grammar = contents.parse::<Grammar>()?;
    let final_string = grammar.produce(&config, rng);
    let mut composition = final_string.compose_v2(grammar.time_signature, Performer::default())?;
    if config.rounded {
        composition.add_rests_to_last_measure();
    }
    Ok(composition)
}

pub fn grammar_from_file(grammar_filename: &str) -> Result<Grammar, RenderError> {
    let contents = std::fs::read_to_string(grammar_filename)?;
    let grammar = contents.parse::<Grammar>()?;
    Ok(grammar)
}
