use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::Arc;

use std::thread::Thread as StdThread;
use std::thread::ThreadId;

use crossbeam::sync::Parker;

struct Pool {}

struct Thread {
    thread: Arc<StdThread>,
    parker: Parker,
    is_active: AtomicBool,
    core: AtomicUsize,
}

// 0. task -> PoolMgr
// 1. PoolMgr -> select thread by core
// 2. send task to this thread
// 3. thread execute task
//      if thread should wait:
//          thread told to PoolMgr that he is parking
//
//      some thread told PoolMgr that this thread is ready
//
//      PoolMgr look at current status and unpark this thread
//

impl Pool {
    pub fn spawn<R: Send>(task: Box<dyn FnOnce() -> R>) {
        todo!()
    }
}

fn yield_current_thread() {
    let id = std::thread::current().id();
    // Find warpped self by id
    // set status to unactive
    // parker.park()
    //
    // visit current core
    // bind to current core
}

fn resume(thread_id: ThreadId) {
    // tell PoolMgr that resume this thread
}
