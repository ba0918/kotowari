use crate::{Error, ErrorKind};
use std::{num::NonZeroUsize, sync::Arc};

#[derive(Debug, Clone, Copy)]
pub struct AsyncOptions {
    pub max_concurrency: NonZeroUsize,
}
impl Default for AsyncOptions {
    fn default() -> Self {
        Self {
            max_concurrency: NonZeroUsize::MIN,
        }
    }
}

#[derive(Clone)]
struct Blocking {
    slots: Arc<tokio::sync::Semaphore>,
}
impl Blocking {
    fn new(options: AsyncOptions) -> Self {
        Self {
            slots: Arc::new(tokio::sync::Semaphore::new(options.max_concurrency.get())),
        }
    }
    async fn run<T: Send + 'static>(
        &self,
        work: impl FnOnce() -> Result<T, Error> + Send + 'static,
    ) -> Result<T, Error> {
        let runtime = tokio::runtime::Handle::try_current().map_err(|error| Error {
            kind: ErrorKind::RuntimeUnavailable,
            detail: error.to_string(),
        })?;
        let permit = self
            .slots
            .clone()
            .acquire_owned()
            .await
            .map_err(|error| Error {
                kind: ErrorKind::TaskFailure,
                detail: error.to_string(),
            })?;
        runtime
            .spawn_blocking(move || {
                // A detached worker must still own its slot until the synchronous operation finishes.
                let _permit = permit;
                work()
            })
            .await
            .map_err(|error| Error {
                kind: ErrorKind::TaskFailure,
                detail: error.to_string(),
            })?
    }
}

#[derive(Clone)]
pub struct AsyncProject {
    project: crate::Project,
    blocking: Blocking,
}
impl AsyncProject {
    pub fn new(options: crate::ProjectOptions, asynchronous: AsyncOptions) -> Result<Self, Error> {
        Ok(Self {
            project: crate::Project::new(options)?,
            blocking: Blocking::new(asynchronous),
        })
    }
    pub async fn read(&self) -> Result<crate::ReadModel, Error> {
        let project = self.project.clone();
        self.blocking.run(move || project.read()).await
    }
    pub async fn inspect(&self) -> Result<crate::Inspection, Error> {
        let project = self.project.clone();
        self.blocking.run(move || project.inspect()).await
    }
    pub async fn check(&self) -> Result<crate::CheckReport, Error> {
        let project = self.project.clone();
        self.blocking.run(move || project.check()).await
    }
    pub async fn list(&self) -> Result<crate::ReadList, Error> {
        let project = self.project.clone();
        self.blocking.run(move || project.list()).await
    }
    pub async fn query(&self, id: &str) -> Result<crate::QueryReport, Error> {
        let project = self.project.clone();
        let id = id.to_owned();
        self.blocking.run(move || project.query(&id)).await
    }
    pub async fn status(&self) -> Result<crate::StatusReport, Error> {
        let project = self.project.clone();
        self.blocking.run(move || project.status()).await
    }
    pub async fn plan(&self, path: &std::path::Path) -> Result<crate::PlanReport, Error> {
        let project = self.project.clone();
        let path = path.to_path_buf();
        self.blocking.run(move || project.plan(&path)).await
    }
    pub async fn mutants(
        &self,
        options: &crate::MutantsOptions,
    ) -> Result<crate::MutantsReport, Error> {
        let project = self.project.clone();
        let options = options.clone();
        self.blocking.run(move || project.mutants(&options)).await
    }
    pub async fn overview_prepare(&self) -> Result<crate::OverviewPrepared, Error> {
        let project = self.project.clone();
        self.blocking.run(move || project.overview_prepare()).await
    }
    pub async fn overview_build(&self) -> Result<crate::OverviewBuild, Error> {
        let project = self.project.clone();
        self.blocking.run(move || project.overview_build()).await
    }
}
#[cfg(test)]
#[path = "async_tests.rs"]
mod tests;
