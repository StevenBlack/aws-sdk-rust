// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::list_ca_certificates::_list_ca_certificates_output::ListCaCertificatesOutputBuilder;

pub use crate::operation::list_ca_certificates::_list_ca_certificates_input::ListCaCertificatesInputBuilder;

impl ListCaCertificatesInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::list_ca_certificates::ListCaCertificatesOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::list_ca_certificates::ListCACertificatesError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.list_ca_certificates();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `ListCACertificates`.
///
/// <p>Lists the CA certificates registered for your Amazon Web Services account.</p>
/// <p>The results are paginated with a default page size of 25. You can use the returned marker to retrieve additional results.</p>
/// <p>Requires permission to access the <a href="https://docs.aws.amazon.com/service-authorization/latest/reference/list_awsiot.html#awsiot-actions-as-permissions">ListCACertificates</a> action.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ListCACertificatesFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::list_ca_certificates::builders::ListCaCertificatesInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::list_ca_certificates::ListCaCertificatesOutput,
        crate::operation::list_ca_certificates::ListCACertificatesError,
    > for ListCACertificatesFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::list_ca_certificates::ListCaCertificatesOutput,
            crate::operation::list_ca_certificates::ListCACertificatesError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl ListCACertificatesFluentBuilder {
    /// Creates a new `ListCACertificates`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the ListCACertificates as a reference.
    pub fn as_input(&self) -> &crate::operation::list_ca_certificates::builders::ListCaCertificatesInputBuilder {
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
        crate::operation::list_ca_certificates::ListCaCertificatesOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::list_ca_certificates::ListCACertificatesError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self.inner.build().map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::list_ca_certificates::ListCACertificates::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::list_ca_certificates::ListCACertificates::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent.
    // TODO(enableNewSmithyRuntimeCleanup): Remove `async` and `Result` once we switch to orchestrator
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::orchestrator::CustomizableOperation<
            crate::operation::list_ca_certificates::ListCaCertificatesOutput,
            crate::operation::list_ca_certificates::ListCACertificatesError,
            Self,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::list_ca_certificates::ListCACertificatesError>,
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
    /// Paginators are used by calling [`send().await`](crate::operation::list_ca_certificates::paginator::ListCaCertificatesPaginator::send) which returns a `Stream`.
    pub fn into_paginator(self) -> crate::operation::list_ca_certificates::paginator::ListCaCertificatesPaginator {
        crate::operation::list_ca_certificates::paginator::ListCaCertificatesPaginator::new(self.handle, self.inner)
    }
    /// <p>The result page size.</p>
    pub fn page_size(mut self, input: i32) -> Self {
        self.inner = self.inner.page_size(input);
        self
    }
    /// <p>The result page size.</p>
    pub fn set_page_size(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_page_size(input);
        self
    }
    /// <p>The result page size.</p>
    pub fn get_page_size(&self) -> &::std::option::Option<i32> {
        self.inner.get_page_size()
    }
    /// <p>The marker for the next set of results.</p>
    pub fn marker(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.marker(input.into());
        self
    }
    /// <p>The marker for the next set of results.</p>
    pub fn set_marker(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_marker(input);
        self
    }
    /// <p>The marker for the next set of results.</p>
    pub fn get_marker(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_marker()
    }
    /// <p>Determines the order of the results.</p>
    pub fn ascending_order(mut self, input: bool) -> Self {
        self.inner = self.inner.ascending_order(input);
        self
    }
    /// <p>Determines the order of the results.</p>
    pub fn set_ascending_order(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_ascending_order(input);
        self
    }
    /// <p>Determines the order of the results.</p>
    pub fn get_ascending_order(&self) -> &::std::option::Option<bool> {
        self.inner.get_ascending_order()
    }
    /// <p>The name of the provisioning template.</p>
    pub fn template_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.template_name(input.into());
        self
    }
    /// <p>The name of the provisioning template.</p>
    pub fn set_template_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_template_name(input);
        self
    }
    /// <p>The name of the provisioning template.</p>
    pub fn get_template_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_template_name()
    }
}
