use crate::knowledge::errors::KnowledgeResult;
use crate::knowledge::loader::FileLoader;
use crate::knowledge::storage::InMemoryKnowledgeStorage;
use crate::knowledge::chunker::FixedChunker;
use crate::knowledge::hash::Blake3HashEngine;
use crate::knowledge::pipeline::KnowledgePipeline;
use crate::knowledge::traits::{DocumentLoader, DocumentValidator, Chunker, HashEngine, KnowledgeStorage};

/// Builder-style pipeline assembly for the generator. Defaults to real components.
pub struct GeneratorPipeline {
    pub validator: Box<dyn DocumentValidator>,
    pub loader: Box<dyn DocumentLoader>,
    pub chunker: Box<dyn Chunker>,
    pub hash_engine: Box<dyn HashEngine>,
    pub storage: Box<dyn KnowledgeStorage>,
}

impl GeneratorPipeline {
    pub fn new() -> Self {
        Self {
            validator: Box::new(crate::knowledge::generator::validator::GeneratorValidator::new()),
            loader: Box::new(FileLoader::new()),
            chunker: Box::new(FixedChunker::new(2048)),
            hash_engine: Box::new(Blake3HashEngine::new()),
            storage: Box::new(InMemoryKnowledgeStorage::new()),
        }
    }

    pub fn into_knowledge_pipeline(self) -> KnowledgePipeline {
        KnowledgePipeline::new(self.validator, self.loader, self.chunker, self.hash_engine, self.storage)
    }
}
