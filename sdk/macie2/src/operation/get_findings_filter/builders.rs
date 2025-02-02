// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::get_findings_filter::_get_findings_filter_output::GetFindingsFilterOutputBuilder;

pub use crate::operation::get_findings_filter::_get_findings_filter_input::GetFindingsFilterInputBuilder;

impl GetFindingsFilterInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::get_findings_filter::GetFindingsFilterOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::get_findings_filter::GetFindingsFilterError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.get_findings_filter();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `GetFindingsFilter`.
///
/// <p>Retrieves the criteria and other settings for a findings filter.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct GetFindingsFilterFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::get_findings_filter::builders::GetFindingsFilterInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::get_findings_filter::GetFindingsFilterOutput,
        crate::operation::get_findings_filter::GetFindingsFilterError,
    > for GetFindingsFilterFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::get_findings_filter::GetFindingsFilterOutput,
            crate::operation::get_findings_filter::GetFindingsFilterError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl GetFindingsFilterFluentBuilder {
    /// Creates a new `GetFindingsFilter`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the GetFindingsFilter as a reference.
    pub fn as_input(&self) -> &crate::operation::get_findings_filter::builders::GetFindingsFilterInputBuilder {
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
        crate::operation::get_findings_filter::GetFindingsFilterOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::get_findings_filter::GetFindingsFilterError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self.inner.build().map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::get_findings_filter::GetFindingsFilter::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::get_findings_filter::GetFindingsFilter::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent.
    // TODO(enableNewSmithyRuntimeCleanup): Remove `async` and `Result` once we switch to orchestrator
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::orchestrator::CustomizableOperation<
            crate::operation::get_findings_filter::GetFindingsFilterOutput,
            crate::operation::get_findings_filter::GetFindingsFilterError,
            Self,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::get_findings_filter::GetFindingsFilterError>,
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
    /// <p>The unique identifier for the Amazon Macie resource that the request applies to.</p>
    pub fn id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.id(input.into());
        self
    }
    /// <p>The unique identifier for the Amazon Macie resource that the request applies to.</p>
    pub fn set_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_id(input);
        self
    }
    /// <p>The unique identifier for the Amazon Macie resource that the request applies to.</p>
    pub fn get_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_id()
    }
}
