// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The IP endpoint and protocol for calling.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct IpDialIn {
    /// <p>The IP address.</p>
    pub endpoint: ::std::option::Option<::std::string::String>,
    /// <p>The protocol, including SIP, SIPS, and H323.</p>
    pub comms_protocol: ::std::option::Option<crate::types::CommsProtocol>,
}
impl IpDialIn {
    /// <p>The IP address.</p>
    pub fn endpoint(&self) -> ::std::option::Option<&str> {
        self.endpoint.as_deref()
    }
    /// <p>The protocol, including SIP, SIPS, and H323.</p>
    pub fn comms_protocol(&self) -> ::std::option::Option<&crate::types::CommsProtocol> {
        self.comms_protocol.as_ref()
    }
}
impl IpDialIn {
    /// Creates a new builder-style object to manufacture [`IpDialIn`](crate::types::IpDialIn).
    pub fn builder() -> crate::types::builders::IpDialInBuilder {
        crate::types::builders::IpDialInBuilder::default()
    }
}

/// A builder for [`IpDialIn`](crate::types::IpDialIn).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct IpDialInBuilder {
    pub(crate) endpoint: ::std::option::Option<::std::string::String>,
    pub(crate) comms_protocol: ::std::option::Option<crate::types::CommsProtocol>,
}
impl IpDialInBuilder {
    /// <p>The IP address.</p>
    pub fn endpoint(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.endpoint = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The IP address.</p>
    pub fn set_endpoint(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.endpoint = input;
        self
    }
    /// <p>The IP address.</p>
    pub fn get_endpoint(&self) -> &::std::option::Option<::std::string::String> {
        &self.endpoint
    }
    /// <p>The protocol, including SIP, SIPS, and H323.</p>
    pub fn comms_protocol(mut self, input: crate::types::CommsProtocol) -> Self {
        self.comms_protocol = ::std::option::Option::Some(input);
        self
    }
    /// <p>The protocol, including SIP, SIPS, and H323.</p>
    pub fn set_comms_protocol(mut self, input: ::std::option::Option<crate::types::CommsProtocol>) -> Self {
        self.comms_protocol = input;
        self
    }
    /// <p>The protocol, including SIP, SIPS, and H323.</p>
    pub fn get_comms_protocol(&self) -> &::std::option::Option<crate::types::CommsProtocol> {
        &self.comms_protocol
    }
    /// Consumes the builder and constructs a [`IpDialIn`](crate::types::IpDialIn).
    pub fn build(self) -> crate::types::IpDialIn {
        crate::types::IpDialIn {
            endpoint: self.endpoint,
            comms_protocol: self.comms_protocol,
        }
    }
}
