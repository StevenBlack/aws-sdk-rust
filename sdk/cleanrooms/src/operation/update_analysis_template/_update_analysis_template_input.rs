// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct UpdateAnalysisTemplateInput {
    /// <p>The identifier for a membership resource.</p>
    pub membership_identifier: ::std::option::Option<::std::string::String>,
    /// <p>The identifier for the analysis template resource.</p>
    pub analysis_template_identifier: ::std::option::Option<::std::string::String>,
    /// <p>A new description for the analysis template.</p>
    pub description: ::std::option::Option<::std::string::String>,
}
impl UpdateAnalysisTemplateInput {
    /// <p>The identifier for a membership resource.</p>
    pub fn membership_identifier(&self) -> ::std::option::Option<&str> {
        self.membership_identifier.as_deref()
    }
    /// <p>The identifier for the analysis template resource.</p>
    pub fn analysis_template_identifier(&self) -> ::std::option::Option<&str> {
        self.analysis_template_identifier.as_deref()
    }
    /// <p>A new description for the analysis template.</p>
    pub fn description(&self) -> ::std::option::Option<&str> {
        self.description.as_deref()
    }
}
impl UpdateAnalysisTemplateInput {
    /// Creates a new builder-style object to manufacture [`UpdateAnalysisTemplateInput`](crate::operation::update_analysis_template::UpdateAnalysisTemplateInput).
    pub fn builder() -> crate::operation::update_analysis_template::builders::UpdateAnalysisTemplateInputBuilder {
        crate::operation::update_analysis_template::builders::UpdateAnalysisTemplateInputBuilder::default()
    }
}

/// A builder for [`UpdateAnalysisTemplateInput`](crate::operation::update_analysis_template::UpdateAnalysisTemplateInput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct UpdateAnalysisTemplateInputBuilder {
    pub(crate) membership_identifier: ::std::option::Option<::std::string::String>,
    pub(crate) analysis_template_identifier: ::std::option::Option<::std::string::String>,
    pub(crate) description: ::std::option::Option<::std::string::String>,
}
impl UpdateAnalysisTemplateInputBuilder {
    /// <p>The identifier for a membership resource.</p>
    pub fn membership_identifier(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.membership_identifier = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The identifier for a membership resource.</p>
    pub fn set_membership_identifier(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.membership_identifier = input;
        self
    }
    /// <p>The identifier for a membership resource.</p>
    pub fn get_membership_identifier(&self) -> &::std::option::Option<::std::string::String> {
        &self.membership_identifier
    }
    /// <p>The identifier for the analysis template resource.</p>
    pub fn analysis_template_identifier(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.analysis_template_identifier = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The identifier for the analysis template resource.</p>
    pub fn set_analysis_template_identifier(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.analysis_template_identifier = input;
        self
    }
    /// <p>The identifier for the analysis template resource.</p>
    pub fn get_analysis_template_identifier(&self) -> &::std::option::Option<::std::string::String> {
        &self.analysis_template_identifier
    }
    /// <p>A new description for the analysis template.</p>
    pub fn description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.description = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A new description for the analysis template.</p>
    pub fn set_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.description = input;
        self
    }
    /// <p>A new description for the analysis template.</p>
    pub fn get_description(&self) -> &::std::option::Option<::std::string::String> {
        &self.description
    }
    /// Consumes the builder and constructs a [`UpdateAnalysisTemplateInput`](crate::operation::update_analysis_template::UpdateAnalysisTemplateInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<crate::operation::update_analysis_template::UpdateAnalysisTemplateInput, ::aws_smithy_http::operation::error::BuildError>
    {
        ::std::result::Result::Ok(crate::operation::update_analysis_template::UpdateAnalysisTemplateInput {
            membership_identifier: self.membership_identifier,
            analysis_template_identifier: self.analysis_template_identifier,
            description: self.description,
        })
    }
}
