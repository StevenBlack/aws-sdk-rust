// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct CreateLakeFormationOptInInput {
    /// <p>The Lake Formation principal. Supported principals are IAM users or IAM roles.</p>
    pub principal: ::std::option::Option<crate::types::DataLakePrincipal>,
    /// <p>A structure for the resource.</p>
    pub resource: ::std::option::Option<crate::types::Resource>,
}
impl CreateLakeFormationOptInInput {
    /// <p>The Lake Formation principal. Supported principals are IAM users or IAM roles.</p>
    pub fn principal(&self) -> ::std::option::Option<&crate::types::DataLakePrincipal> {
        self.principal.as_ref()
    }
    /// <p>A structure for the resource.</p>
    pub fn resource(&self) -> ::std::option::Option<&crate::types::Resource> {
        self.resource.as_ref()
    }
}
impl CreateLakeFormationOptInInput {
    /// Creates a new builder-style object to manufacture [`CreateLakeFormationOptInInput`](crate::operation::create_lake_formation_opt_in::CreateLakeFormationOptInInput).
    pub fn builder() -> crate::operation::create_lake_formation_opt_in::builders::CreateLakeFormationOptInInputBuilder {
        crate::operation::create_lake_formation_opt_in::builders::CreateLakeFormationOptInInputBuilder::default()
    }
}

/// A builder for [`CreateLakeFormationOptInInput`](crate::operation::create_lake_formation_opt_in::CreateLakeFormationOptInInput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct CreateLakeFormationOptInInputBuilder {
    pub(crate) principal: ::std::option::Option<crate::types::DataLakePrincipal>,
    pub(crate) resource: ::std::option::Option<crate::types::Resource>,
}
impl CreateLakeFormationOptInInputBuilder {
    /// <p>The Lake Formation principal. Supported principals are IAM users or IAM roles.</p>
    pub fn principal(mut self, input: crate::types::DataLakePrincipal) -> Self {
        self.principal = ::std::option::Option::Some(input);
        self
    }
    /// <p>The Lake Formation principal. Supported principals are IAM users or IAM roles.</p>
    pub fn set_principal(mut self, input: ::std::option::Option<crate::types::DataLakePrincipal>) -> Self {
        self.principal = input;
        self
    }
    /// <p>The Lake Formation principal. Supported principals are IAM users or IAM roles.</p>
    pub fn get_principal(&self) -> &::std::option::Option<crate::types::DataLakePrincipal> {
        &self.principal
    }
    /// <p>A structure for the resource.</p>
    pub fn resource(mut self, input: crate::types::Resource) -> Self {
        self.resource = ::std::option::Option::Some(input);
        self
    }
    /// <p>A structure for the resource.</p>
    pub fn set_resource(mut self, input: ::std::option::Option<crate::types::Resource>) -> Self {
        self.resource = input;
        self
    }
    /// <p>A structure for the resource.</p>
    pub fn get_resource(&self) -> &::std::option::Option<crate::types::Resource> {
        &self.resource
    }
    /// Consumes the builder and constructs a [`CreateLakeFormationOptInInput`](crate::operation::create_lake_formation_opt_in::CreateLakeFormationOptInInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::create_lake_formation_opt_in::CreateLakeFormationOptInInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::create_lake_formation_opt_in::CreateLakeFormationOptInInput {
            principal: self.principal,
            resource: self.resource,
        })
    }
}
