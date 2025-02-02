// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Contains information about a hardware security module (HSM) in an AWS CloudHSM cluster.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct Hsm {
    /// <p>The Availability Zone that contains the HSM.</p>
    pub availability_zone: ::std::option::Option<::std::string::String>,
    /// <p>The identifier (ID) of the cluster that contains the HSM.</p>
    pub cluster_id: ::std::option::Option<::std::string::String>,
    /// <p>The subnet that contains the HSM's elastic network interface (ENI).</p>
    pub subnet_id: ::std::option::Option<::std::string::String>,
    /// <p>The identifier (ID) of the HSM's elastic network interface (ENI).</p>
    pub eni_id: ::std::option::Option<::std::string::String>,
    /// <p>The IP address of the HSM's elastic network interface (ENI).</p>
    pub eni_ip: ::std::option::Option<::std::string::String>,
    /// <p>The HSM's identifier (ID).</p>
    pub hsm_id: ::std::option::Option<::std::string::String>,
    /// <p>The HSM's state.</p>
    pub state: ::std::option::Option<crate::types::HsmState>,
    /// <p>A description of the HSM's state.</p>
    pub state_message: ::std::option::Option<::std::string::String>,
}
impl Hsm {
    /// <p>The Availability Zone that contains the HSM.</p>
    pub fn availability_zone(&self) -> ::std::option::Option<&str> {
        self.availability_zone.as_deref()
    }
    /// <p>The identifier (ID) of the cluster that contains the HSM.</p>
    pub fn cluster_id(&self) -> ::std::option::Option<&str> {
        self.cluster_id.as_deref()
    }
    /// <p>The subnet that contains the HSM's elastic network interface (ENI).</p>
    pub fn subnet_id(&self) -> ::std::option::Option<&str> {
        self.subnet_id.as_deref()
    }
    /// <p>The identifier (ID) of the HSM's elastic network interface (ENI).</p>
    pub fn eni_id(&self) -> ::std::option::Option<&str> {
        self.eni_id.as_deref()
    }
    /// <p>The IP address of the HSM's elastic network interface (ENI).</p>
    pub fn eni_ip(&self) -> ::std::option::Option<&str> {
        self.eni_ip.as_deref()
    }
    /// <p>The HSM's identifier (ID).</p>
    pub fn hsm_id(&self) -> ::std::option::Option<&str> {
        self.hsm_id.as_deref()
    }
    /// <p>The HSM's state.</p>
    pub fn state(&self) -> ::std::option::Option<&crate::types::HsmState> {
        self.state.as_ref()
    }
    /// <p>A description of the HSM's state.</p>
    pub fn state_message(&self) -> ::std::option::Option<&str> {
        self.state_message.as_deref()
    }
}
impl Hsm {
    /// Creates a new builder-style object to manufacture [`Hsm`](crate::types::Hsm).
    pub fn builder() -> crate::types::builders::HsmBuilder {
        crate::types::builders::HsmBuilder::default()
    }
}

/// A builder for [`Hsm`](crate::types::Hsm).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct HsmBuilder {
    pub(crate) availability_zone: ::std::option::Option<::std::string::String>,
    pub(crate) cluster_id: ::std::option::Option<::std::string::String>,
    pub(crate) subnet_id: ::std::option::Option<::std::string::String>,
    pub(crate) eni_id: ::std::option::Option<::std::string::String>,
    pub(crate) eni_ip: ::std::option::Option<::std::string::String>,
    pub(crate) hsm_id: ::std::option::Option<::std::string::String>,
    pub(crate) state: ::std::option::Option<crate::types::HsmState>,
    pub(crate) state_message: ::std::option::Option<::std::string::String>,
}
impl HsmBuilder {
    /// <p>The Availability Zone that contains the HSM.</p>
    pub fn availability_zone(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.availability_zone = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Availability Zone that contains the HSM.</p>
    pub fn set_availability_zone(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.availability_zone = input;
        self
    }
    /// <p>The Availability Zone that contains the HSM.</p>
    pub fn get_availability_zone(&self) -> &::std::option::Option<::std::string::String> {
        &self.availability_zone
    }
    /// <p>The identifier (ID) of the cluster that contains the HSM.</p>
    pub fn cluster_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.cluster_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The identifier (ID) of the cluster that contains the HSM.</p>
    pub fn set_cluster_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.cluster_id = input;
        self
    }
    /// <p>The identifier (ID) of the cluster that contains the HSM.</p>
    pub fn get_cluster_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.cluster_id
    }
    /// <p>The subnet that contains the HSM's elastic network interface (ENI).</p>
    pub fn subnet_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.subnet_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The subnet that contains the HSM's elastic network interface (ENI).</p>
    pub fn set_subnet_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.subnet_id = input;
        self
    }
    /// <p>The subnet that contains the HSM's elastic network interface (ENI).</p>
    pub fn get_subnet_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.subnet_id
    }
    /// <p>The identifier (ID) of the HSM's elastic network interface (ENI).</p>
    pub fn eni_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.eni_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The identifier (ID) of the HSM's elastic network interface (ENI).</p>
    pub fn set_eni_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.eni_id = input;
        self
    }
    /// <p>The identifier (ID) of the HSM's elastic network interface (ENI).</p>
    pub fn get_eni_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.eni_id
    }
    /// <p>The IP address of the HSM's elastic network interface (ENI).</p>
    pub fn eni_ip(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.eni_ip = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The IP address of the HSM's elastic network interface (ENI).</p>
    pub fn set_eni_ip(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.eni_ip = input;
        self
    }
    /// <p>The IP address of the HSM's elastic network interface (ENI).</p>
    pub fn get_eni_ip(&self) -> &::std::option::Option<::std::string::String> {
        &self.eni_ip
    }
    /// <p>The HSM's identifier (ID).</p>
    pub fn hsm_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.hsm_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The HSM's identifier (ID).</p>
    pub fn set_hsm_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.hsm_id = input;
        self
    }
    /// <p>The HSM's identifier (ID).</p>
    pub fn get_hsm_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.hsm_id
    }
    /// <p>The HSM's state.</p>
    pub fn state(mut self, input: crate::types::HsmState) -> Self {
        self.state = ::std::option::Option::Some(input);
        self
    }
    /// <p>The HSM's state.</p>
    pub fn set_state(mut self, input: ::std::option::Option<crate::types::HsmState>) -> Self {
        self.state = input;
        self
    }
    /// <p>The HSM's state.</p>
    pub fn get_state(&self) -> &::std::option::Option<crate::types::HsmState> {
        &self.state
    }
    /// <p>A description of the HSM's state.</p>
    pub fn state_message(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.state_message = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A description of the HSM's state.</p>
    pub fn set_state_message(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.state_message = input;
        self
    }
    /// <p>A description of the HSM's state.</p>
    pub fn get_state_message(&self) -> &::std::option::Option<::std::string::String> {
        &self.state_message
    }
    /// Consumes the builder and constructs a [`Hsm`](crate::types::Hsm).
    pub fn build(self) -> crate::types::Hsm {
        crate::types::Hsm {
            availability_zone: self.availability_zone,
            cluster_id: self.cluster_id,
            subnet_id: self.subnet_id,
            eni_id: self.eni_id,
            eni_ip: self.eni_ip,
            hsm_id: self.hsm_id,
            state: self.state,
            state_message: self.state_message,
        }
    }
}
