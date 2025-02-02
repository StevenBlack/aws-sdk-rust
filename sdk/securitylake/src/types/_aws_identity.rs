// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The AWS identity.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct AwsIdentity {
    /// <p>The AWS identity principal.</p>
    pub principal: ::std::option::Option<::std::string::String>,
    /// <p>The external ID used to estalish trust relationship with the AWS identity.</p>
    pub external_id: ::std::option::Option<::std::string::String>,
}
impl AwsIdentity {
    /// <p>The AWS identity principal.</p>
    pub fn principal(&self) -> ::std::option::Option<&str> {
        self.principal.as_deref()
    }
    /// <p>The external ID used to estalish trust relationship with the AWS identity.</p>
    pub fn external_id(&self) -> ::std::option::Option<&str> {
        self.external_id.as_deref()
    }
}
impl AwsIdentity {
    /// Creates a new builder-style object to manufacture [`AwsIdentity`](crate::types::AwsIdentity).
    pub fn builder() -> crate::types::builders::AwsIdentityBuilder {
        crate::types::builders::AwsIdentityBuilder::default()
    }
}

/// A builder for [`AwsIdentity`](crate::types::AwsIdentity).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct AwsIdentityBuilder {
    pub(crate) principal: ::std::option::Option<::std::string::String>,
    pub(crate) external_id: ::std::option::Option<::std::string::String>,
}
impl AwsIdentityBuilder {
    /// <p>The AWS identity principal.</p>
    pub fn principal(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.principal = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The AWS identity principal.</p>
    pub fn set_principal(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.principal = input;
        self
    }
    /// <p>The AWS identity principal.</p>
    pub fn get_principal(&self) -> &::std::option::Option<::std::string::String> {
        &self.principal
    }
    /// <p>The external ID used to estalish trust relationship with the AWS identity.</p>
    pub fn external_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.external_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The external ID used to estalish trust relationship with the AWS identity.</p>
    pub fn set_external_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.external_id = input;
        self
    }
    /// <p>The external ID used to estalish trust relationship with the AWS identity.</p>
    pub fn get_external_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.external_id
    }
    /// Consumes the builder and constructs a [`AwsIdentity`](crate::types::AwsIdentity).
    pub fn build(self) -> crate::types::AwsIdentity {
        crate::types::AwsIdentity {
            principal: self.principal,
            external_id: self.external_id,
        }
    }
}
