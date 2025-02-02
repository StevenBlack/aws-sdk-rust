// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::list_vpce_configurations::_list_vpce_configurations_output::ListVpceConfigurationsOutputBuilder;

pub use crate::operation::list_vpce_configurations::_list_vpce_configurations_input::ListVpceConfigurationsInputBuilder;

impl ListVpceConfigurationsInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::list_vpce_configurations::ListVpceConfigurationsOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::list_vpce_configurations::ListVPCEConfigurationsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.list_vpce_configurations();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `ListVPCEConfigurations`.
///
/// <p>Returns information about all Amazon Virtual Private Cloud (VPC) endpoint configurations in the AWS account.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ListVPCEConfigurationsFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::list_vpce_configurations::builders::ListVpceConfigurationsInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::list_vpce_configurations::ListVpceConfigurationsOutput,
        crate::operation::list_vpce_configurations::ListVPCEConfigurationsError,
    > for ListVPCEConfigurationsFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::list_vpce_configurations::ListVpceConfigurationsOutput,
            crate::operation::list_vpce_configurations::ListVPCEConfigurationsError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl ListVPCEConfigurationsFluentBuilder {
    /// Creates a new `ListVPCEConfigurations`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the ListVPCEConfigurations as a reference.
    pub fn as_input(&self) -> &crate::operation::list_vpce_configurations::builders::ListVpceConfigurationsInputBuilder {
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
        crate::operation::list_vpce_configurations::ListVpceConfigurationsOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::list_vpce_configurations::ListVPCEConfigurationsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self.inner.build().map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::list_vpce_configurations::ListVPCEConfigurations::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::list_vpce_configurations::ListVPCEConfigurations::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent.
    // TODO(enableNewSmithyRuntimeCleanup): Remove `async` and `Result` once we switch to orchestrator
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::orchestrator::CustomizableOperation<
            crate::operation::list_vpce_configurations::ListVpceConfigurationsOutput,
            crate::operation::list_vpce_configurations::ListVPCEConfigurationsError,
            Self,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::list_vpce_configurations::ListVPCEConfigurationsError>,
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
    /// <p>An integer that specifies the maximum number of items you want to return in the API response.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_results(input);
        self
    }
    /// <p>An integer that specifies the maximum number of items you want to return in the API response.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
    /// <p>An integer that specifies the maximum number of items you want to return in the API response.</p>
    pub fn get_max_results(&self) -> &::std::option::Option<i32> {
        self.inner.get_max_results()
    }
    /// <p>An identifier that was returned from the previous call to this operation, which can be used to return the next set of items in the list.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>An identifier that was returned from the previous call to this operation, which can be used to return the next set of items in the list.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
    /// <p>An identifier that was returned from the previous call to this operation, which can be used to return the next set of items in the list.</p>
    pub fn get_next_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_next_token()
    }
}
