// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::get_folder::_get_folder_output::GetFolderOutputBuilder;

pub use crate::operation::get_folder::_get_folder_input::GetFolderInputBuilder;

impl GetFolderInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::get_folder::GetFolderOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::get_folder::GetFolderError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.get_folder();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `GetFolder`.
///
/// <p>Retrieves the metadata of the specified folder.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct GetFolderFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::get_folder::builders::GetFolderInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl crate::client::customize::internal::CustomizableSend<crate::operation::get_folder::GetFolderOutput, crate::operation::get_folder::GetFolderError>
    for GetFolderFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<crate::operation::get_folder::GetFolderOutput, crate::operation::get_folder::GetFolderError>,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl GetFolderFluentBuilder {
    /// Creates a new `GetFolder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the GetFolder as a reference.
    pub fn as_input(&self) -> &crate::operation::get_folder::builders::GetFolderInputBuilder {
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
        crate::operation::get_folder::GetFolderOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::get_folder::GetFolderError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self.inner.build().map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::get_folder::GetFolder::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::get_folder::GetFolder::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent.
    // TODO(enableNewSmithyRuntimeCleanup): Remove `async` and `Result` once we switch to orchestrator
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::orchestrator::CustomizableOperation<
            crate::operation::get_folder::GetFolderOutput,
            crate::operation::get_folder::GetFolderError,
            Self,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::get_folder::GetFolderError>,
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
    /// <p>Amazon WorkDocs authentication token. Not required when using Amazon Web Services administrator credentials to access the API.</p>
    pub fn authentication_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.authentication_token(input.into());
        self
    }
    /// <p>Amazon WorkDocs authentication token. Not required when using Amazon Web Services administrator credentials to access the API.</p>
    pub fn set_authentication_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_authentication_token(input);
        self
    }
    /// <p>Amazon WorkDocs authentication token. Not required when using Amazon Web Services administrator credentials to access the API.</p>
    pub fn get_authentication_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_authentication_token()
    }
    /// <p>The ID of the folder.</p>
    pub fn folder_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.folder_id(input.into());
        self
    }
    /// <p>The ID of the folder.</p>
    pub fn set_folder_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_folder_id(input);
        self
    }
    /// <p>The ID of the folder.</p>
    pub fn get_folder_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_folder_id()
    }
    /// <p>Set to TRUE to include custom metadata in the response.</p>
    pub fn include_custom_metadata(mut self, input: bool) -> Self {
        self.inner = self.inner.include_custom_metadata(input);
        self
    }
    /// <p>Set to TRUE to include custom metadata in the response.</p>
    pub fn set_include_custom_metadata(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_include_custom_metadata(input);
        self
    }
    /// <p>Set to TRUE to include custom metadata in the response.</p>
    pub fn get_include_custom_metadata(&self) -> &::std::option::Option<bool> {
        self.inner.get_include_custom_metadata()
    }
}
