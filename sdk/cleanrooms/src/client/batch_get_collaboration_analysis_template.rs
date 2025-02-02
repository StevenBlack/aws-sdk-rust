// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`BatchGetCollaborationAnalysisTemplate`](crate::operation::batch_get_collaboration_analysis_template::builders::BatchGetCollaborationAnalysisTemplateFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`collaboration_identifier(impl Into<String>)`](crate::operation::batch_get_collaboration_analysis_template::builders::BatchGetCollaborationAnalysisTemplateFluentBuilder::collaboration_identifier) / [`set_collaboration_identifier(Option<String>)`](crate::operation::batch_get_collaboration_analysis_template::builders::BatchGetCollaborationAnalysisTemplateFluentBuilder::set_collaboration_identifier): <p>A unique identifier for the collaboration that the analysis templates belong to. Currently accepts collaboration ID.</p>
    ///   - [`analysis_template_arns(impl Into<String>)`](crate::operation::batch_get_collaboration_analysis_template::builders::BatchGetCollaborationAnalysisTemplateFluentBuilder::analysis_template_arns) / [`set_analysis_template_arns(Option<Vec<String>>)`](crate::operation::batch_get_collaboration_analysis_template::builders::BatchGetCollaborationAnalysisTemplateFluentBuilder::set_analysis_template_arns): <p>The Amazon Resource Name (ARN) associated with the analysis template within a collaboration.</p>
    /// - On success, responds with [`BatchGetCollaborationAnalysisTemplateOutput`](crate::operation::batch_get_collaboration_analysis_template::BatchGetCollaborationAnalysisTemplateOutput) with field(s):
    ///   - [`collaboration_analysis_templates(Option<Vec<CollaborationAnalysisTemplate>>)`](crate::operation::batch_get_collaboration_analysis_template::BatchGetCollaborationAnalysisTemplateOutput::collaboration_analysis_templates): <p>The retrieved list of analysis templates within a collaboration.</p>
    ///   - [`errors(Option<Vec<BatchGetCollaborationAnalysisTemplateError>>)`](crate::operation::batch_get_collaboration_analysis_template::BatchGetCollaborationAnalysisTemplateOutput::errors): <p>Error reasons for collaboration analysis templates that could not be retrieved. One error is returned for every collaboration analysis template that could not be retrieved.</p>
    /// - On failure, responds with [`SdkError<BatchGetCollaborationAnalysisTemplateError>`](crate::operation::batch_get_collaboration_analysis_template::BatchGetCollaborationAnalysisTemplateError)
    pub fn batch_get_collaboration_analysis_template(
        &self,
    ) -> crate::operation::batch_get_collaboration_analysis_template::builders::BatchGetCollaborationAnalysisTemplateFluentBuilder {
        crate::operation::batch_get_collaboration_analysis_template::builders::BatchGetCollaborationAnalysisTemplateFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
