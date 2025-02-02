// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_function::_create_function_output::CreateFunctionOutputBuilder;

pub use crate::operation::create_function::_create_function_input::CreateFunctionInputBuilder;

impl CreateFunctionInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::create_function::CreateFunctionOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::create_function::CreateFunctionError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.create_function();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `CreateFunction`.
///
/// <p>Creates a CloudFront function.</p>
/// <p>To create a function, you provide the function code and some configuration information about the function. The response contains an Amazon Resource Name (ARN) that uniquely identifies the function.</p>
/// <p>When you create a function, it's in the <code>DEVELOPMENT</code> stage. In this stage, you can test the function with <code>TestFunction</code>, and update it with <code>UpdateFunction</code>.</p>
/// <p>When you're ready to use your function with a CloudFront distribution, use <code>PublishFunction</code> to copy the function from the <code>DEVELOPMENT</code> stage to <code>LIVE</code>. When it's live, you can attach the function to a distribution's cache behavior, using the function's ARN.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateFunctionFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_function::builders::CreateFunctionInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::create_function::CreateFunctionOutput,
        crate::operation::create_function::CreateFunctionError,
    > for CreateFunctionFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::create_function::CreateFunctionOutput,
            crate::operation::create_function::CreateFunctionError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl CreateFunctionFluentBuilder {
    /// Creates a new `CreateFunction`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the CreateFunction as a reference.
    pub fn as_input(&self) -> &crate::operation::create_function::builders::CreateFunctionInputBuilder {
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
        crate::operation::create_function::CreateFunctionOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::create_function::CreateFunctionError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self.inner.build().map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::create_function::CreateFunction::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::create_function::CreateFunction::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent.
    // TODO(enableNewSmithyRuntimeCleanup): Remove `async` and `Result` once we switch to orchestrator
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::orchestrator::CustomizableOperation<
            crate::operation::create_function::CreateFunctionOutput,
            crate::operation::create_function::CreateFunctionError,
            Self,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::create_function::CreateFunctionError>,
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
    /// <p>A name to identify the function.</p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.name(input.into());
        self
    }
    /// <p>A name to identify the function.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_name(input);
        self
    }
    /// <p>A name to identify the function.</p>
    pub fn get_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_name()
    }
    /// <p>Configuration information about the function, including an optional comment and the function's runtime.</p>
    pub fn function_config(mut self, input: crate::types::FunctionConfig) -> Self {
        self.inner = self.inner.function_config(input);
        self
    }
    /// <p>Configuration information about the function, including an optional comment and the function's runtime.</p>
    pub fn set_function_config(mut self, input: ::std::option::Option<crate::types::FunctionConfig>) -> Self {
        self.inner = self.inner.set_function_config(input);
        self
    }
    /// <p>Configuration information about the function, including an optional comment and the function's runtime.</p>
    pub fn get_function_config(&self) -> &::std::option::Option<crate::types::FunctionConfig> {
        self.inner.get_function_config()
    }
    /// <p>The function code. For more information about writing a CloudFront function, see <a href="https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/writing-function-code.html">Writing function code for CloudFront Functions</a> in the <i>Amazon CloudFront Developer Guide</i>.</p>
    pub fn function_code(mut self, input: ::aws_smithy_types::Blob) -> Self {
        self.inner = self.inner.function_code(input);
        self
    }
    /// <p>The function code. For more information about writing a CloudFront function, see <a href="https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/writing-function-code.html">Writing function code for CloudFront Functions</a> in the <i>Amazon CloudFront Developer Guide</i>.</p>
    pub fn set_function_code(mut self, input: ::std::option::Option<::aws_smithy_types::Blob>) -> Self {
        self.inner = self.inner.set_function_code(input);
        self
    }
    /// <p>The function code. For more information about writing a CloudFront function, see <a href="https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/writing-function-code.html">Writing function code for CloudFront Functions</a> in the <i>Amazon CloudFront Developer Guide</i>.</p>
    pub fn get_function_code(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
        self.inner.get_function_code()
    }
}
