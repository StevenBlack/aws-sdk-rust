// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::delete_instance_event_window::_delete_instance_event_window_output::DeleteInstanceEventWindowOutputBuilder;

pub use crate::operation::delete_instance_event_window::_delete_instance_event_window_input::DeleteInstanceEventWindowInputBuilder;

impl DeleteInstanceEventWindowInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::delete_instance_event_window::DeleteInstanceEventWindowOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::delete_instance_event_window::DeleteInstanceEventWindowError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.delete_instance_event_window();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `DeleteInstanceEventWindow`.
///
/// <p>Deletes the specified event window.</p>
/// <p>For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/event-windows.html">Define event windows for scheduled events</a> in the <i>Amazon EC2 User Guide</i>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DeleteInstanceEventWindowFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::delete_instance_event_window::builders::DeleteInstanceEventWindowInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::delete_instance_event_window::DeleteInstanceEventWindowOutput,
        crate::operation::delete_instance_event_window::DeleteInstanceEventWindowError,
    > for DeleteInstanceEventWindowFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::delete_instance_event_window::DeleteInstanceEventWindowOutput,
            crate::operation::delete_instance_event_window::DeleteInstanceEventWindowError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl DeleteInstanceEventWindowFluentBuilder {
    /// Creates a new `DeleteInstanceEventWindow`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the DeleteInstanceEventWindow as a reference.
    pub fn as_input(&self) -> &crate::operation::delete_instance_event_window::builders::DeleteInstanceEventWindowInputBuilder {
        &self.inner
    }
    /// Sends the request and returns the response.
    ///
    /// If an error occurs, an `SdkError` will be returned with additional details that
    /// can be matched against.
    ///
    /// By default, any retryable failures will be retried twice. Retry behavior
    /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
    /// set when configuring the client.
    pub async fn send(
        self,
    ) -> ::std::result::Result<
        crate::operation::delete_instance_event_window::DeleteInstanceEventWindowOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::delete_instance_event_window::DeleteInstanceEventWindowError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self.inner.build().map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::delete_instance_event_window::DeleteInstanceEventWindow::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::delete_instance_event_window::DeleteInstanceEventWindow::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent.
    // TODO(enableNewSmithyRuntimeCleanup): Remove `async` and `Result` once we switch to orchestrator
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::orchestrator::CustomizableOperation<
            crate::operation::delete_instance_event_window::DeleteInstanceEventWindowOutput,
            crate::operation::delete_instance_event_window::DeleteInstanceEventWindowError,
            Self,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::delete_instance_event_window::DeleteInstanceEventWindowError>,
    > {
        ::std::result::Result::Ok(crate::client::customize::orchestrator::CustomizableOperation::new(self))
    }
    pub(crate) fn config_override(mut self, config_override: impl Into<crate::config::Builder>) -> Self {
        self.set_config_override(Some(config_override.into()));
        self
    }

    pub(crate) fn set_config_override(&mut self, config_override: Option<crate::config::Builder>) -> &mut Self {
        self.config_override = config_override;
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(mut self, input: bool) -> Self {
        self.inner = self.inner.dry_run(input);
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn set_dry_run(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_dry_run(input);
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn get_dry_run(&self) -> &::std::option::Option<bool> {
        self.inner.get_dry_run()
    }
    /// <p>Specify <code>true</code> to force delete the event window. Use the force delete parameter if the event window is currently associated with targets.</p>
    pub fn force_delete(mut self, input: bool) -> Self {
        self.inner = self.inner.force_delete(input);
        self
    }
    /// <p>Specify <code>true</code> to force delete the event window. Use the force delete parameter if the event window is currently associated with targets.</p>
    pub fn set_force_delete(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_force_delete(input);
        self
    }
    /// <p>Specify <code>true</code> to force delete the event window. Use the force delete parameter if the event window is currently associated with targets.</p>
    pub fn get_force_delete(&self) -> &::std::option::Option<bool> {
        self.inner.get_force_delete()
    }
    /// <p>The ID of the event window.</p>
    pub fn instance_event_window_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.instance_event_window_id(input.into());
        self
    }
    /// <p>The ID of the event window.</p>
    pub fn set_instance_event_window_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_instance_event_window_id(input);
        self
    }
    /// <p>The ID of the event window.</p>
    pub fn get_instance_event_window_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_instance_event_window_id()
    }
}
