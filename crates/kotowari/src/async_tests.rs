use super::*;
use std::{
    future::Future,
    sync::mpsc,
    task::{Context, Poll, Waker},
};

fn poll<F: Future>(future: std::pin::Pin<&mut F>) -> Poll<F::Output> {
    future.poll(&mut Context::from_waker(Waker::noop()))
}

// @kotowari[REQ-core-319, REQ-core-320, REQ-schema-075, EX-core-493, EX-core-494, EX-schema-092]
#[test]
fn clones_hold_started_permits_and_cancelled_waiters_are_never_submitted() {
    assert_eq!(AsyncOptions::default().max_concurrency.get(), 1);
    let runtime = tokio::runtime::Builder::new_current_thread()
        .build()
        .unwrap();
    let scheduler = Blocking::new(AsyncOptions::default());
    let clone = scheduler.clone();
    let (entered_tx, entered_rx) = mpsc::channel();
    let (release_tx, release_rx) = mpsc::channel();
    let (ran_tx, ran_rx) = mpsc::channel();
    runtime.block_on(async {
        let mut first = Box::pin(scheduler.run(move || {
            entered_tx.send(()).unwrap();
            release_rx.recv().unwrap();
            Ok(())
        }));
        assert!(poll(first.as_mut()).is_pending());
        entered_rx.recv().unwrap();
        let ran = ran_tx.clone();
        let mut cancelled = Box::pin(clone.run(move || {
            ran.send("cancelled").unwrap();
            Ok(())
        }));
        assert!(poll(cancelled.as_mut()).is_pending());
        drop(cancelled);
        drop(first);
        let mut live = Box::pin(clone.run(move || {
            ran_tx.send("live").unwrap();
            Ok(())
        }));
        assert!(poll(live.as_mut()).is_pending());
        assert!(ran_rx.try_recv().is_err());
        release_tx.send(()).unwrap();
        live.await.unwrap();
        assert_eq!(ran_rx.recv().unwrap(), "live");
        assert!(ran_rx.try_recv().is_err());
    });
}

// @kotowari[REQ-core-319, REQ-schema-075, EX-core-493, EX-schema-092]
#[test]
fn configured_limits_allow_two_workers_and_independent_objects_do_not_share_slots() {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .build()
        .unwrap();
    let scheduler = Blocking::new(AsyncOptions {
        max_concurrency: std::num::NonZeroUsize::new(2).unwrap(),
    });
    let other = Blocking::new(AsyncOptions::default());
    let (entered_tx, entered_rx) = mpsc::channel();
    let (release_one, receive_one) = mpsc::channel();
    let (release_two, receive_two) = mpsc::channel();
    runtime.block_on(async {
        let entered_two = entered_tx.clone();
        let mut first = Box::pin(scheduler.run(move || {
            entered_tx.send(1).unwrap();
            receive_one.recv().unwrap();
            Ok(())
        }));
        let mut second = Box::pin(scheduler.run(move || {
            entered_two.send(2).unwrap();
            receive_two.recv().unwrap();
            Ok(())
        }));
        assert!(poll(first.as_mut()).is_pending());
        assert!(poll(second.as_mut()).is_pending());
        let mut entered = vec![entered_rx.recv().unwrap(), entered_rx.recv().unwrap()];
        entered.sort();
        assert_eq!(entered, vec![1, 2]);
        assert_eq!(other.run(|| Ok(3)).await.unwrap(), 3);
        let mut third = Box::pin(scheduler.run(|| Ok(4)));
        assert!(poll(third.as_mut()).is_pending());
        release_one.send(()).unwrap();
        release_two.send(()).unwrap();
        first.await.unwrap();
        second.await.unwrap();
        assert_eq!(third.await.unwrap(), 4);
    });
}

// @kotowari[REQ-core-321, REQ-schema-075, EX-core-492, EX-schema-092]
#[test]
fn runtime_shutdown_of_queued_blocking_work_is_a_typed_task_failure() {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .max_blocking_threads(1)
        .build()
        .unwrap();
    let (entered_tx, entered_rx) = mpsc::channel();
    let (release_tx, release_rx) = mpsc::channel();
    let (done_tx, done_rx) = mpsc::channel();
    let scheduler = Blocking::new(AsyncOptions::default());
    let mut started = Box::pin(scheduler.run(move || {
        entered_tx.send(()).unwrap();
        release_rx.recv().unwrap();
        done_tx.send(()).unwrap();
        Ok(())
    }));
    {
        let _enter = runtime.enter();
        assert!(poll(started.as_mut()).is_pending());
    }
    entered_rx.recv().unwrap();
    let mut queued = Box::pin(scheduler.run(|| Ok(())));
    {
        let _enter = runtime.enter();
        assert!(poll(queued.as_mut()).is_pending());
    }
    runtime.shutdown_background();
    release_tx.send(()).unwrap();
    done_rx.recv().unwrap();
    let next = tokio::runtime::Builder::new_current_thread()
        .build()
        .unwrap();
    next.block_on(started).unwrap();
    assert_eq!(
        next.block_on(queued).unwrap_err().kind(),
        ErrorKind::TaskFailure
    );
}
