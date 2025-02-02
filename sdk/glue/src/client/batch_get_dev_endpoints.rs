// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`BatchGetDevEndpoints`](crate::operation::batch_get_dev_endpoints::builders::BatchGetDevEndpointsFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`dev_endpoint_names(impl Into<String>)`](crate::operation::batch_get_dev_endpoints::builders::BatchGetDevEndpointsFluentBuilder::dev_endpoint_names) / [`set_dev_endpoint_names(Option<Vec<String>>)`](crate::operation::batch_get_dev_endpoints::builders::BatchGetDevEndpointsFluentBuilder::set_dev_endpoint_names): <p>The list of <code>DevEndpoint</code> names, which might be the names returned from the <code>ListDevEndpoint</code> operation.</p>
    /// - On success, responds with [`BatchGetDevEndpointsOutput`](crate::operation::batch_get_dev_endpoints::BatchGetDevEndpointsOutput) with field(s):
    ///   - [`dev_endpoints(Option<Vec<DevEndpoint>>)`](crate::operation::batch_get_dev_endpoints::BatchGetDevEndpointsOutput::dev_endpoints): <p>A list of <code>DevEndpoint</code> definitions.</p>
    ///   - [`dev_endpoints_not_found(Option<Vec<String>>)`](crate::operation::batch_get_dev_endpoints::BatchGetDevEndpointsOutput::dev_endpoints_not_found): <p>A list of <code>DevEndpoints</code> not found.</p>
    /// - On failure, responds with [`SdkError<BatchGetDevEndpointsError>`](crate::operation::batch_get_dev_endpoints::BatchGetDevEndpointsError)
    pub fn batch_get_dev_endpoints(&self) -> crate::operation::batch_get_dev_endpoints::builders::BatchGetDevEndpointsFluentBuilder {
        crate::operation::batch_get_dev_endpoints::builders::BatchGetDevEndpointsFluentBuilder::new(self.handle.clone())
    }
}
