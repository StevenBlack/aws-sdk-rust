// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteBackend`](crate::operation::delete_backend::builders::DeleteBackendFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`app_id(impl Into<String>)`](crate::operation::delete_backend::builders::DeleteBackendFluentBuilder::app_id) / [`set_app_id(Option<String>)`](crate::operation::delete_backend::builders::DeleteBackendFluentBuilder::set_app_id): <p>The app ID.</p>
    ///   - [`backend_environment_name(impl Into<String>)`](crate::operation::delete_backend::builders::DeleteBackendFluentBuilder::backend_environment_name) / [`set_backend_environment_name(Option<String>)`](crate::operation::delete_backend::builders::DeleteBackendFluentBuilder::set_backend_environment_name): <p>The name of the backend environment.</p>
    /// - On success, responds with [`DeleteBackendOutput`](crate::operation::delete_backend::DeleteBackendOutput) with field(s):
    ///   - [`app_id(Option<String>)`](crate::operation::delete_backend::DeleteBackendOutput::app_id): <p>The app ID.</p>
    ///   - [`backend_environment_name(Option<String>)`](crate::operation::delete_backend::DeleteBackendOutput::backend_environment_name): <p>The name of the backend environment.</p>
    ///   - [`error(Option<String>)`](crate::operation::delete_backend::DeleteBackendOutput::error): <p>If the request fails, this error is returned.</p>
    ///   - [`job_id(Option<String>)`](crate::operation::delete_backend::DeleteBackendOutput::job_id): <p>The ID for the job.</p>
    ///   - [`operation(Option<String>)`](crate::operation::delete_backend::DeleteBackendOutput::operation): <p>The name of the operation.</p>
    ///   - [`status(Option<String>)`](crate::operation::delete_backend::DeleteBackendOutput::status): <p>The current status of the request.</p>
    /// - On failure, responds with [`SdkError<DeleteBackendError>`](crate::operation::delete_backend::DeleteBackendError)
    pub fn delete_backend(&self) -> crate::operation::delete_backend::builders::DeleteBackendFluentBuilder {
        crate::operation::delete_backend::builders::DeleteBackendFluentBuilder::new(self.handle.clone())
    }
}
