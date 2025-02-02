// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::list_queued_messages::_list_queued_messages_output::ListQueuedMessagesOutputBuilder;

pub use crate::operation::list_queued_messages::_list_queued_messages_input::ListQueuedMessagesInputBuilder;

impl ListQueuedMessagesInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::list_queued_messages::ListQueuedMessagesOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::list_queued_messages::ListQueuedMessagesError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.list_queued_messages();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `ListQueuedMessages`.
///
/// <p>List queued messages in the downlink queue.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ListQueuedMessagesFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::list_queued_messages::builders::ListQueuedMessagesInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::list_queued_messages::ListQueuedMessagesOutput,
        crate::operation::list_queued_messages::ListQueuedMessagesError,
    > for ListQueuedMessagesFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::list_queued_messages::ListQueuedMessagesOutput,
            crate::operation::list_queued_messages::ListQueuedMessagesError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl ListQueuedMessagesFluentBuilder {
    /// Creates a new `ListQueuedMessages`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the ListQueuedMessages as a reference.
    pub fn as_input(&self) -> &crate::operation::list_queued_messages::builders::ListQueuedMessagesInputBuilder {
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
        crate::operation::list_queued_messages::ListQueuedMessagesOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::list_queued_messages::ListQueuedMessagesError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self.inner.build().map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::list_queued_messages::ListQueuedMessages::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::list_queued_messages::ListQueuedMessages::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent.
    // TODO(enableNewSmithyRuntimeCleanup): Remove `async` and `Result` once we switch to orchestrator
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::orchestrator::CustomizableOperation<
            crate::operation::list_queued_messages::ListQueuedMessagesOutput,
            crate::operation::list_queued_messages::ListQueuedMessagesError,
            Self,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::list_queued_messages::ListQueuedMessagesError>,
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
    /// Create a paginator for this request
    ///
    /// Paginators are used by calling [`send().await`](crate::operation::list_queued_messages::paginator::ListQueuedMessagesPaginator::send) which returns a `Stream`.
    pub fn into_paginator(self) -> crate::operation::list_queued_messages::paginator::ListQueuedMessagesPaginator {
        crate::operation::list_queued_messages::paginator::ListQueuedMessagesPaginator::new(self.handle, self.inner)
    }
    /// <p>The ID of a given wireless device which the downlink message packets are being sent.</p>
    pub fn id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.id(input.into());
        self
    }
    /// <p>The ID of a given wireless device which the downlink message packets are being sent.</p>
    pub fn set_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_id(input);
        self
    }
    /// <p>The ID of a given wireless device which the downlink message packets are being sent.</p>
    pub fn get_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_id()
    }
    /// <p>To retrieve the next set of results, the <code>nextToken</code> value from a previous response; otherwise <b>null</b> to receive the first set of results.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>To retrieve the next set of results, the <code>nextToken</code> value from a previous response; otherwise <b>null</b> to receive the first set of results.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
    /// <p>To retrieve the next set of results, the <code>nextToken</code> value from a previous response; otherwise <b>null</b> to receive the first set of results.</p>
    pub fn get_next_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_next_token()
    }
    /// <p>The maximum number of results to return in this operation.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_results(input);
        self
    }
    /// <p>The maximum number of results to return in this operation.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
    /// <p>The maximum number of results to return in this operation.</p>
    pub fn get_max_results(&self) -> &::std::option::Option<i32> {
        self.inner.get_max_results()
    }
    /// <p>The wireless device type, whic can be either Sidewalk or LoRaWAN.</p>
    pub fn wireless_device_type(mut self, input: crate::types::WirelessDeviceType) -> Self {
        self.inner = self.inner.wireless_device_type(input);
        self
    }
    /// <p>The wireless device type, whic can be either Sidewalk or LoRaWAN.</p>
    pub fn set_wireless_device_type(mut self, input: ::std::option::Option<crate::types::WirelessDeviceType>) -> Self {
        self.inner = self.inner.set_wireless_device_type(input);
        self
    }
    /// <p>The wireless device type, whic can be either Sidewalk or LoRaWAN.</p>
    pub fn get_wireless_device_type(&self) -> &::std::option::Option<crate::types::WirelessDeviceType> {
        self.inner.get_wireless_device_type()
    }
}
