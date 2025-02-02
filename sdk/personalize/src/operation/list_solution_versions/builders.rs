// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::list_solution_versions::_list_solution_versions_output::ListSolutionVersionsOutputBuilder;

pub use crate::operation::list_solution_versions::_list_solution_versions_input::ListSolutionVersionsInputBuilder;

impl ListSolutionVersionsInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::list_solution_versions::ListSolutionVersionsOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::list_solution_versions::ListSolutionVersionsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.list_solution_versions();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `ListSolutionVersions`.
///
/// <p>Returns a list of solution versions for the given solution. When a solution is not specified, all the solution versions associated with the account are listed. The response provides the properties for each solution version, including the Amazon Resource Name (ARN).</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ListSolutionVersionsFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::list_solution_versions::builders::ListSolutionVersionsInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::list_solution_versions::ListSolutionVersionsOutput,
        crate::operation::list_solution_versions::ListSolutionVersionsError,
    > for ListSolutionVersionsFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::list_solution_versions::ListSolutionVersionsOutput,
            crate::operation::list_solution_versions::ListSolutionVersionsError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl ListSolutionVersionsFluentBuilder {
    /// Creates a new `ListSolutionVersions`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the ListSolutionVersions as a reference.
    pub fn as_input(&self) -> &crate::operation::list_solution_versions::builders::ListSolutionVersionsInputBuilder {
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
        crate::operation::list_solution_versions::ListSolutionVersionsOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::list_solution_versions::ListSolutionVersionsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self.inner.build().map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::list_solution_versions::ListSolutionVersions::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::list_solution_versions::ListSolutionVersions::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent.
    // TODO(enableNewSmithyRuntimeCleanup): Remove `async` and `Result` once we switch to orchestrator
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::orchestrator::CustomizableOperation<
            crate::operation::list_solution_versions::ListSolutionVersionsOutput,
            crate::operation::list_solution_versions::ListSolutionVersionsError,
            Self,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::list_solution_versions::ListSolutionVersionsError>,
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
    /// Create a paginator for this request
    ///
    /// Paginators are used by calling [`send().await`](crate::operation::list_solution_versions::paginator::ListSolutionVersionsPaginator::send) which returns a `Stream`.
    pub fn into_paginator(self) -> crate::operation::list_solution_versions::paginator::ListSolutionVersionsPaginator {
        crate::operation::list_solution_versions::paginator::ListSolutionVersionsPaginator::new(self.handle, self.inner)
    }
    /// <p>The Amazon Resource Name (ARN) of the solution.</p>
    pub fn solution_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.solution_arn(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the solution.</p>
    pub fn set_solution_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_solution_arn(input);
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the solution.</p>
    pub fn get_solution_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_solution_arn()
    }
    /// <p>A token returned from the previous call to <code>ListSolutionVersions</code> for getting the next set of solution versions (if they exist).</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>A token returned from the previous call to <code>ListSolutionVersions</code> for getting the next set of solution versions (if they exist).</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
    /// <p>A token returned from the previous call to <code>ListSolutionVersions</code> for getting the next set of solution versions (if they exist).</p>
    pub fn get_next_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_next_token()
    }
    /// <p>The maximum number of solution versions to return.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_results(input);
        self
    }
    /// <p>The maximum number of solution versions to return.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
    /// <p>The maximum number of solution versions to return.</p>
    pub fn get_max_results(&self) -> &::std::option::Option<i32> {
        self.inner.get_max_results()
    }
}
