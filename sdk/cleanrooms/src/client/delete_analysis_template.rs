// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteAnalysisTemplate`](crate::operation::delete_analysis_template::builders::DeleteAnalysisTemplateFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`membership_identifier(impl Into<String>)`](crate::operation::delete_analysis_template::builders::DeleteAnalysisTemplateFluentBuilder::membership_identifier) / [`set_membership_identifier(Option<String>)`](crate::operation::delete_analysis_template::builders::DeleteAnalysisTemplateFluentBuilder::set_membership_identifier): <p>The identifier for a membership resource.</p>
    ///   - [`analysis_template_identifier(impl Into<String>)`](crate::operation::delete_analysis_template::builders::DeleteAnalysisTemplateFluentBuilder::analysis_template_identifier) / [`set_analysis_template_identifier(Option<String>)`](crate::operation::delete_analysis_template::builders::DeleteAnalysisTemplateFluentBuilder::set_analysis_template_identifier): <p>The identifier for the analysis template resource.</p>
    /// - On success, responds with [`DeleteAnalysisTemplateOutput`](crate::operation::delete_analysis_template::DeleteAnalysisTemplateOutput)
    /// - On failure, responds with [`SdkError<DeleteAnalysisTemplateError>`](crate::operation::delete_analysis_template::DeleteAnalysisTemplateError)
    pub fn delete_analysis_template(&self) -> crate::operation::delete_analysis_template::builders::DeleteAnalysisTemplateFluentBuilder {
        crate::operation::delete_analysis_template::builders::DeleteAnalysisTemplateFluentBuilder::new(self.handle.clone())
    }
}
