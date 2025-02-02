// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`CreateLogStream`](crate::operation::create_log_stream::builders::CreateLogStreamFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`log_group_name(impl Into<String>)`](crate::operation::create_log_stream::builders::CreateLogStreamFluentBuilder::log_group_name) / [`set_log_group_name(Option<String>)`](crate::operation::create_log_stream::builders::CreateLogStreamFluentBuilder::set_log_group_name): <p>The name of the log group.</p>
    ///   - [`log_stream_name(impl Into<String>)`](crate::operation::create_log_stream::builders::CreateLogStreamFluentBuilder::log_stream_name) / [`set_log_stream_name(Option<String>)`](crate::operation::create_log_stream::builders::CreateLogStreamFluentBuilder::set_log_stream_name): <p>The name of the log stream.</p>
    /// - On success, responds with [`CreateLogStreamOutput`](crate::operation::create_log_stream::CreateLogStreamOutput)
    /// - On failure, responds with [`SdkError<CreateLogStreamError>`](crate::operation::create_log_stream::CreateLogStreamError)
    pub fn create_log_stream(&self) -> crate::operation::create_log_stream::builders::CreateLogStreamFluentBuilder {
        crate::operation::create_log_stream::builders::CreateLogStreamFluentBuilder::new(self.handle.clone())
    }
}
