// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::list_service_instance_provisioned_resources::_list_service_instance_provisioned_resources_output::ListServiceInstanceProvisionedResourcesOutputBuilder;

pub use crate::operation::list_service_instance_provisioned_resources::_list_service_instance_provisioned_resources_input::ListServiceInstanceProvisionedResourcesInputBuilder;

impl ListServiceInstanceProvisionedResourcesInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::list_service_instance_provisioned_resources::ListServiceInstanceProvisionedResourcesOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::list_service_instance_provisioned_resources::ListServiceInstanceProvisionedResourcesError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.list_service_instance_provisioned_resources();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `ListServiceInstanceProvisionedResources`.
///
/// <p>List provisioned resources for a service instance with details.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ListServiceInstanceProvisionedResourcesFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::list_service_instance_provisioned_resources::builders::ListServiceInstanceProvisionedResourcesInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::list_service_instance_provisioned_resources::ListServiceInstanceProvisionedResourcesOutput,
        crate::operation::list_service_instance_provisioned_resources::ListServiceInstanceProvisionedResourcesError,
    > for ListServiceInstanceProvisionedResourcesFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::list_service_instance_provisioned_resources::ListServiceInstanceProvisionedResourcesOutput,
            crate::operation::list_service_instance_provisioned_resources::ListServiceInstanceProvisionedResourcesError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl ListServiceInstanceProvisionedResourcesFluentBuilder {
    /// Creates a new `ListServiceInstanceProvisionedResources`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the ListServiceInstanceProvisionedResources as a reference.
    pub fn as_input(
        &self,
    ) -> &crate::operation::list_service_instance_provisioned_resources::builders::ListServiceInstanceProvisionedResourcesInputBuilder {
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
        crate::operation::list_service_instance_provisioned_resources::ListServiceInstanceProvisionedResourcesOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::list_service_instance_provisioned_resources::ListServiceInstanceProvisionedResourcesError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self.inner.build().map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        let runtime_plugins =
            crate::operation::list_service_instance_provisioned_resources::ListServiceInstanceProvisionedResources::operation_runtime_plugins(
                self.handle.runtime_plugins.clone(),
                &self.handle.conf,
                self.config_override,
            );
        crate::operation::list_service_instance_provisioned_resources::ListServiceInstanceProvisionedResources::orchestrate(&runtime_plugins, input)
            .await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent.
    // TODO(enableNewSmithyRuntimeCleanup): Remove `async` and `Result` once we switch to orchestrator
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::orchestrator::CustomizableOperation<
            crate::operation::list_service_instance_provisioned_resources::ListServiceInstanceProvisionedResourcesOutput,
            crate::operation::list_service_instance_provisioned_resources::ListServiceInstanceProvisionedResourcesError,
            Self,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::list_service_instance_provisioned_resources::ListServiceInstanceProvisionedResourcesError,
        >,
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
    /// Paginators are used by calling [`send().await`](crate::operation::list_service_instance_provisioned_resources::paginator::ListServiceInstanceProvisionedResourcesPaginator::send) which returns a `Stream`.
    pub fn into_paginator(
        self,
    ) -> crate::operation::list_service_instance_provisioned_resources::paginator::ListServiceInstanceProvisionedResourcesPaginator {
        crate::operation::list_service_instance_provisioned_resources::paginator::ListServiceInstanceProvisionedResourcesPaginator::new(
            self.handle,
            self.inner,
        )
    }
    /// <p>The name of the service that <code>serviceInstanceName</code> is associated to.</p>
    pub fn service_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.service_name(input.into());
        self
    }
    /// <p>The name of the service that <code>serviceInstanceName</code> is associated to.</p>
    pub fn set_service_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_service_name(input);
        self
    }
    /// <p>The name of the service that <code>serviceInstanceName</code> is associated to.</p>
    pub fn get_service_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_service_name()
    }
    /// <p>The name of the service instance whose provisioned resources you want.</p>
    pub fn service_instance_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.service_instance_name(input.into());
        self
    }
    /// <p>The name of the service instance whose provisioned resources you want.</p>
    pub fn set_service_instance_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_service_instance_name(input);
        self
    }
    /// <p>The name of the service instance whose provisioned resources you want.</p>
    pub fn get_service_instance_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_service_instance_name()
    }
    /// <p>A token that indicates the location of the next provisioned resource in the array of provisioned resources, after the list of provisioned resources that was previously requested.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>A token that indicates the location of the next provisioned resource in the array of provisioned resources, after the list of provisioned resources that was previously requested.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
    /// <p>A token that indicates the location of the next provisioned resource in the array of provisioned resources, after the list of provisioned resources that was previously requested.</p>
    pub fn get_next_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_next_token()
    }
}
