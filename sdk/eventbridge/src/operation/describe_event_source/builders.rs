// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::describe_event_source::_describe_event_source_output::DescribeEventSourceOutputBuilder;

pub use crate::operation::describe_event_source::_describe_event_source_input::DescribeEventSourceInputBuilder;

impl DescribeEventSourceInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::describe_event_source::DescribeEventSourceOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::describe_event_source::DescribeEventSourceError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.describe_event_source();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `DescribeEventSource`.
///
/// <p>This operation lists details about a partner event source that is shared with your account.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DescribeEventSourceFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::describe_event_source::builders::DescribeEventSourceInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::describe_event_source::DescribeEventSourceOutput,
        crate::operation::describe_event_source::DescribeEventSourceError,
    > for DescribeEventSourceFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::describe_event_source::DescribeEventSourceOutput,
            crate::operation::describe_event_source::DescribeEventSourceError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl DescribeEventSourceFluentBuilder {
    /// Creates a new `DescribeEventSource`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the DescribeEventSource as a reference.
    pub fn as_input(&self) -> &crate::operation::describe_event_source::builders::DescribeEventSourceInputBuilder {
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
        crate::operation::describe_event_source::DescribeEventSourceOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::describe_event_source::DescribeEventSourceError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self.inner.build().map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::describe_event_source::DescribeEventSource::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::describe_event_source::DescribeEventSource::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent.
    // TODO(enableNewSmithyRuntimeCleanup): Remove `async` and `Result` once we switch to orchestrator
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::orchestrator::CustomizableOperation<
            crate::operation::describe_event_source::DescribeEventSourceOutput,
            crate::operation::describe_event_source::DescribeEventSourceError,
            Self,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::describe_event_source::DescribeEventSourceError>,
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
    /// <p>The name of the partner event source to display the details of.</p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.name(input.into());
        self
    }
    /// <p>The name of the partner event source to display the details of.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_name(input);
        self
    }
    /// <p>The name of the partner event source to display the details of.</p>
    pub fn get_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_name()
    }
}
