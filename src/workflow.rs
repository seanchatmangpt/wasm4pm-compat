//! Parallel Workflow Typestate Model.
//!
//! Defined in [workflow.rs](file:///Users/sac/wasm4pm-compat/src/workflow.rs).
//!
//! This module provides a compile-time typestate workflow tracking mechanism for checking
//! process flow topology, transitions, and cancellation branches.
//!
//! ## Representation
//! All types in this module, including [`crate::workflow::BranchToken`], [`crate::workflow::ParallelWorkflow`], [`crate::workflow::JoinPoint`],
//! and the state markers, are zero-sized types (0 bytes). They use [`core::marker::PhantomData`] to carry compile-time
//! structural and generic type information without runtime overhead.
//!
//! ## Structure-only
//! This module does not execute code, spawn threads, or run tasks. It acts strictly as a static shape and
//! transition validator to verify that branches are correctly initialized, progressed, canceled, or joined.
//!
//! ## Graduation
//! While this module ensures correct compile-time workflow structures, it graduates to the dynamic execution,
//! event broker, and orchestrator subsystems of `wasm4pm`.

use std::marker::PhantomData;

// =============================================================================
// 1. Zero-Sized State Markers representing the lifecycle of workflow branches
// =============================================================================

/// The branch is initialized but has not started executing.
///
/// ### Representation
/// A zero-sized state marker [`Pending`](file:///Users/sac/wasm4pm-compat/src/workflow.rs) that occupies 0 bytes of memory.
///
/// ### Structure-only
/// Represents a static state indicating a branch is ready to start. It contains no runtime status,
/// data values, or execution logic.
///
/// ### Graduation
/// In the full `wasm4pm` execution engine, a pending task is placed in a scheduler queue for dynamic dispatch.
/// In this compat crate, it is modeled purely as a compile-time static type parameter.
pub struct Pending;

/// The branch is currently active and executing.
///
/// ### Representation
/// A zero-sized state marker [`Running`](file:///Users/sac/wasm4pm-compat/src/workflow.rs) that occupies 0 bytes of memory.
///
/// ### Structure-only
/// Represents a static state indicating a branch is currently running. It holds no association with threads,
/// tasks, futures, or async runtimes.
///
/// ### Graduation
/// The `wasm4pm` engine tracks running tasks dynamically via host-managed event states and logs.
/// Here, it is represented as a static compile-time type parameter.
pub struct Running;

/// The branch completed its task successfully.
///
/// ### Representation
/// A zero-sized state marker [`Completed`](file:///Users/sac/wasm4pm-compat/src/workflow.rs) that occupies 0 bytes of memory.
///
/// ### Structure-only
/// Represents a static state indicating successful execution completion of a branch. It does not store
/// task results, return values, or logs.
///
/// ### Graduation
/// In `wasm4pm`, branch completion generates trace events and triggers downstream control flows dynamically.
/// In this compat layer, it statically proves path completion for compile-time checking.
pub struct Completed;

/// The branch was canceled by a concurrent cancellation region.
///
/// ### Representation
/// A zero-sized state marker [`Canceled`](file:///Users/sac/wasm4pm-compat/src/workflow.rs) that occupies 0 bytes of memory.
///
/// ### Structure-only
/// Represents a static state indicating a branch was canceled. It carries no cancellation error context
/// or cancellation signals.
///
/// ### Graduation
/// In `wasm4pm`, cancellation propagates dynamically via signals to active tasks, aborting execution.
/// Here, cancellation is a static transition that consumes the token, preventing future completion.
pub struct Canceled;

/// A marker trait to constrain allowed typestate markers.
///
/// ### Representation
/// A marker trait [`BranchState`](file:///Users/sac/wasm4pm-compat/src/workflow.rs) with no runtime footprint, generic parameters, or data fields.
///
/// ### Structure-only
/// Used purely as a compile-time type boundary to restrict state parameters to [`Pending`], [`Running`],
/// [`Completed`], or [`Canceled`]. It provides no functional logic.
///
/// ### Graduation
/// During graduation to the `wasm4pm` execution engine, these compile-time constraints map to dynamic,
/// schema-validated state transition policies.
pub trait BranchState {}
impl BranchState for Pending {}
impl BranchState for Running {}
impl BranchState for Completed {}
impl BranchState for Canceled {}

// =============================================================================
// 2. Linear Branch Token
// =============================================================================

/// An ownership-bearing token representing a specific execution path.
/// Since `_task` and `_state` are `PhantomData`, this struct has a size of 0 bytes.
///
/// ### Representation
/// A zero-sized type [`BranchToken`](file:///Users/sac/wasm4pm-compat/src/workflow.rs) holding a `PhantomData<T>` and `PhantomData<S>`.
/// Its size in memory is 0 bytes.
///
/// ### Structure-only
/// Models compile-time ownership of an abstract execution path. It holds no runtime task references,
/// performs no execution, and does not schedule workloads.
///
/// ### Graduation
/// In `wasm4pm`, a token represents a dynamic work item tracked by the runtime database. In the compat library,
/// it enforces compile-time sequencing and typestate safety.
pub struct BranchToken<T, S: BranchState> {
    pub _task: PhantomData<T>,
    pub _state: PhantomData<S>,
}

impl<T> BranchToken<T, Pending> {
    /// Progresses the branch from Pending to Running.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use wasm4pm_compat::workflow::{BranchToken, Pending, Running};
    /// use std::marker::PhantomData;
    ///
    /// struct MyTask;
    /// let token: BranchToken<MyTask, Pending> = BranchToken {
    ///     _task: PhantomData,
    ///     _state: PhantomData,
    /// };
    /// let running_token: BranchToken<MyTask, Running> = token.start();
    /// ```
    #[inline(always)]
    pub fn start(self) -> BranchToken<T, Running> {
        BranchToken {
            _task: PhantomData,
            _state: PhantomData,
        }
    }
}

impl<T> BranchToken<T, Running> {
    /// Normal successful completion of the branch.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use wasm4pm_compat::workflow::{BranchToken, Running, Completed};
    /// use std::marker::PhantomData;
    ///
    /// struct MyTask;
    /// let token: BranchToken<MyTask, Running> = BranchToken {
    ///     _task: PhantomData,
    ///     _state: PhantomData,
    /// };
    /// let completed_token: BranchToken<MyTask, Completed> = token.complete();
    /// ```
    #[inline(always)]
    pub fn complete(self) -> BranchToken<T, Completed> {
        BranchToken {
            _task: PhantomData,
            _state: PhantomData,
        }
    }
}

// =============================================================================
// 3. Parallel Workflow Carrier (AND-Split State)
// =============================================================================

/// Tracks the status of two concurrent branches (A and B) at compile time.
///
/// ### Representation
/// Struct [`ParallelWorkflow`](file:///Users/sac/wasm4pm-compat/src/workflow.rs) containing `branch_a` and `branch_b` of type `BranchToken<A, SA>` and `BranchToken<B, SB>`
/// respectively. Since both tokens are zero-sized, `ParallelWorkflow` is also zero-sized (0 bytes).
///
/// ### Structure-only
/// Models an AND-split/concurrency pattern between two branches without using OS threads, async futures,
/// or mutexes.
///
/// ### Graduation
/// Graduates to `wasm4pm`'s parallel gateways (BPMN AND-split and AND-join, or Petri Net transitions)
/// which manage dynamic, multi-threaded or distributed scheduling.
pub struct ParallelWorkflow<A, B, SA: BranchState, SB: BranchState> {
    pub branch_a: BranchToken<A, SA>,
    pub branch_b: BranchToken<B, SB>,
}

impl<A, B> ParallelWorkflow<A, B, Pending, Pending> {
    /// Initializes a parallel workflow split (AND-Split).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use wasm4pm_compat::workflow::{ParallelWorkflow, Pending};
    ///
    /// struct TaskA;
    /// struct TaskB;
    /// let workflow: ParallelWorkflow<TaskA, TaskB, Pending, Pending> = ParallelWorkflow::split();
    /// ```
    #[inline(always)]
    pub fn split() -> Self {
        ParallelWorkflow {
            branch_a: BranchToken {
                _task: PhantomData,
                _state: PhantomData,
            },
            branch_b: BranchToken {
                _task: PhantomData,
                _state: PhantomData,
            },
        }
    }
}

// =============================================================================
// 4. State Transitions (Normal & Cancellation Paths)
// =============================================================================

impl<A, B, SB: BranchState> ParallelWorkflow<A, B, Running, SB> {
    /// Completes the task on Branch A.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use wasm4pm_compat::workflow::{ParallelWorkflow, Pending, Running, Completed};
    ///
    /// struct TaskA;
    /// struct TaskB;
    /// let workflow: ParallelWorkflow<TaskA, TaskB, Pending, Pending> = ParallelWorkflow::split();
    /// // Start both branches
    /// let workflow = ParallelWorkflow {
    ///     branch_a: workflow.branch_a.start(),
    ///     branch_b: workflow.branch_b.start(),
    /// };
    /// // Complete branch A
    /// let workflow: ParallelWorkflow<TaskA, TaskB, Completed, Running> = workflow.complete_a();
    /// ```
    #[inline(always)]
    pub fn complete_a(self) -> ParallelWorkflow<A, B, Completed, SB> {
        ParallelWorkflow {
            branch_a: self.branch_a.complete(),
            branch_b: self.branch_b,
        }
    }
}

impl<A, B, SA: BranchState> ParallelWorkflow<A, B, SA, Running> {
    /// Completes the task on Branch B.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use wasm4pm_compat::workflow::{ParallelWorkflow, Pending, Running, Completed};
    ///
    /// struct TaskA;
    /// struct TaskB;
    /// let workflow: ParallelWorkflow<TaskA, TaskB, Pending, Pending> = ParallelWorkflow::split();
    /// // Start both branches
    /// let workflow = ParallelWorkflow {
    ///     branch_a: workflow.branch_a.start(),
    ///     branch_b: workflow.branch_b.start(),
    /// };
    /// // Complete branch B
    /// let workflow: ParallelWorkflow<TaskA, TaskB, Running, Completed> = workflow.complete_b();
    /// ```
    #[inline(always)]
    pub fn complete_b(self) -> ParallelWorkflow<A, B, SA, Completed> {
        ParallelWorkflow {
            branch_a: self.branch_a,
            branch_b: self.branch_b.complete(),
        }
    }
}

impl<A, B> ParallelWorkflow<A, B, Running, Running> {
    /// Fires a cancellation event from Branch A that targets Branch B.
    /// This transition consumes the active `BranchToken<B, Running>` and returns
    /// a `BranchToken<B, Canceled>` token. Because the running token is consumed
    /// and cannot be cloned, Branch B can never be completed.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use wasm4pm_compat::workflow::{ParallelWorkflow, Pending, Running, Completed, Canceled};
    ///
    /// struct TaskA;
    /// struct TaskB;
    /// let workflow: ParallelWorkflow<TaskA, TaskB, Pending, Pending> = ParallelWorkflow::split();
    /// // Start both branches
    /// let workflow = ParallelWorkflow {
    ///     branch_a: workflow.branch_a.start(),
    ///     branch_b: workflow.branch_b.start(),
    /// };
    /// // Cancel Branch B from Branch A
    /// let workflow: ParallelWorkflow<TaskA, TaskB, Completed, Canceled> = workflow.cancel_b_from_a();
    /// ```
    #[inline(always)]
    pub fn cancel_b_from_a(self) -> ParallelWorkflow<A, B, Completed, Canceled> {
        ParallelWorkflow {
            branch_a: BranchToken {
                _task: PhantomData,
                _state: PhantomData,
            },
            branch_b: BranchToken {
                _task: PhantomData,
                _state: PhantomData,
            },
        }
    }
}

// =============================================================================
// 5. Final Synchronization Point (AND-Join)
// =============================================================================

/// Represents a fully joined or synchronized workflow.
///
/// ### Representation
/// Struct [`CompletedWorkflow`](file:///Users/sac/wasm4pm-compat/src/workflow.rs) containing a single private field: `_private: ()`. It is zero-sized and takes 0 bytes.
///
/// ### Structure-only
/// Serves as a terminal state indicating a workflow has successfully synchronized or terminated. It contains no
/// executable code, control flow logic, or data payloads.
///
/// ### Graduation
/// In the `wasm4pm` execution engine, a completed workflow corresponds to a closed process instance or a finished case.
/// Here, it is just a terminal type proving synchronization.
pub struct CompletedWorkflow {
    pub _private: (),
}

/// A synchronization point (AND-Join) namespace for combining concurrent branches.
///
/// ### Representation
/// A zero-sized namespace struct [`JoinPoint`](file:///Users/sac/wasm4pm-compat/src/workflow.rs) that takes 0-bytes of memory.
///
/// ### Structure-only
/// Provides static helper methods to join concurrent branches without runtime synchronization mechanisms
/// like locks, semaphores, or condition variables.
///
/// ### Graduation
/// In the `wasm4pm` engine, synchronization is managed by a database-backed coordinator or actor system
/// evaluating event streams. Here, it is purely a type check.
pub struct JoinPoint;

impl JoinPoint {
    /// Synchronizes the workflow when both branches complete successfully.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use wasm4pm_compat::workflow::{ParallelWorkflow, Pending, Completed, CompletedWorkflow, JoinPoint};
    ///
    /// struct TaskA;
    /// struct TaskB;
    /// let workflow: ParallelWorkflow<TaskA, TaskB, Pending, Pending> = ParallelWorkflow::split();
    /// // Start and complete both branches
    /// let workflow = ParallelWorkflow {
    ///     branch_a: workflow.branch_a.start().complete(),
    ///     branch_b: workflow.branch_b.start().complete(),
    /// };
    /// // Synchronize using JoinPoint
    /// let completed: CompletedWorkflow = JoinPoint::join_success(workflow);
    /// ```
    #[inline(always)]
    pub fn join_success<A, B>(
        _wf: ParallelWorkflow<A, B, Completed, Completed>,
    ) -> CompletedWorkflow {
        CompletedWorkflow { _private: () }
    }

    /// Synchronizes the workflow when Branch B was cancelled.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use wasm4pm_compat::workflow::{ParallelWorkflow, Pending, Completed, Canceled, CompletedWorkflow, JoinPoint};
    ///
    /// struct TaskA;
    /// struct TaskB;
    /// let workflow: ParallelWorkflow<TaskA, TaskB, Pending, Pending> = ParallelWorkflow::split();
    /// // Start both, then cancel branch B from branch A
    /// let workflow = ParallelWorkflow {
    ///     branch_a: workflow.branch_a.start(),
    ///     branch_b: workflow.branch_b.start(),
    /// };
    /// let workflow: ParallelWorkflow<TaskA, TaskB, Completed, Canceled> = workflow.cancel_b_from_a();
    /// // Synchronize using JoinPoint
    /// let completed: CompletedWorkflow = JoinPoint::join_canceled_b(workflow);
    /// ```
    #[inline(always)]
    pub fn join_canceled_b<A, B>(
        _wf: ParallelWorkflow<A, B, Completed, Canceled>,
    ) -> CompletedWorkflow {
        CompletedWorkflow { _private: () }
    }
}
