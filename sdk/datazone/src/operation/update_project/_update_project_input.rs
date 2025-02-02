// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq)]
pub struct UpdateProjectInput {
    /// <p>The identifier of the Amazon DataZone domain in which a project is to be updated.</p>
    pub domain_identifier: ::std::option::Option<::std::string::String>,
    /// <p>The identifier of the project that is to be updated.</p>
    pub identifier: ::std::option::Option<::std::string::String>,
    /// <p>The name to be updated as part of the <code>UpdateProject</code> action.</p>
    pub name: ::std::option::Option<::std::string::String>,
    /// <p>The description to be updated as part of the <code>UpdateProject</code> action.</p>
    pub description: ::std::option::Option<::std::string::String>,
    /// <p>The glossary terms to be updated as part of the <code>UpdateProject</code> action.</p>
    pub glossary_terms: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
impl UpdateProjectInput {
    /// <p>The identifier of the Amazon DataZone domain in which a project is to be updated.</p>
    pub fn domain_identifier(&self) -> ::std::option::Option<&str> {
        self.domain_identifier.as_deref()
    }
    /// <p>The identifier of the project that is to be updated.</p>
    pub fn identifier(&self) -> ::std::option::Option<&str> {
        self.identifier.as_deref()
    }
    /// <p>The name to be updated as part of the <code>UpdateProject</code> action.</p>
    pub fn name(&self) -> ::std::option::Option<&str> {
        self.name.as_deref()
    }
    /// <p>The description to be updated as part of the <code>UpdateProject</code> action.</p>
    pub fn description(&self) -> ::std::option::Option<&str> {
        self.description.as_deref()
    }
    /// <p>The glossary terms to be updated as part of the <code>UpdateProject</code> action.</p>
    pub fn glossary_terms(&self) -> ::std::option::Option<&[::std::string::String]> {
        self.glossary_terms.as_deref()
    }
}
impl ::std::fmt::Debug for UpdateProjectInput {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("UpdateProjectInput");
        formatter.field("domain_identifier", &self.domain_identifier);
        formatter.field("identifier", &self.identifier);
        formatter.field("name", &"*** Sensitive Data Redacted ***");
        formatter.field("description", &"*** Sensitive Data Redacted ***");
        formatter.field("glossary_terms", &self.glossary_terms);
        formatter.finish()
    }
}
impl UpdateProjectInput {
    /// Creates a new builder-style object to manufacture [`UpdateProjectInput`](crate::operation::update_project::UpdateProjectInput).
    pub fn builder() -> crate::operation::update_project::builders::UpdateProjectInputBuilder {
        crate::operation::update_project::builders::UpdateProjectInputBuilder::default()
    }
}

/// A builder for [`UpdateProjectInput`](crate::operation::update_project::UpdateProjectInput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default)]
pub struct UpdateProjectInputBuilder {
    pub(crate) domain_identifier: ::std::option::Option<::std::string::String>,
    pub(crate) identifier: ::std::option::Option<::std::string::String>,
    pub(crate) name: ::std::option::Option<::std::string::String>,
    pub(crate) description: ::std::option::Option<::std::string::String>,
    pub(crate) glossary_terms: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
impl UpdateProjectInputBuilder {
    /// <p>The identifier of the Amazon DataZone domain in which a project is to be updated.</p>
    pub fn domain_identifier(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.domain_identifier = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The identifier of the Amazon DataZone domain in which a project is to be updated.</p>
    pub fn set_domain_identifier(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.domain_identifier = input;
        self
    }
    /// <p>The identifier of the Amazon DataZone domain in which a project is to be updated.</p>
    pub fn get_domain_identifier(&self) -> &::std::option::Option<::std::string::String> {
        &self.domain_identifier
    }
    /// <p>The identifier of the project that is to be updated.</p>
    pub fn identifier(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.identifier = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The identifier of the project that is to be updated.</p>
    pub fn set_identifier(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.identifier = input;
        self
    }
    /// <p>The identifier of the project that is to be updated.</p>
    pub fn get_identifier(&self) -> &::std::option::Option<::std::string::String> {
        &self.identifier
    }
    /// <p>The name to be updated as part of the <code>UpdateProject</code> action.</p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name to be updated as part of the <code>UpdateProject</code> action.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.name = input;
        self
    }
    /// <p>The name to be updated as part of the <code>UpdateProject</code> action.</p>
    pub fn get_name(&self) -> &::std::option::Option<::std::string::String> {
        &self.name
    }
    /// <p>The description to be updated as part of the <code>UpdateProject</code> action.</p>
    pub fn description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.description = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The description to be updated as part of the <code>UpdateProject</code> action.</p>
    pub fn set_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.description = input;
        self
    }
    /// <p>The description to be updated as part of the <code>UpdateProject</code> action.</p>
    pub fn get_description(&self) -> &::std::option::Option<::std::string::String> {
        &self.description
    }
    /// Appends an item to `glossary_terms`.
    ///
    /// To override the contents of this collection use [`set_glossary_terms`](Self::set_glossary_terms).
    ///
    /// <p>The glossary terms to be updated as part of the <code>UpdateProject</code> action.</p>
    pub fn glossary_terms(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        let mut v = self.glossary_terms.unwrap_or_default();
        v.push(input.into());
        self.glossary_terms = ::std::option::Option::Some(v);
        self
    }
    /// <p>The glossary terms to be updated as part of the <code>UpdateProject</code> action.</p>
    pub fn set_glossary_terms(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.glossary_terms = input;
        self
    }
    /// <p>The glossary terms to be updated as part of the <code>UpdateProject</code> action.</p>
    pub fn get_glossary_terms(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        &self.glossary_terms
    }
    /// Consumes the builder and constructs a [`UpdateProjectInput`](crate::operation::update_project::UpdateProjectInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<crate::operation::update_project::UpdateProjectInput, ::aws_smithy_http::operation::error::BuildError> {
        ::std::result::Result::Ok(crate::operation::update_project::UpdateProjectInput {
            domain_identifier: self.domain_identifier,
            identifier: self.identifier,
            name: self.name,
            description: self.description,
            glossary_terms: self.glossary_terms,
        })
    }
}
impl ::std::fmt::Debug for UpdateProjectInputBuilder {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("UpdateProjectInputBuilder");
        formatter.field("domain_identifier", &self.domain_identifier);
        formatter.field("identifier", &self.identifier);
        formatter.field("name", &"*** Sensitive Data Redacted ***");
        formatter.field("description", &"*** Sensitive Data Redacted ***");
        formatter.field("glossary_terms", &self.glossary_terms);
        formatter.finish()
    }
}
