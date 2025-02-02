// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::get_relational_database_bundles::_get_relational_database_bundles_output::GetRelationalDatabaseBundlesOutputBuilder;

pub use crate::operation::get_relational_database_bundles::_get_relational_database_bundles_input::GetRelationalDatabaseBundlesInputBuilder;

impl GetRelationalDatabaseBundlesInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::get_relational_database_bundles::GetRelationalDatabaseBundlesOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::get_relational_database_bundles::GetRelationalDatabaseBundlesError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.get_relational_database_bundles();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `GetRelationalDatabaseBundles`.
///
/// <p>Returns the list of bundles that are available in Amazon Lightsail. A bundle describes the performance specifications for a database.</p>
/// <p>You can use a bundle ID to create a new database with explicit performance specifications.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct GetRelationalDatabaseBundlesFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::get_relational_database_bundles::builders::GetRelationalDatabaseBundlesInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::get_relational_database_bundles::GetRelationalDatabaseBundlesOutput,
        crate::operation::get_relational_database_bundles::GetRelationalDatabaseBundlesError,
    > for GetRelationalDatabaseBundlesFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::get_relational_database_bundles::GetRelationalDatabaseBundlesOutput,
            crate::operation::get_relational_database_bundles::GetRelationalDatabaseBundlesError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl GetRelationalDatabaseBundlesFluentBuilder {
    /// Creates a new `GetRelationalDatabaseBundles`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the GetRelationalDatabaseBundles as a reference.
    pub fn as_input(&self) -> &crate::operation::get_relational_database_bundles::builders::GetRelationalDatabaseBundlesInputBuilder {
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
        crate::operation::get_relational_database_bundles::GetRelationalDatabaseBundlesOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::get_relational_database_bundles::GetRelationalDatabaseBundlesError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self.inner.build().map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::get_relational_database_bundles::GetRelationalDatabaseBundles::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::get_relational_database_bundles::GetRelationalDatabaseBundles::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent.
    // TODO(enableNewSmithyRuntimeCleanup): Remove `async` and `Result` once we switch to orchestrator
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::orchestrator::CustomizableOperation<
            crate::operation::get_relational_database_bundles::GetRelationalDatabaseBundlesOutput,
            crate::operation::get_relational_database_bundles::GetRelationalDatabaseBundlesError,
            Self,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::get_relational_database_bundles::GetRelationalDatabaseBundlesError>,
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
    /// <p>The token to advance to the next page of results from your request.</p>
    /// <p>To get a page token, perform an initial <code>GetRelationalDatabaseBundles</code> request. If your results are paginated, the response will return a next page token that you can specify as the page token in a subsequent request.</p>
    pub fn page_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.page_token(input.into());
        self
    }
    /// <p>The token to advance to the next page of results from your request.</p>
    /// <p>To get a page token, perform an initial <code>GetRelationalDatabaseBundles</code> request. If your results are paginated, the response will return a next page token that you can specify as the page token in a subsequent request.</p>
    pub fn set_page_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_page_token(input);
        self
    }
    /// <p>The token to advance to the next page of results from your request.</p>
    /// <p>To get a page token, perform an initial <code>GetRelationalDatabaseBundles</code> request. If your results are paginated, the response will return a next page token that you can specify as the page token in a subsequent request.</p>
    pub fn get_page_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_page_token()
    }
    /// <p>A Boolean value that indicates whether to include inactive (unavailable) bundles in the response of your request.</p>
    pub fn include_inactive(mut self, input: bool) -> Self {
        self.inner = self.inner.include_inactive(input);
        self
    }
    /// <p>A Boolean value that indicates whether to include inactive (unavailable) bundles in the response of your request.</p>
    pub fn set_include_inactive(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_include_inactive(input);
        self
    }
    /// <p>A Boolean value that indicates whether to include inactive (unavailable) bundles in the response of your request.</p>
    pub fn get_include_inactive(&self) -> &::std::option::Option<bool> {
        self.inner.get_include_inactive()
    }
}
