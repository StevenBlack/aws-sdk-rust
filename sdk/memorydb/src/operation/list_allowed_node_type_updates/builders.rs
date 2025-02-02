// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::list_allowed_node_type_updates::_list_allowed_node_type_updates_output::ListAllowedNodeTypeUpdatesOutputBuilder;

pub use crate::operation::list_allowed_node_type_updates::_list_allowed_node_type_updates_input::ListAllowedNodeTypeUpdatesInputBuilder;

impl ListAllowedNodeTypeUpdatesInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::list_allowed_node_type_updates::ListAllowedNodeTypeUpdatesOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::list_allowed_node_type_updates::ListAllowedNodeTypeUpdatesError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.list_allowed_node_type_updates();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `ListAllowedNodeTypeUpdates`.
///
/// <p>Lists all available node types that you can scale to from your cluster's current node type. When you use the UpdateCluster operation to scale your cluster, the value of the NodeType parameter must be one of the node types returned by this operation.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ListAllowedNodeTypeUpdatesFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::list_allowed_node_type_updates::builders::ListAllowedNodeTypeUpdatesInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::list_allowed_node_type_updates::ListAllowedNodeTypeUpdatesOutput,
        crate::operation::list_allowed_node_type_updates::ListAllowedNodeTypeUpdatesError,
    > for ListAllowedNodeTypeUpdatesFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::list_allowed_node_type_updates::ListAllowedNodeTypeUpdatesOutput,
            crate::operation::list_allowed_node_type_updates::ListAllowedNodeTypeUpdatesError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl ListAllowedNodeTypeUpdatesFluentBuilder {
    /// Creates a new `ListAllowedNodeTypeUpdates`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the ListAllowedNodeTypeUpdates as a reference.
    pub fn as_input(&self) -> &crate::operation::list_allowed_node_type_updates::builders::ListAllowedNodeTypeUpdatesInputBuilder {
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
        crate::operation::list_allowed_node_type_updates::ListAllowedNodeTypeUpdatesOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::list_allowed_node_type_updates::ListAllowedNodeTypeUpdatesError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self.inner.build().map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::list_allowed_node_type_updates::ListAllowedNodeTypeUpdates::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::list_allowed_node_type_updates::ListAllowedNodeTypeUpdates::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent.
    // TODO(enableNewSmithyRuntimeCleanup): Remove `async` and `Result` once we switch to orchestrator
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::orchestrator::CustomizableOperation<
            crate::operation::list_allowed_node_type_updates::ListAllowedNodeTypeUpdatesOutput,
            crate::operation::list_allowed_node_type_updates::ListAllowedNodeTypeUpdatesError,
            Self,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::list_allowed_node_type_updates::ListAllowedNodeTypeUpdatesError>,
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
    /// <p>The name of the cluster you want to scale. MemoryDB uses the cluster name to identify the current node type being used by this cluster, and from that to create a list of node types you can scale up to.</p>
    pub fn cluster_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.cluster_name(input.into());
        self
    }
    /// <p>The name of the cluster you want to scale. MemoryDB uses the cluster name to identify the current node type being used by this cluster, and from that to create a list of node types you can scale up to.</p>
    pub fn set_cluster_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_cluster_name(input);
        self
    }
    /// <p>The name of the cluster you want to scale. MemoryDB uses the cluster name to identify the current node type being used by this cluster, and from that to create a list of node types you can scale up to.</p>
    pub fn get_cluster_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_cluster_name()
    }
}
