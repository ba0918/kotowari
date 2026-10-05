#![cfg(feature = "tokio")]
use kotowari_markdown_schema::ValidationOptions;
use kotowari_markdown_schema_io::{
    AsyncOptions, AsyncSchemaLoader, ErrorKind, LoaderOptions, SchemaLoader,
};
use std::{
    future::Future,
    path::Path,
    task::{Context, Poll, Waker},
};
fn send<T: Send>(value: T) -> T {
    value
}

// @kotowari[REQ-schema-075, REQ-core-318, REQ-core-321, EX-schema-092, EX-core-492, EX-core-495]
#[test]
fn loader_futures_and_results_are_send_and_match_sync_operations() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(
        dir.path().join("schema.yaml"),
        "name: Memory\ndocument:\n  title:\n    pattern: '^Document$'\n    extract: title\n",
    )
    .unwrap();
    std::fs::write(
        dir.path().join("document.md"),
        "---\n$schema: schema.yaml\n---\n# Document\n",
    )
    .unwrap();
    let options = send(LoaderOptions::new(dir.path().into()));
    let sync = SchemaLoader::new(options.clone()).unwrap();
    let adapter = AsyncSchemaLoader::new(options, AsyncOptions::default()).unwrap();
    let path = Path::new("document.md");
    let validation = ValidationOptions::default();
    let mut future = Box::pin(send(adapter.load(path)));
    let waker = Waker::noop();
    let Poll::Ready(Err(error)) = future.as_mut().poll(&mut Context::from_waker(waker)) else {
        panic!("expected runtime failure")
    };
    assert_eq!(error.kind(), ErrorKind::RuntimeUnavailable);
    tokio::runtime::Runtime::new().unwrap().block_on(async {
        let loaded = send(send(adapter.load(path)).await.unwrap());
        assert_eq!(
            loaded.schema().name(),
            sync.load(path).unwrap().schema().name()
        );
        assert_eq!(
            send(send(adapter.check(path, validation)).await.unwrap())
                .files()
                .len(),
            sync.check(path, validation).unwrap().files().len()
        );
        assert_eq!(
            send(
                send(adapter.extract_partial(path, validation))
                    .await
                    .unwrap()
            )
            .values(),
            sync.extract_partial(path, validation).unwrap().values()
        );
        assert_eq!(
            send(adapter.extract_validated(path, validation))
                .await
                .unwrap()
                .is_ok(),
            sync.extract_validated(path, validation).unwrap().is_ok()
        );
        assert_eq!(
            send(adapter.load(Path::new("missing.md")))
                .await
                .err()
                .unwrap()
                .kind(),
            sync.load(Path::new("missing.md")).err().unwrap().kind()
        );
    });
}
