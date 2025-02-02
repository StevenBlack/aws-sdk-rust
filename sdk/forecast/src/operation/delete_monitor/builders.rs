// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::delete_monitor::_delete_monitor_output::DeleteMonitorOutputBuilder;

pub use crate::operation::delete_monitor::_delete_monitor_input::DeleteMonitorInputBuilder;

impl DeleteMonitorInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::delete_monitor::DeleteMonitorOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::delete_monitor::DeleteMonitorError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.delete_monitor();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `DeleteMonitor`.
///
/// <p>Deletes a monitor resource. You can only delete a monitor resource with a status of <code>ACTIVE</code>, <code>ACTIVE_STOPPED</code>, <code>CREATE_FAILED</code>, or <code>CREATE_STOPPED</code>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DeleteMonitorFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::delete_monitor::builders::DeleteMonitorInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::delete_monitor::DeleteMonitorOutput,
        crate::operation::delete_monitor::DeleteMonitorError,
    > for DeleteMonitorFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::delete_monitor::DeleteMonitorOutput,
            crate::operation::delete_monitor::DeleteMonitorError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl DeleteMonitorFluentBuilder {
    /// Creates a new `DeleteMonitor`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the DeleteMonitor as a reference.
    pub fn as_input(&self) -> &crate::operation::delete_monitor::builders::DeleteMonitorInputBuilder {
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
        crate::operation::delete_monitor::DeleteMonitorOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::delete_monitor::DeleteMonitorError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self.inner.build().map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::delete_monitor::DeleteMonitor::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::delete_monitor::DeleteMonitor::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent.
    // TODO(enableNewSmithyRuntimeCleanup): Remove `async` and `Result` once we switch to orchestrator
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::orchestrator::CustomizableOperation<
            crate::operation::delete_monitor::DeleteMonitorOutput,
            crate::operation::delete_monitor::DeleteMonitorError,
            Self,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::delete_monitor::DeleteMonitorError>,
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
    /// <p>The Amazon Resource Name (ARN) of the monitor resource to delete.</p>
    pub fn monitor_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.monitor_arn(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the monitor resource to delete.</p>
    pub fn set_monitor_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_monitor_arn(input);
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the monitor resource to delete.</p>
    pub fn get_monitor_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_monitor_arn()
    }
}
