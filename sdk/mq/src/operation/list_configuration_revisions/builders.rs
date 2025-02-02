// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::list_configuration_revisions::_list_configuration_revisions_output::ListConfigurationRevisionsOutputBuilder;

pub use crate::operation::list_configuration_revisions::_list_configuration_revisions_input::ListConfigurationRevisionsInputBuilder;

impl ListConfigurationRevisionsInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::list_configuration_revisions::ListConfigurationRevisionsOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::list_configuration_revisions::ListConfigurationRevisionsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.list_configuration_revisions();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `ListConfigurationRevisions`.
///
/// <p>Returns a list of all revisions for the specified configuration.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ListConfigurationRevisionsFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::list_configuration_revisions::builders::ListConfigurationRevisionsInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::list_configuration_revisions::ListConfigurationRevisionsOutput,
        crate::operation::list_configuration_revisions::ListConfigurationRevisionsError,
    > for ListConfigurationRevisionsFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::list_configuration_revisions::ListConfigurationRevisionsOutput,
            crate::operation::list_configuration_revisions::ListConfigurationRevisionsError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl ListConfigurationRevisionsFluentBuilder {
    /// Creates a new `ListConfigurationRevisions`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the ListConfigurationRevisions as a reference.
    pub fn as_input(&self) -> &crate::operation::list_configuration_revisions::builders::ListConfigurationRevisionsInputBuilder {
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
        crate::operation::list_configuration_revisions::ListConfigurationRevisionsOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::list_configuration_revisions::ListConfigurationRevisionsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self.inner.build().map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::list_configuration_revisions::ListConfigurationRevisions::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::list_configuration_revisions::ListConfigurationRevisions::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent.
    // TODO(enableNewSmithyRuntimeCleanup): Remove `async` and `Result` once we switch to orchestrator
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::orchestrator::CustomizableOperation<
            crate::operation::list_configuration_revisions::ListConfigurationRevisionsOutput,
            crate::operation::list_configuration_revisions::ListConfigurationRevisionsError,
            Self,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::list_configuration_revisions::ListConfigurationRevisionsError>,
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
    /// <p>The unique ID that Amazon MQ generates for the configuration.</p>
    pub fn configuration_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.configuration_id(input.into());
        self
    }
    /// <p>The unique ID that Amazon MQ generates for the configuration.</p>
    pub fn set_configuration_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_configuration_id(input);
        self
    }
    /// <p>The unique ID that Amazon MQ generates for the configuration.</p>
    pub fn get_configuration_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_configuration_id()
    }
    /// <p>The maximum number of brokers that Amazon MQ can return per page (20 by default). This value must be an integer from 5 to 100.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_results(input);
        self
    }
    /// <p>The maximum number of brokers that Amazon MQ can return per page (20 by default). This value must be an integer from 5 to 100.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
    /// <p>The maximum number of brokers that Amazon MQ can return per page (20 by default). This value must be an integer from 5 to 100.</p>
    pub fn get_max_results(&self) -> &::std::option::Option<i32> {
        self.inner.get_max_results()
    }
    /// <p>The token that specifies the next page of results Amazon MQ should return. To request the first page, leave nextToken empty.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>The token that specifies the next page of results Amazon MQ should return. To request the first page, leave nextToken empty.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
    /// <p>The token that specifies the next page of results Amazon MQ should return. To request the first page, leave nextToken empty.</p>
    pub fn get_next_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_next_token()
    }
}
