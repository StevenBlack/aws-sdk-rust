// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ValidatePipelineDefinition`](crate::operation::validate_pipeline_definition::builders::ValidatePipelineDefinitionFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`pipeline_id(impl Into<String>)`](crate::operation::validate_pipeline_definition::builders::ValidatePipelineDefinitionFluentBuilder::pipeline_id) / [`set_pipeline_id(Option<String>)`](crate::operation::validate_pipeline_definition::builders::ValidatePipelineDefinitionFluentBuilder::set_pipeline_id): <p>The ID of the pipeline.</p>
    ///   - [`pipeline_objects(PipelineObject)`](crate::operation::validate_pipeline_definition::builders::ValidatePipelineDefinitionFluentBuilder::pipeline_objects) / [`set_pipeline_objects(Option<Vec<PipelineObject>>)`](crate::operation::validate_pipeline_definition::builders::ValidatePipelineDefinitionFluentBuilder::set_pipeline_objects): <p>The objects that define the pipeline changes to validate against the pipeline.</p>
    ///   - [`parameter_objects(ParameterObject)`](crate::operation::validate_pipeline_definition::builders::ValidatePipelineDefinitionFluentBuilder::parameter_objects) / [`set_parameter_objects(Option<Vec<ParameterObject>>)`](crate::operation::validate_pipeline_definition::builders::ValidatePipelineDefinitionFluentBuilder::set_parameter_objects): <p>The parameter objects used with the pipeline.</p>
    ///   - [`parameter_values(ParameterValue)`](crate::operation::validate_pipeline_definition::builders::ValidatePipelineDefinitionFluentBuilder::parameter_values) / [`set_parameter_values(Option<Vec<ParameterValue>>)`](crate::operation::validate_pipeline_definition::builders::ValidatePipelineDefinitionFluentBuilder::set_parameter_values): <p>The parameter values used with the pipeline.</p>
    /// - On success, responds with [`ValidatePipelineDefinitionOutput`](crate::operation::validate_pipeline_definition::ValidatePipelineDefinitionOutput) with field(s):
    ///   - [`validation_errors(Option<Vec<ValidationError>>)`](crate::operation::validate_pipeline_definition::ValidatePipelineDefinitionOutput::validation_errors): <p>Any validation errors that were found.</p>
    ///   - [`validation_warnings(Option<Vec<ValidationWarning>>)`](crate::operation::validate_pipeline_definition::ValidatePipelineDefinitionOutput::validation_warnings): <p>Any validation warnings that were found.</p>
    ///   - [`errored(bool)`](crate::operation::validate_pipeline_definition::ValidatePipelineDefinitionOutput::errored): <p>Indicates whether there were validation errors.</p>
    /// - On failure, responds with [`SdkError<ValidatePipelineDefinitionError>`](crate::operation::validate_pipeline_definition::ValidatePipelineDefinitionError)
    pub fn validate_pipeline_definition(&self) -> crate::operation::validate_pipeline_definition::builders::ValidatePipelineDefinitionFluentBuilder {
        crate::operation::validate_pipeline_definition::builders::ValidatePipelineDefinitionFluentBuilder::new(self.handle.clone())
    }
}
