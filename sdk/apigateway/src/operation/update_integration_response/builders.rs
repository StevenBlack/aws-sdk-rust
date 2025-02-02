// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_integration_response::_update_integration_response_output::UpdateIntegrationResponseOutputBuilder;

pub use crate::operation::update_integration_response::_update_integration_response_input::UpdateIntegrationResponseInputBuilder;

impl UpdateIntegrationResponseInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::update_integration_response::UpdateIntegrationResponseOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_integration_response::UpdateIntegrationResponseError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.update_integration_response();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `UpdateIntegrationResponse`.
///
/// <p>Represents an update integration response.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UpdateIntegrationResponseFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_integration_response::builders::UpdateIntegrationResponseInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::update_integration_response::UpdateIntegrationResponseOutput,
        crate::operation::update_integration_response::UpdateIntegrationResponseError,
    > for UpdateIntegrationResponseFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::update_integration_response::UpdateIntegrationResponseOutput,
            crate::operation::update_integration_response::UpdateIntegrationResponseError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl UpdateIntegrationResponseFluentBuilder {
    /// Creates a new `UpdateIntegrationResponse`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the UpdateIntegrationResponse as a reference.
    pub fn as_input(&self) -> &crate::operation::update_integration_response::builders::UpdateIntegrationResponseInputBuilder {
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
        crate::operation::update_integration_response::UpdateIntegrationResponseOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_integration_response::UpdateIntegrationResponseError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self.inner.build().map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::update_integration_response::UpdateIntegrationResponse::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::update_integration_response::UpdateIntegrationResponse::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent.
    // TODO(enableNewSmithyRuntimeCleanup): Remove `async` and `Result` once we switch to orchestrator
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::orchestrator::CustomizableOperation<
            crate::operation::update_integration_response::UpdateIntegrationResponseOutput,
            crate::operation::update_integration_response::UpdateIntegrationResponseError,
            Self,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::update_integration_response::UpdateIntegrationResponseError>,
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
    /// <p>The string identifier of the associated RestApi.</p>
    pub fn rest_api_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.rest_api_id(input.into());
        self
    }
    /// <p>The string identifier of the associated RestApi.</p>
    pub fn set_rest_api_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_rest_api_id(input);
        self
    }
    /// <p>The string identifier of the associated RestApi.</p>
    pub fn get_rest_api_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_rest_api_id()
    }
    /// <p>Specifies an update integration response request's resource identifier.</p>
    pub fn resource_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.resource_id(input.into());
        self
    }
    /// <p>Specifies an update integration response request's resource identifier.</p>
    pub fn set_resource_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_resource_id(input);
        self
    }
    /// <p>Specifies an update integration response request's resource identifier.</p>
    pub fn get_resource_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_resource_id()
    }
    /// <p>Specifies an update integration response request's HTTP method.</p>
    pub fn http_method(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.http_method(input.into());
        self
    }
    /// <p>Specifies an update integration response request's HTTP method.</p>
    pub fn set_http_method(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_http_method(input);
        self
    }
    /// <p>Specifies an update integration response request's HTTP method.</p>
    pub fn get_http_method(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_http_method()
    }
    /// <p>Specifies an update integration response request's status code.</p>
    pub fn status_code(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.status_code(input.into());
        self
    }
    /// <p>Specifies an update integration response request's status code.</p>
    pub fn set_status_code(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_status_code(input);
        self
    }
    /// <p>Specifies an update integration response request's status code.</p>
    pub fn get_status_code(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_status_code()
    }
    /// Appends an item to `patchOperations`.
    ///
    /// To override the contents of this collection use [`set_patch_operations`](Self::set_patch_operations).
    ///
    /// <p>For more information about supported patch operations, see <a href="https://docs.aws.amazon.com/apigateway/latest/api/patch-operations.html">Patch Operations</a>.</p>
    pub fn patch_operations(mut self, input: crate::types::PatchOperation) -> Self {
        self.inner = self.inner.patch_operations(input);
        self
    }
    /// <p>For more information about supported patch operations, see <a href="https://docs.aws.amazon.com/apigateway/latest/api/patch-operations.html">Patch Operations</a>.</p>
    pub fn set_patch_operations(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::PatchOperation>>) -> Self {
        self.inner = self.inner.set_patch_operations(input);
        self
    }
    /// <p>For more information about supported patch operations, see <a href="https://docs.aws.amazon.com/apigateway/latest/api/patch-operations.html">Patch Operations</a>.</p>
    pub fn get_patch_operations(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::PatchOperation>> {
        self.inner.get_patch_operations()
    }
}
