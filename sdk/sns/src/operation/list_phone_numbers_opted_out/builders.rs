// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::list_phone_numbers_opted_out::_list_phone_numbers_opted_out_output::ListPhoneNumbersOptedOutOutputBuilder;

pub use crate::operation::list_phone_numbers_opted_out::_list_phone_numbers_opted_out_input::ListPhoneNumbersOptedOutInputBuilder;

impl ListPhoneNumbersOptedOutInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::list_phone_numbers_opted_out::ListPhoneNumbersOptedOutOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::list_phone_numbers_opted_out::ListPhoneNumbersOptedOutError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.list_phone_numbers_opted_out();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `ListPhoneNumbersOptedOut`.
///
/// <p>Returns a list of phone numbers that are opted out, meaning you cannot send SMS messages to them.</p>
/// <p>The results for <code>ListPhoneNumbersOptedOut</code> are paginated, and each page returns up to 100 phone numbers. If additional phone numbers are available after the first page of results, then a <code>NextToken</code> string will be returned. To receive the next page, you call <code>ListPhoneNumbersOptedOut</code> again using the <code>NextToken</code> string received from the previous call. When there are no more records to return, <code>NextToken</code> will be null.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ListPhoneNumbersOptedOutFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::list_phone_numbers_opted_out::builders::ListPhoneNumbersOptedOutInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::list_phone_numbers_opted_out::ListPhoneNumbersOptedOutOutput,
        crate::operation::list_phone_numbers_opted_out::ListPhoneNumbersOptedOutError,
    > for ListPhoneNumbersOptedOutFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::list_phone_numbers_opted_out::ListPhoneNumbersOptedOutOutput,
            crate::operation::list_phone_numbers_opted_out::ListPhoneNumbersOptedOutError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl ListPhoneNumbersOptedOutFluentBuilder {
    /// Creates a new `ListPhoneNumbersOptedOut`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the ListPhoneNumbersOptedOut as a reference.
    pub fn as_input(&self) -> &crate::operation::list_phone_numbers_opted_out::builders::ListPhoneNumbersOptedOutInputBuilder {
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
        crate::operation::list_phone_numbers_opted_out::ListPhoneNumbersOptedOutOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::list_phone_numbers_opted_out::ListPhoneNumbersOptedOutError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self.inner.build().map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::list_phone_numbers_opted_out::ListPhoneNumbersOptedOut::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::list_phone_numbers_opted_out::ListPhoneNumbersOptedOut::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent.
    // TODO(enableNewSmithyRuntimeCleanup): Remove `async` and `Result` once we switch to orchestrator
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::orchestrator::CustomizableOperation<
            crate::operation::list_phone_numbers_opted_out::ListPhoneNumbersOptedOutOutput,
            crate::operation::list_phone_numbers_opted_out::ListPhoneNumbersOptedOutError,
            Self,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::list_phone_numbers_opted_out::ListPhoneNumbersOptedOutError>,
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
    /// Paginators are used by calling [`send().await`](crate::operation::list_phone_numbers_opted_out::paginator::ListPhoneNumbersOptedOutPaginator::send) which returns a `Stream`.
    pub fn into_paginator(self) -> crate::operation::list_phone_numbers_opted_out::paginator::ListPhoneNumbersOptedOutPaginator {
        crate::operation::list_phone_numbers_opted_out::paginator::ListPhoneNumbersOptedOutPaginator::new(self.handle, self.inner)
    }
    /// <p>A <code>NextToken</code> string is used when you call the <code>ListPhoneNumbersOptedOut</code> action to retrieve additional records that are available after the first page of results.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>A <code>NextToken</code> string is used when you call the <code>ListPhoneNumbersOptedOut</code> action to retrieve additional records that are available after the first page of results.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
    /// <p>A <code>NextToken</code> string is used when you call the <code>ListPhoneNumbersOptedOut</code> action to retrieve additional records that are available after the first page of results.</p>
    pub fn get_next_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_next_token()
    }
}
