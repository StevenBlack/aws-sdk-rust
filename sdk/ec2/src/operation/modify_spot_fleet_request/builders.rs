// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::modify_spot_fleet_request::_modify_spot_fleet_request_output::ModifySpotFleetRequestOutputBuilder;

pub use crate::operation::modify_spot_fleet_request::_modify_spot_fleet_request_input::ModifySpotFleetRequestInputBuilder;

impl ModifySpotFleetRequestInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::modify_spot_fleet_request::ModifySpotFleetRequestOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::modify_spot_fleet_request::ModifySpotFleetRequestError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.modify_spot_fleet_request();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `ModifySpotFleetRequest`.
///
/// <p>Modifies the specified Spot Fleet request.</p>
/// <p>You can only modify a Spot Fleet request of type <code>maintain</code>.</p>
/// <p>While the Spot Fleet request is being modified, it is in the <code>modifying</code> state.</p>
/// <p>To scale up your Spot Fleet, increase its target capacity. The Spot Fleet launches the additional Spot Instances according to the allocation strategy for the Spot Fleet request. If the allocation strategy is <code>lowestPrice</code>, the Spot Fleet launches instances using the Spot Instance pool with the lowest price. If the allocation strategy is <code>diversified</code>, the Spot Fleet distributes the instances across the Spot Instance pools. If the allocation strategy is <code>capacityOptimized</code>, Spot Fleet launches instances from Spot Instance pools with optimal capacity for the number of instances that are launching.</p>
/// <p>To scale down your Spot Fleet, decrease its target capacity. First, the Spot Fleet cancels any open requests that exceed the new target capacity. You can request that the Spot Fleet terminate Spot Instances until the size of the fleet no longer exceeds the new target capacity. If the allocation strategy is <code>lowestPrice</code>, the Spot Fleet terminates the instances with the highest price per unit. If the allocation strategy is <code>capacityOptimized</code>, the Spot Fleet terminates the instances in the Spot Instance pools that have the least available Spot Instance capacity. If the allocation strategy is <code>diversified</code>, the Spot Fleet terminates instances across the Spot Instance pools. Alternatively, you can request that the Spot Fleet keep the fleet at its current size, but not replace any Spot Instances that are interrupted or that you terminate manually.</p>
/// <p>If you are finished with your Spot Fleet for now, but will use it again later, you can set the target capacity to 0.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ModifySpotFleetRequestFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::modify_spot_fleet_request::builders::ModifySpotFleetRequestInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::modify_spot_fleet_request::ModifySpotFleetRequestOutput,
        crate::operation::modify_spot_fleet_request::ModifySpotFleetRequestError,
    > for ModifySpotFleetRequestFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::modify_spot_fleet_request::ModifySpotFleetRequestOutput,
            crate::operation::modify_spot_fleet_request::ModifySpotFleetRequestError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl ModifySpotFleetRequestFluentBuilder {
    /// Creates a new `ModifySpotFleetRequest`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the ModifySpotFleetRequest as a reference.
    pub fn as_input(&self) -> &crate::operation::modify_spot_fleet_request::builders::ModifySpotFleetRequestInputBuilder {
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
        crate::operation::modify_spot_fleet_request::ModifySpotFleetRequestOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::modify_spot_fleet_request::ModifySpotFleetRequestError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self.inner.build().map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::modify_spot_fleet_request::ModifySpotFleetRequest::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::modify_spot_fleet_request::ModifySpotFleetRequest::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent.
    // TODO(enableNewSmithyRuntimeCleanup): Remove `async` and `Result` once we switch to orchestrator
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::orchestrator::CustomizableOperation<
            crate::operation::modify_spot_fleet_request::ModifySpotFleetRequestOutput,
            crate::operation::modify_spot_fleet_request::ModifySpotFleetRequestError,
            Self,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::modify_spot_fleet_request::ModifySpotFleetRequestError>,
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
    /// <p>Indicates whether running instances should be terminated if the target capacity of the Spot Fleet request is decreased below the current size of the Spot Fleet.</p>
    /// <p>Supported only for fleets of type <code>maintain</code>.</p>
    pub fn excess_capacity_termination_policy(mut self, input: crate::types::ExcessCapacityTerminationPolicy) -> Self {
        self.inner = self.inner.excess_capacity_termination_policy(input);
        self
    }
    /// <p>Indicates whether running instances should be terminated if the target capacity of the Spot Fleet request is decreased below the current size of the Spot Fleet.</p>
    /// <p>Supported only for fleets of type <code>maintain</code>.</p>
    pub fn set_excess_capacity_termination_policy(mut self, input: ::std::option::Option<crate::types::ExcessCapacityTerminationPolicy>) -> Self {
        self.inner = self.inner.set_excess_capacity_termination_policy(input);
        self
    }
    /// <p>Indicates whether running instances should be terminated if the target capacity of the Spot Fleet request is decreased below the current size of the Spot Fleet.</p>
    /// <p>Supported only for fleets of type <code>maintain</code>.</p>
    pub fn get_excess_capacity_termination_policy(&self) -> &::std::option::Option<crate::types::ExcessCapacityTerminationPolicy> {
        self.inner.get_excess_capacity_termination_policy()
    }
    /// Appends an item to `LaunchTemplateConfigs`.
    ///
    /// To override the contents of this collection use [`set_launch_template_configs`](Self::set_launch_template_configs).
    ///
    /// <p>The launch template and overrides. You can only use this parameter if you specified a launch template (<code>LaunchTemplateConfigs</code>) in your Spot Fleet request. If you specified <code>LaunchSpecifications</code> in your Spot Fleet request, then omit this parameter.</p>
    pub fn launch_template_configs(mut self, input: crate::types::LaunchTemplateConfig) -> Self {
        self.inner = self.inner.launch_template_configs(input);
        self
    }
    /// <p>The launch template and overrides. You can only use this parameter if you specified a launch template (<code>LaunchTemplateConfigs</code>) in your Spot Fleet request. If you specified <code>LaunchSpecifications</code> in your Spot Fleet request, then omit this parameter.</p>
    pub fn set_launch_template_configs(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::LaunchTemplateConfig>>) -> Self {
        self.inner = self.inner.set_launch_template_configs(input);
        self
    }
    /// <p>The launch template and overrides. You can only use this parameter if you specified a launch template (<code>LaunchTemplateConfigs</code>) in your Spot Fleet request. If you specified <code>LaunchSpecifications</code> in your Spot Fleet request, then omit this parameter.</p>
    pub fn get_launch_template_configs(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::LaunchTemplateConfig>> {
        self.inner.get_launch_template_configs()
    }
    /// <p>The ID of the Spot Fleet request.</p>
    pub fn spot_fleet_request_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.spot_fleet_request_id(input.into());
        self
    }
    /// <p>The ID of the Spot Fleet request.</p>
    pub fn set_spot_fleet_request_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_spot_fleet_request_id(input);
        self
    }
    /// <p>The ID of the Spot Fleet request.</p>
    pub fn get_spot_fleet_request_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_spot_fleet_request_id()
    }
    /// <p>The size of the fleet.</p>
    pub fn target_capacity(mut self, input: i32) -> Self {
        self.inner = self.inner.target_capacity(input);
        self
    }
    /// <p>The size of the fleet.</p>
    pub fn set_target_capacity(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_target_capacity(input);
        self
    }
    /// <p>The size of the fleet.</p>
    pub fn get_target_capacity(&self) -> &::std::option::Option<i32> {
        self.inner.get_target_capacity()
    }
    /// <p>The number of On-Demand Instances in the fleet.</p>
    pub fn on_demand_target_capacity(mut self, input: i32) -> Self {
        self.inner = self.inner.on_demand_target_capacity(input);
        self
    }
    /// <p>The number of On-Demand Instances in the fleet.</p>
    pub fn set_on_demand_target_capacity(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_on_demand_target_capacity(input);
        self
    }
    /// <p>The number of On-Demand Instances in the fleet.</p>
    pub fn get_on_demand_target_capacity(&self) -> &::std::option::Option<i32> {
        self.inner.get_on_demand_target_capacity()
    }
    /// <p>Reserved.</p>
    pub fn context(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.context(input.into());
        self
    }
    /// <p>Reserved.</p>
    pub fn set_context(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_context(input);
        self
    }
    /// <p>Reserved.</p>
    pub fn get_context(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_context()
    }
}
