// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::get_database::_get_database_output::GetDatabaseOutputBuilder;

pub use crate::operation::get_database::_get_database_input::GetDatabaseInputBuilder;

impl GetDatabaseInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::get_database::GetDatabaseOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::get_database::GetDatabaseError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.get_database();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `GetDatabase`.
///
/// <p>Gets the SAP HANA database of an application registered with AWS Systems Manager for SAP.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct GetDatabaseFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::get_database::builders::GetDatabaseInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::get_database::GetDatabaseOutput,
        crate::operation::get_database::GetDatabaseError,
    > for GetDatabaseFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::get_database::GetDatabaseOutput,
            crate::operation::get_database::GetDatabaseError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl GetDatabaseFluentBuilder {
    /// Creates a new `GetDatabase`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the GetDatabase as a reference.
    pub fn as_input(&self) -> &crate::operation::get_database::builders::GetDatabaseInputBuilder {
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
        crate::operation::get_database::GetDatabaseOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::get_database::GetDatabaseError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self.inner.build().map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::get_database::GetDatabase::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::get_database::GetDatabase::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent.
    // TODO(enableNewSmithyRuntimeCleanup): Remove `async` and `Result` once we switch to orchestrator
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::orchestrator::CustomizableOperation<
            crate::operation::get_database::GetDatabaseOutput,
            crate::operation::get_database::GetDatabaseError,
            Self,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::get_database::GetDatabaseError>,
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
    /// <p>The ID of the application.</p>
    pub fn application_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.application_id(input.into());
        self
    }
    /// <p>The ID of the application.</p>
    pub fn set_application_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_application_id(input);
        self
    }
    /// <p>The ID of the application.</p>
    pub fn get_application_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_application_id()
    }
    /// <p>The ID of the component.</p>
    pub fn component_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.component_id(input.into());
        self
    }
    /// <p>The ID of the component.</p>
    pub fn set_component_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_component_id(input);
        self
    }
    /// <p>The ID of the component.</p>
    pub fn get_component_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_component_id()
    }
    /// <p>The ID of the database.</p>
    pub fn database_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.database_id(input.into());
        self
    }
    /// <p>The ID of the database.</p>
    pub fn set_database_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_database_id(input);
        self
    }
    /// <p>The ID of the database.</p>
    pub fn get_database_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_database_id()
    }
    /// <p>The Amazon Resource Name (ARN) of the database.</p>
    pub fn database_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.database_arn(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the database.</p>
    pub fn set_database_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_database_arn(input);
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the database.</p>
    pub fn get_database_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_database_arn()
    }
}
