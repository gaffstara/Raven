Executor ECC Roadmap
Raven AI Agent
Versi: 1.0
Status: Design Specification
Tujuan
Executor ECC bertugas memvalidasi, memperbaiki, mengklasifikasi, dan memberi keputusan terhadap seluruh Execution Plan sebelum Executor menjalankan aksi apa pun.
Executor ECC adalah lapisan terakhir sebelum eksekusi nyata dilakukan.
Pipeline:
Planner
      │
            ▼
            Workflow
                  │
                        ▼
                        Tool
                              │
                                    ▼
                                    Executor ECC
                                          │
                                                ▼
                                                Executor Runtime
                                                      │
                                                            ▼
                                                            Shell / Python / Rust / HTTP / Memory
                                                            Executor tidak boleh mengeksekusi apa pun tanpa melalui Executor ECC.
                                                            Prinsip Desain
                                                            Tidak menggunakan mock.
                                                            Tidak menggunakan placeholder.
                                                            Tidak menggunakan skeleton implementation.
                                                            Tidak ada todo!.
                                                            Tidak ada unreachable!.
                                                            Tidak ada panic! untuk alur normal.
                                                            Zero unsafe code.
                                                            Deterministic.
                                                            Modular.
                                                            Thread-safe.
                                                            Production Ready.
                                                            Struktur Modul
                                                            src/ecc/executor/

                                                            mod.rs

                                                            engine.rs

                                                            builder.rs

                                                            pipeline.rs

                                                            context.rs

                                                            types.rs

                                                            traits.rs

                                                            validator.rs

                                                            corrector.rs

                                                            classifier.rs

                                                            policy.rs

                                                            confidence.rs

                                                            report.rs

                                                            errors.rs

                                                            rules/

                                                            stages/

                                                            tests/
                                                            Domain Model
                                                            ExecutionPlan
                                                            ExecutionStep
                                                            ExecutionAction
                                                            ExecutionTarget
                                                            ExecutionContext
                                                            ExecutionMetadata
                                                            ExecutionReport
                                                            ExecutionIssue
                                                            ExecutionSeverity
                                                            ExecutionDecision
                                                            ExecutionConfidence
                                                            ExecutionPolicy
                                                            ExecutionPipeline
                                                            Validation Rules
                                                            Minimal terdiri dari:
                                                            ExecutionNotEmptyRule
                                                            Execution minimal memiliki satu langkah.
                                                            StepOrderRule
                                                            Semua step harus memiliki urutan valid.
                                                            DuplicateExecutionRule
                                                            Tidak boleh ada step identik.
                                                            DependencyRule
                                                            Semua dependency harus tersedia.
                                                            CircularExecutionRule
                                                            Tidak boleh ada siklus.
                                                            ActionExistsRule
                                                            Action harus dikenal sistem.
                                                            ActionParameterRule
                                                            Semua parameter wajib tersedia.
                                                            PermissionRule
                                                            Action harus memiliki izin.
                                                            DangerousExecutionRule
                                                            Deteksi aksi berbahaya.
                                                            TimeoutRule
                                                            Timeout valid.
                                                            RetryRule
                                                            Retry valid.
                                                            ResourceLimitRule
                                                            CPU
                                                            Memory
                                                            Disk
                                                            Thread
                                                            harus sesuai batas.
                                                            SandboxCompatibilityRule
                                                            Execution kompatibel dengan Sandbox Runtime.
                                                            EnvironmentRule
                                                            Environment valid.
                                                            WorkspaceRule
                                                            Workspace valid.
                                                            OutputRule
                                                            Output target valid.
                                                            MemoryDependencyRule
                                                            Memory dependency valid.
                                                            WorkflowDependencyRule
                                                            Workflow dependency valid.
                                                            Correction Stage
                                                            Executor ECC boleh memperbaiki:
                                                            timeout default
                                                            retry default
                                                            dependency ordering
                                                            duplicate step
                                                            empty metadata
                                                            missing optional field
                                                            invalid priority
                                                            invalid execution id
                                                            invalid timestamps
                                                            Perbaikan harus non-destruktif.
                                                            Classification
                                                            Critical
                                                            High
                                                            Medium
                                                            Low
                                                            Info
                                                            Policy
                                                            Accept
                                                            Reject
                                                            Retry
                                                            Repair
                                                            Manual Review
                                                            Confidence Score
                                                            0.0
                                                            hingga
                                                            1.0
                                                            berdasarkan:
                                                            validasi
                                                            dependency
                                                            keamanan
                                                            konsistensi
                                                            resource
                                                            policy
                                                            Pipeline
                                                            Validation

                                                            ↓

                                                            Correction

                                                            ↓

                                                            Classification

                                                            ↓

                                                            Policy

                                                            ↓

                                                            Confidence

                                                            ↓

                                                            Reporting
                                                            Reporting
                                                            ExecutorReport harus berisi:
                                                            semua issue
                                                            semua fix
                                                            confidence
                                                            severity
                                                            execution summary
                                                            execution duration
                                                            policy result
                                                            Engine
                                                            ExecutorEngine menjadi orchestrator.
                                                            Engine bertugas:
                                                            menjalankan pipeline
                                                            mengumpulkan report
                                                            mengembalikan ExecutionDecision
                                                            Builder
                                                            Gunakan Builder Pattern.
                                                            ExecutorPipelineBuilder

                                                            ↓

                                                            Validation

                                                            ↓

                                                            Correction

                                                            ↓

                                                            Classification

                                                            ↓

                                                            Policy

                                                            ↓

                                                            Confidence

                                                            ↓

                                                            Reporter
                                                            Traits
                                                            Minimal:
                                                            Validator
                                                            Corrector
                                                            Classifier
                                                            Policy
                                                            ConfidenceScorer
                                                            Reporter
                                                            PipelineStage
                                                            ExecutorRule
                                                            Integrasi
                                                            Executor ECC harus terintegrasi dengan:
                                                            Planner ECC
                                                            Workflow ECC
                                                            Tool ECC
                                                            Memory ECC
                                                            Unit Test
                                                            Semua Rule memiliki unit test.
                                                            Target coverage:
                                                            100%
                                                            Integration Test
                                                            Pipeline penuh.
                                                            Valid execution.
                                                            Invalid execution.
                                                            Dangerous execution.
                                                            Missing dependency.
                                                            Circular dependency.
                                                            Retry flow.
                                                            Repair flow.
                                                            Reject flow.
                                                            Dokumentasi
                                                            cargo doc harus berhasil.
                                                            Semua public API memiliki RustDoc.
                                                            Target Akhir
                                                            Executor ECC harus:
                                                            Modular.
                                                            Deterministic.
                                                            Bebas warning Clippy.
                                                            Lulus cargo fmt.
                                                            Lulus cargo check.
                                                            Lulus seluruh cargo test.
                                                            Lulus cargo clippy --workspace --all-targets --all-features -- -D warnings.
                                                            Lulus cargo doc --no-deps.
                                                            Tidak mengubah perilaku modul ECC lain.
                                                            Menjadi lapisan validasi terakhir sebelum Executor Runtime.