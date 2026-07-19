use crate::ecc::errors::EccResult;
use crate::ecc::executor::types::{ExecutorEccContext, ExecutorRequest};
use crate::ecc::pipeline::{Pipeline, PipelineContext};
use crate::ecc::traits::{
    ConfidenceScorer, Corrector, ErrorClassifier, Policy, Reporter, Validator,
};
use std::sync::Arc;

/// Executor ECC engine that runs the full executor ECC pipeline.
pub struct ExecutorEccEngine {
    _validator: Arc<dyn Validator<ExecutorRequest>>,
    _corrector: Arc<dyn Corrector<ExecutorRequest>>,
    _classifier: Arc<dyn ErrorClassifier<ExecutorRequest>>,
    _scorer: Arc<dyn ConfidenceScorer<ExecutorRequest>>,
    _reporter: Arc<dyn Reporter<ExecutorRequest>>,
    _policy: Arc<dyn Policy>,
    pipeline: Pipeline<ExecutorRequest>,
    _context: ExecutorEccContext,
}

impl ExecutorEccEngine {
    /// Create a new executor ECC engine with all pipeline components.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        validator: Arc<dyn Validator<ExecutorRequest>>,
        corrector: Arc<dyn Corrector<ExecutorRequest>>,
        classifier: Arc<dyn ErrorClassifier<ExecutorRequest>>,
        scorer: Arc<dyn ConfidenceScorer<ExecutorRequest>>,
        reporter: Arc<dyn Reporter<ExecutorRequest>>,
        policy: Arc<dyn Policy>,
        pipeline: Pipeline<ExecutorRequest>,
        context: ExecutorEccContext,
    ) -> Self {
        Self {
            _validator: validator,
            _corrector: corrector,
            _classifier: classifier,
            _scorer: scorer,
            _reporter: reporter,
            _policy: policy,
            pipeline,
            _context: context,
        }
    }

    /// Execute the full Executor ECC pipeline.
    pub fn execute(&self, request: ExecutorRequest) -> EccResult<crate::ecc::report::EccReport> {
        let mut context = PipelineContext::new(request);
        self.pipeline.run(&mut context)
    }
}
