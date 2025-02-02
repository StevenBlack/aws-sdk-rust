// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::list_device_definition_versions::_list_device_definition_versions_output::ListDeviceDefinitionVersionsOutputBuilder;

pub use crate::operation::list_device_definition_versions::_list_device_definition_versions_input::ListDeviceDefinitionVersionsInputBuilder;

impl ListDeviceDefinitionVersionsInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::list_device_definition_versions::ListDeviceDefinitionVersionsOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::list_device_definition_versions::ListDeviceDefinitionVersionsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.list_device_definition_versions();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `ListDeviceDefinitionVersions`.
///
/// Lists the versions of a device definition.
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ListDeviceDefinitionVersionsFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::list_device_definition_versions::builders::ListDeviceDefinitionVersionsInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::list_device_definition_versions::ListDeviceDefinitionVersionsOutput,
        crate::operation::list_device_definition_versions::ListDeviceDefinitionVersionsError,
    > for ListDeviceDefinitionVersionsFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::list_device_definition_versions::ListDeviceDefinitionVersionsOutput,
            crate::operation::list_device_definition_versions::ListDeviceDefinitionVersionsError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl ListDeviceDefinitionVersionsFluentBuilder {
    /// Creates a new `ListDeviceDefinitionVersions`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the ListDeviceDefinitionVersions as a reference.
    pub fn as_input(&self) -> &crate::operation::list_device_definition_versions::builders::ListDeviceDefinitionVersionsInputBuilder {
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
        crate::operation::list_device_definition_versions::ListDeviceDefinitionVersionsOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::list_device_definition_versions::ListDeviceDefinitionVersionsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self.inner.build().map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::list_device_definition_versions::ListDeviceDefinitionVersions::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::list_device_definition_versions::ListDeviceDefinitionVersions::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent.
    // TODO(enableNewSmithyRuntimeCleanup): Remove `async` and `Result` once we switch to orchestrator
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::orchestrator::CustomizableOperation<
            crate::operation::list_device_definition_versions::ListDeviceDefinitionVersionsOutput,
            crate::operation::list_device_definition_versions::ListDeviceDefinitionVersionsError,
            Self,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::list_device_definition_versions::ListDeviceDefinitionVersionsError>,
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
    /// The ID of the device definition.
    pub fn device_definition_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.device_definition_id(input.into());
        self
    }
    /// The ID of the device definition.
    pub fn set_device_definition_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_device_definition_id(input);
        self
    }
    /// The ID of the device definition.
    pub fn get_device_definition_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_device_definition_id()
    }
    /// The maximum number of results to be returned per request.
    pub fn max_results(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.max_results(input.into());
        self
    }
    /// The maximum number of results to be returned per request.
    pub fn set_max_results(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
    /// The maximum number of results to be returned per request.
    pub fn get_max_results(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_max_results()
    }
    /// The token for the next set of results, or ''null'' if there are no additional results.
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// The token for the next set of results, or ''null'' if there are no additional results.
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
    /// The token for the next set of results, or ''null'' if there are no additional results.
    pub fn get_next_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_next_token()
    }
}
