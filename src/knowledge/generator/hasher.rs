use crate::knowledge::hash::Blake3HashEngine;
use crate::knowledge::traits::HashEngine;

/// Simple wrapper exposing the project's Blake3 hasher for generator.
#[derive(Debug)]
pub struct GeneratorHasher {
    engine: Box<dyn HashEngine>,
}

impl GeneratorHasher {
    pub fn new() -> Self {
        Self {
            engine: Box::new(Blake3HashEngine::new()),
        }
    }

    pub fn hash(&self, data: &[u8]) -> String {
        self.engine.hash(data)
    }
}
