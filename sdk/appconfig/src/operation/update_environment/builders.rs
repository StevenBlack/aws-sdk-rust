// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_environment::_update_environment_output::UpdateEnvironmentOutputBuilder;

pub use crate::operation::update_environment::_update_environment_input::UpdateEnvironmentInputBuilder;

impl UpdateEnvironmentInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::update_environment::UpdateEnvironmentOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_environment::UpdateEnvironmentError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.update_environment();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `UpdateEnvironment`.
///
/// <p>Updates an environment.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UpdateEnvironmentFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_environment::builders::UpdateEnvironmentInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::update_environment::UpdateEnvironmentOutput,
        crate::operation::update_environment::UpdateEnvironmentError,
    > for UpdateEnvironmentFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::update_environment::UpdateEnvironmentOutput,
            crate::operation::update_environment::UpdateEnvironmentError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl UpdateEnvironmentFluentBuilder {
    /// Creates a new `UpdateEnvironment`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the UpdateEnvironment as a reference.
    pub fn as_input(&self) -> &crate::operation::update_environment::builders::UpdateEnvironmentInputBuilder {
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
        crate::operation::update_environment::UpdateEnvironmentOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_environment::UpdateEnvironmentError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self.inner.build().map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::update_environment::UpdateEnvironment::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::update_environment::UpdateEnvironment::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent.
    // TODO(enableNewSmithyRuntimeCleanup): Remove `async` and `Result` once we switch to orchestrator
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::orchestrator::CustomizableOperation<
            crate::operation::update_environment::UpdateEnvironmentOutput,
            crate::operation::update_environment::UpdateEnvironmentError,
            Self,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::update_environment::UpdateEnvironmentError>,
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
    /// <p>The application ID.</p>
    pub fn application_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.application_id(input.into());
        self
    }
    /// <p>The application ID.</p>
    pub fn set_application_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_application_id(input);
        self
    }
    /// <p>The application ID.</p>
    pub fn get_application_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_application_id()
    }
    /// <p>The environment ID.</p>
    pub fn environment_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.environment_id(input.into());
        self
    }
    /// <p>The environment ID.</p>
    pub fn set_environment_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_environment_id(input);
        self
    }
    /// <p>The environment ID.</p>
    pub fn get_environment_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_environment_id()
    }
    /// <p>The name of the environment.</p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.name(input.into());
        self
    }
    /// <p>The name of the environment.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_name(input);
        self
    }
    /// <p>The name of the environment.</p>
    pub fn get_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_name()
    }
    /// <p>A description of the environment.</p>
    pub fn description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.description(input.into());
        self
    }
    /// <p>A description of the environment.</p>
    pub fn set_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_description(input);
        self
    }
    /// <p>A description of the environment.</p>
    pub fn get_description(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_description()
    }
    /// Appends an item to `Monitors`.
    ///
    /// To override the contents of this collection use [`set_monitors`](Self::set_monitors).
    ///
    /// <p>Amazon CloudWatch alarms to monitor during the deployment process.</p>
    pub fn monitors(mut self, input: crate::types::Monitor) -> Self {
        self.inner = self.inner.monitors(input);
        self
    }
    /// <p>Amazon CloudWatch alarms to monitor during the deployment process.</p>
    pub fn set_monitors(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::Monitor>>) -> Self {
        self.inner = self.inner.set_monitors(input);
        self
    }
    /// <p>Amazon CloudWatch alarms to monitor during the deployment process.</p>
    pub fn get_monitors(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::Monitor>> {
        self.inner.get_monitors()
    }
}
