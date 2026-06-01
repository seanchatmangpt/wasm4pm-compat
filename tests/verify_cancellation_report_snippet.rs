use std::marker::PhantomData;

// =============================================================================
// 1. Zero-Sized State Markers representing the lifecycle of workflow branches
// =============================================================================

/// The branch is initialized but has not started executing.
pub struct Pending;
/// The branch is currently active and executing.
pub struct Running;
/// The branch completed its task successfully.
pub struct Completed;
/// The branch was canceled by a concurrent cancellation region.
pub struct Canceled;

/// A marker trait to constrain allowed typestate markers.
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
pub struct BranchToken<T, S: BranchState> {
    _task: PhantomData<T>,
    _state: PhantomData<S>,
}

impl<T> BranchToken<T, Pending> {
    /// Progresses the branch from Pending to Running.
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
pub struct ParallelWorkflow<A, B, SA: BranchState, SB: BranchState> {
    pub branch_a: BranchToken<A, SA>,
    pub branch_b: BranchToken<B, SB>,
}

impl<A, B> ParallelWorkflow<A, B, Pending, Pending> {
    /// Initializes a parallel workflow split (AND-Split).
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

pub struct CompletedWorkflow {
    _private: (),
}

pub struct JoinPoint;

impl JoinPoint {
    /// Synchronizes the workflow when both branches complete successfully.
    #[inline(always)]
    pub fn join_success<A, B>(
        _wf: ParallelWorkflow<A, B, Completed, Completed>,
    ) -> CompletedWorkflow {
        CompletedWorkflow { _private: () }
    }

    /// Synchronizes the workflow when Branch B was cancelled.
    #[inline(always)]
    pub fn join_canceled_b<A, B>(
        _wf: ParallelWorkflow<A, B, Completed, Canceled>,
    ) -> CompletedWorkflow {
        CompletedWorkflow { _private: () }
    }
}

// =============================================================================
// 6. Verification Tests
// =============================================================================

#[test]
fn test_workflow_success_path() {
    struct TaskA;
    struct TaskB;

    let wf = ParallelWorkflow::<TaskA, TaskB, Pending, Pending>::split();
    // Start both branches
    let wf = ParallelWorkflow {
        branch_a: wf.branch_a.start(),
        branch_b: wf.branch_b.start(),
    };

    // Complete both branches
    let wf = wf.complete_a();
    let wf = wf.complete_b();

    // Join
    let _completed = JoinPoint::join_success(wf);
}

#[test]
fn test_workflow_cancellation_path() {
    struct TaskA;
    struct TaskB;

    let wf = ParallelWorkflow::<TaskA, TaskB, Pending, Pending>::split();
    // Start both branches
    let wf = ParallelWorkflow {
        branch_a: wf.branch_a.start(),
        branch_b: wf.branch_b.start(),
    };

    // Branch A cancels Branch B
    let wf = wf.cancel_b_from_a();

    // Join
    let _completed = JoinPoint::join_canceled_b(wf);
}
