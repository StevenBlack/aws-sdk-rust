// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::search_devices::_search_devices_output::SearchDevicesOutputBuilder;

pub use crate::operation::search_devices::_search_devices_input::SearchDevicesInputBuilder;

impl SearchDevicesInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::search_devices::SearchDevicesOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::search_devices::SearchDevicesError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.search_devices();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `SearchDevices`.
///
/// <p>Searches for devices using the specified filters.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct SearchDevicesFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::search_devices::builders::SearchDevicesInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::search_devices::SearchDevicesOutput,
        crate::operation::search_devices::SearchDevicesError,
    > for SearchDevicesFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::search_devices::SearchDevicesOutput,
            crate::operation::search_devices::SearchDevicesError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl SearchDevicesFluentBuilder {
    /// Creates a new `SearchDevices`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the SearchDevices as a reference.
    pub fn as_input(&self) -> &crate::operation::search_devices::builders::SearchDevicesInputBuilder {
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
        crate::operation::search_devices::SearchDevicesOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::search_devices::SearchDevicesError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self.inner.build().map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::search_devices::SearchDevices::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::search_devices::SearchDevices::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent.
    // TODO(enableNewSmithyRuntimeCleanup): Remove `async` and `Result` once we switch to orchestrator
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::orchestrator::CustomizableOperation<
            crate::operation::search_devices::SearchDevicesOutput,
            crate::operation::search_devices::SearchDevicesError,
            Self,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::search_devices::SearchDevicesError>,
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
    /// Paginators are used by calling [`send().await`](crate::operation::search_devices::paginator::SearchDevicesPaginator::send) which returns a `Stream`.
    pub fn into_paginator(self) -> crate::operation::search_devices::paginator::SearchDevicesPaginator {
        crate::operation::search_devices::paginator::SearchDevicesPaginator::new(self.handle, self.inner)
    }
    /// <p>A token used for pagination of results returned in the response. Use the token returned from the previous request continue results where the previous request ended.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>A token used for pagination of results returned in the response. Use the token returned from the previous request continue results where the previous request ended.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
    /// <p>A token used for pagination of results returned in the response. Use the token returned from the previous request continue results where the previous request ended.</p>
    pub fn get_next_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_next_token()
    }
    /// <p>The maximum number of results to return in the response.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_results(input);
        self
    }
    /// <p>The maximum number of results to return in the response.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
    /// <p>The maximum number of results to return in the response.</p>
    pub fn get_max_results(&self) -> &::std::option::Option<i32> {
        self.inner.get_max_results()
    }
    /// Appends an item to `filters`.
    ///
    /// To override the contents of this collection use [`set_filters`](Self::set_filters).
    ///
    /// <p>The filter values to use to search for a device.</p>
    pub fn filters(mut self, input: crate::types::SearchDevicesFilter) -> Self {
        self.inner = self.inner.filters(input);
        self
    }
    /// <p>The filter values to use to search for a device.</p>
    pub fn set_filters(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::SearchDevicesFilter>>) -> Self {
        self.inner = self.inner.set_filters(input);
        self
    }
    /// <p>The filter values to use to search for a device.</p>
    pub fn get_filters(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::SearchDevicesFilter>> {
        self.inner.get_filters()
    }
}
