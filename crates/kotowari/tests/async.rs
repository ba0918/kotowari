#![cfg(feature = "tokio")]
use kotowari::{
    AsyncOptions, AsyncProject, ChangesOptions, ErrorKind, MutantsOptions, Phase, Project,
    ProjectOptions, Target, Tool,
};
use std::{
    future::Future,
    path::Path,
    sync::Arc,
    task::{Context, Poll, Wake, Waker},
};

struct Noop;
impl Wake for Noop {
    fn wake(self: Arc<Self>) {}
}
fn send<T: Send>(value: T) -> T {
    value
}

// @kotowari[REQ-core-318, REQ-core-321, EX-core-492, EX-core-495]
#[test]
fn every_operation_is_send_and_reuses_sync_results_and_failure_kinds() {
    let dir = tempfile::tempdir().unwrap();
    for path in [
        ".kotowari",
        "docs/ir",
        "docs/decision/records",
        "docs/decision/adr",
    ] {
        std::fs::create_dir_all(dir.path().join(path)).unwrap();
    }
    std::fs::write(
        dir.path().join(".kotowari/config.yaml"),
        "tests:\n  files: []\n",
    )
    .unwrap();
    std::fs::write(dir.path().join("docs/ir/topic.md"), "# Topic\n").unwrap();
    std::fs::write(dir.path().join("plan.md"), "# Plan\n").unwrap();
    std::fs::write(dir.path().join("outcomes.json"), "{\"outcomes\": []}").unwrap();
    let options = send(ProjectOptions::new(dir.path()));
    let sync = Project::new(options.clone()).unwrap();
    let adapter = AsyncProject::new(options, AsyncOptions::default()).unwrap();
    let mut future = Box::pin(send(adapter.check()));
    let waker = Waker::from(Arc::new(Noop));
    let Poll::Ready(Err(error)) = future.as_mut().poll(&mut Context::from_waker(&waker)) else {
        panic!("expected runtime failure")
    };
    assert_eq!(error.kind(), ErrorKind::RuntimeUnavailable);
    let runtime = tokio::runtime::Runtime::new().unwrap();
    runtime.block_on(async {
        assert_eq!(
            send(adapter.check()).await.unwrap().counts(),
            sync.check().unwrap().counts()
        );
        assert_eq!(
            send(adapter.status()).await.unwrap().complete(),
            sync.status().unwrap().complete()
        );
        assert_eq!(
            send(adapter.list()).await.unwrap().items().len(),
            sync.list().unwrap().items().len()
        );
        assert_eq!(
            send(adapter.query("REQ-999")).await.err().unwrap().kind(),
            sync.query("REQ-999").err().unwrap().kind()
        );
        let read = send(send(adapter.read()).await.unwrap());
        let inspection = send(send(adapter.inspect()).await.unwrap());
        assert_eq!(
            read.list().items().len(),
            inspection.read().list().items().len()
        );
        assert_eq!(
            send(adapter.plan(Path::new("plan.md")))
                .await
                .unwrap()
                .findings()
                .len(),
            sync.plan(Path::new("plan.md")).unwrap().findings().len()
        );
        let options = send(MutantsOptions {
            tool: Tool::CargoMutants,
            results: "outcomes.json".into(),
        });
        assert_eq!(
            send(send(adapter.mutants(&options)).await.unwrap())
                .findings()
                .len(),
            sync.mutants(&options).unwrap().findings().len()
        );
        let options = send(ChangesOptions {
            base: "HEAD".into(),
            target: Target::Index,
            phase: Phase::Implementation,
        });
        assert_eq!(
            send(adapter.changes(&options)).await.err().unwrap().kind(),
            sync.changes(&options).err().unwrap().kind()
        );
        assert_eq!(
            send(adapter.plan(Path::new("missing.md")))
                .await
                .err()
                .unwrap()
                .kind(),
            sync.plan(Path::new("missing.md")).err().unwrap().kind()
        );
    });
}

// @kotowari[REQ-core-310, TBL-core-041, REQ-core-318]
#[test]
fn overview_prepare_and_build_return_what_the_sync_operations_return() {
    let make = || {
        let dir = tempfile::tempdir().unwrap();
        for path in [
            ".kotowari/overview",
            "docs/ir",
            "docs/decision/records",
            "docs/decision/adr",
        ] {
            std::fs::create_dir_all(dir.path().join(path)).unwrap();
        }
        std::fs::write(
            dir.path().join(".kotowari/config.yaml"),
            "tests:\n  files: []\noverview:\n  files: ['.kotowari/overview/*.md']\n  toc: .kotowari/toc.yaml\n",
        )
        .unwrap();
        std::fs::write(dir.path().join("docs/ir/topic.md"), "# Topic\n\nScope.\n").unwrap();
        std::fs::write(
            dir.path().join(".kotowari/overview/a.md"),
            "---\nir:\n  - docs/ir/topic.md\n---\n\n# a\n\n```view lead\nconclusion: c\n```\n",
        )
        .unwrap();
        dir
    };
    let (sync_dir, async_dir) = (make(), make());
    let sync = Project::new(ProjectOptions::new(sync_dir.path())).unwrap();
    let adapter = AsyncProject::new(
        ProjectOptions::new(async_dir.path()),
        AsyncOptions::default(),
    )
    .unwrap();
    let runtime = tokio::runtime::Runtime::new().unwrap();
    runtime.block_on(async {
        let prepared = send(adapter.overview_prepare()).await.unwrap();
        assert_eq!(prepared.pages(), sync.overview_prepare().unwrap().pages());
        assert!(!async_dir.path().join(".kotowari/cache").exists());
        let built = send(adapter.overview_build()).await.unwrap();
        assert_eq!(built, sync.overview_build().unwrap());
        assert_eq!(
            send(adapter.overview_build()).await.unwrap(),
            sync.overview_build().unwrap()
        );
        std::fs::remove_file(sync_dir.path().join(".kotowari/overview/a.md")).unwrap();
        std::fs::remove_file(async_dir.path().join(".kotowari/overview/a.md")).unwrap();
        assert_eq!(
            send(adapter.overview_build()).await.unwrap(),
            sync.overview_build().unwrap()
        );
    });
}
