# Agent Runtime Roadmap
**Project:** Raven AI Agent
**Module:** Agent Runtime
**Language:** Rust
**Status:** Design Specification (Single Source of Truth)

---

# Tujuan

Agent Runtime adalah pusat orkestrasi Raven.

Seluruh siklus hidup Agent dikendalikan oleh Agent Runtime.

Agent Runtime **bukan** LLM.

Agent Runtime **bukan** Planner.

Agent Runtime **bukan** Executor.

Agent Runtime adalah "otak pengatur" yang menghubungkan seluruh subsystem Raven menjadi satu kesatuan.

---

# Filosofi

Agent Runtime harus:

- deterministic
- modular
- thread-safe
- asynchronous
- event-driven
- production-ready
- extensible
- testable
- low-memory
- cocok berjalan di Termux
- independen terhadap model LLM

Qwen hanyalah salah satu komponen yang nantinya dipanggil oleh Runtime.

Runtime tidak boleh bergantung pada implementasi model tertentu.

---

# Larangan

Seluruh implementasi WAJIB memenuhi aturan berikut.

Tidak boleh menggunakan:

- mock
- dummy
- placeholder
- skeleton
- fake implementation
- todo!
- unimplemented!
- panic! pada normal flow
- hardcoded workflow
- bypass validation
- unsafe tanpa alasan teknis yang kuat
- temporary workaround

Seluruh kode harus merupakan implementasi nyata.

---

# Tujuan Arsitektur

Runtime bertanggung jawab terhadap seluruh lifecycle Agent.

Diagram konseptual:

User

↓

Agent Runtime

↓

Intent Analyzer

↓

Planner

↓

Memory ECC

↓

Knowledge Service

↓

Tool ECC

↓

Workflow ECC

↓

Executor ECC

↓

Executor Runtime

↓

Memory Update

↓

Reflection

↓

Response

↓

User

Runtime menjadi satu-satunya orchestrator.

Tidak boleh ada modul lain yang memanggil Planner atau Executor secara langsung tanpa melalui Runtime.

---

# Struktur Direktori

Disarankan:

src/agent/

```
runtime/
mod.rs
engine.rs
builder.rs
context.rs
session.rs
lifecycle.rs
orchestrator.rs
scheduler.rs
dispatcher.rs
retry.rs
recovery.rs
state.rs
events.rs
metrics.rs
report.rs
traits.rs
errors.rs
pipeline.rs
types.rs

hooks/
middleware/
tests/
```

Apabila file mulai membesar maka WAJIB dipecah menjadi submodule.

Tidak boleh ada file monster.

---

# Integrasi Wajib

Agent Runtime harus terintegrasi dengan:

- Planner Service
- Planner ECC

- Memory Service
- Memory ECC

- Workflow Service
- Workflow ECC

- Tool Service
- Tool ECC

- Executor Runtime
- Executor ECC

- Reflection Service

- Event Bus

- Logger

- Config

- Workspace

- Database

- Knowledge Service (nantinya)

- LLM Runtime (nantinya)

Seluruh integrasi menggunakan trait yang sudah tersedia.

Tidak boleh menduplikasi service.

---

# Lifecycle

Runtime harus mempunyai lifecycle yang jelas.

Minimal:

Idle

ReceivingInput

UnderstandingIntent

Planning

MemoryLookup

KnowledgeLookup

WorkflowCreation

WorkflowValidation

ExecutionValidation

Execution

Reflection

MemoryUpdate

Completed

Cancelled

Failed

Timeout

Lifecycle harus deterministic.

---

# Session Manager

Runtime harus mampu mengelola banyak sesi.

Session harus memiliki:

Session ID

Conversation ID

Created Time

Updated Time

Conversation State

Runtime Context

Memory Context

Workflow Context

Execution Context

---

# Context Builder

Runtime harus membangun context sebelum memanggil subsystem.

Context terdiri dari:

User Input

Conversation History

Relevant Memory

Knowledge Result

Planner State

Workflow State

Execution State

Runtime Metadata

Context harus immutable sebanyak mungkin.

---

# Orchestrator

Ini adalah inti Runtime.

Tugas:

menerima input

↓

membangun context

↓

memanggil Intent

↓

memanggil Planner

↓

mengambil Memory

↓

mengambil Knowledge

↓

membangun Workflow

↓

memvalidasi Workflow

↓

menjalankan Executor

↓

mengumpulkan hasil

↓

Reflection

↓

Update Memory

↓

Publish Event

↓

Return Response

Tidak boleh ada shortcut.

---

# Scheduler

Scheduler menentukan urutan pekerjaan.

Harus mendukung:

dependency graph

priority

retry

timeout

cancel

pause

resume (opsional)

---

# Dispatcher

Dispatcher memilih subsystem yang tepat.

Misalnya:

Planner

Memory

Knowledge

Tool

Executor

Reflection

LLM

Dispatcher tidak boleh menggunakan hardcoded if-else yang panjang.

Gunakan registry atau trait dispatch.

---

# Retry Manager

Runtime harus memiliki Retry Manager.

Retry harus configurable.

Retry tidak boleh mengulang seluruh pipeline apabila hanya satu subsystem yang gagal.

---

# Recovery Manager

Recovery bertanggung jawab terhadap:

rollback

cleanup

resource release

context recovery

workflow recovery

execution recovery

---

# Event System

Runtime wajib menggunakan Event Bus.

Minimal event:

RuntimeStarted

InputReceived

PlanningStarted

PlanningFinished

MemoryLookupStarted

MemoryLookupFinished

WorkflowStarted

WorkflowFinished

ExecutionStarted

ExecutionFinished

ReflectionStarted

ReflectionFinished

MemoryUpdated

RuntimeCompleted

RuntimeFailed

RuntimeCancelled

---

# Metrics

Runtime harus mencatat:

execution duration

planning duration

memory duration

workflow duration

executor duration

retry count

failure count

tool count

workflow count

memory hit

memory miss

knowledge hit

knowledge miss

---

# Reporting

Runtime menghasilkan RuntimeReport.

Minimal:

status

duration

execution summary

workflow summary

planner summary

memory summary

knowledge summary

retry summary

metrics

error

warning

confidence

policy

---

# State Machine

State Machine harus deterministic.

Minimal:

Created

Initialized

Running

Waiting

Retrying

Recovering

Completed

Cancelled

Failed

Timeout

---

# Error Handling

Gunakan Result.

Tidak boleh panic pada normal flow.

Setiap error harus mempunyai:

severity

source

stage

recoverability

timestamp

report

---

# Trait

Runtime harus berbasis trait.

Minimal:

AgentRuntime

RuntimeLifecycle

RuntimeScheduler

RuntimeDispatcher

RuntimeContextBuilder

RuntimeReporter

RuntimeMetrics

RuntimeRecovery

RuntimeRetry

---

# Dependency Injection

Gunakan Dependency Injection.

Jangan membuat singleton global.

Semua dependency diinjeksikan melalui Builder.

---

# Concurrency

Gunakan Rust async dengan benar.

Pastikan:

thread-safe

Send

Sync

deadlock-free

race-condition-free

---

# Testing

WAJIB membuat:

Unit Test

Integration Test

Runtime Test

Lifecycle Test

Scheduler Test

Dispatcher Test

Retry Test

Recovery Test

Context Test

Session Test

Event Test

Metrics Test

Report Test

State Machine Test

Error Test

Pipeline Test

Coverage harus tinggi.

---

# Dokumentasi

Seluruh public API harus memiliki dokumentasi Rust.

Gunakan:

///

Tidak boleh ada public API tanpa dokumentasi.

---

# Validasi Akhir

Implementasi belum dianggap selesai sebelum seluruh perintah berikut berhasil:

cargo fmt --all

cargo check

cargo clippy --workspace --all-targets --all-features -- -D warnings

cargo test

cargo doc --no-deps

---

# Kriteria Selesai

Tugas belum selesai apabila masih terdapat:

- error
- warning
- clippy lint
- unused import
- unused variable
- dead code
- duplicate code
- mock
- dummy
- placeholder
- skeleton
- todo!
- unimplemented!
- panic normal flow
- cargo check gagal
- cargo test gagal
- cargo doc gagal
- cargo clippy gagal

Semua hasil harus benar-benar hijau.

---

# Target Akhir

Setelah Agent Runtime selesai, Raven memiliki arsitektur berikut:

User

↓

Agent Runtime

↓

Intent Analyzer

↓

Planner ECC

↓

Memory ECC

↓

Knowledge Service

↓

Tool ECC

↓

Workflow ECC

↓

Executor ECC

↓

Executor Runtime

↓

Reflection

↓

Memory Update

↓

Response

↓

User

Pada tahap ini Raven telah memiliki **inti sistem AI Agent** yang lengkap. Integrasi Qwen2.5-Coder 1.5B pada tahap berikutnya cukup dilakukan melalui **LLM Runtime Adapter** tanpa mengubah Planner, Memory, Workflow, Executor, maupun Agent Runtime. Hal ini menjaga arsitektur tetap modular, mudah diuji, dan mudah dikembangkan di masa depan.