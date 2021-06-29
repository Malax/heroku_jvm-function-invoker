use libcnb::{Error, ErrorHandler};

pub struct HerokuBuildpackErrorHandler<E> {
    handler: Box<dyn Fn(E)>,
}

impl<E: std::error::Error> HerokuBuildpackErrorHandler<E> {
    pub fn new(handler: Box<dyn Fn(E)>) -> Self {
        HerokuBuildpackErrorHandler { handler }
    }
}

impl<E: std::error::Error> ErrorHandler<E> for HerokuBuildpackErrorHandler<E> {
    fn handle_error(&self, error: Error<E>) -> i32 {
        match error {
            Error::BuildpackError(buildpack_error) => (self.handler)(buildpack_error),
            Error::LayerLifecycleError(_) => {}
            Error::ProcessTypeError(_) => {}
            Error::CannotDetermineAppDirectory(_) => {}
            Error::CannotDetermineBuildpackDirectory(_) => {}
            Error::CannotDetermineStackId(_) => {}
            Error::CannotCreatePlatformFromPath(_) => {}
            Error::CannotReadBuildpackPlan(_) => {}
            Error::CannotReadBuildpackDescriptor(_) => {}
            Error::CannotWriteBuildPlan(_) => {}
        }

        100
    }
}
