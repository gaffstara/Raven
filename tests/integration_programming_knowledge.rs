use raven_agent::KnowledgeManager;
use std::path::Path;

#[test]
fn integration_programming_library_pipeline() -> Result<(), Box<dyn std::error::Error>> {
    let pipeline = raven_agent::knowledge::builder::KnowledgePipelineBuilder::new()
        .with_storage(Box::new(
            raven_agent::knowledge::storage::InMemoryKnowledgeStorage::new(),
        ))
        .build();

    // Process the programming knowledge directory
    let processed = pipeline
        .process_directory(Path::new("knowledge/programming"))
        .map_err(|e| Box::<dyn std::error::Error>::from(e.to_string()))?;

    // Ensure at least one document was processed
    assert!(
        !processed.is_empty(),
        "no documents were processed from knowledge/programming"
    );

    // Use KnowledgeManager to retrieve by a known token
    let manager =
        raven_agent::knowledge::manager::KnowledgeManagerImpl::new_with_default_engine(pipeline);
    let ctx = manager
        .retrieve("ownership", 5)
        .map_err(|e| Box::<dyn std::error::Error>::from(e.to_string()))?;

    assert!(ctx.document_count >= 1, "retrieval returned no documents");

    Ok(())
}
