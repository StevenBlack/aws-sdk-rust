// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`CreateQuantumTask`](crate::operation::create_quantum_task::builders::CreateQuantumTaskFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`client_token(impl Into<String>)`](crate::operation::create_quantum_task::builders::CreateQuantumTaskFluentBuilder::client_token) / [`set_client_token(Option<String>)`](crate::operation::create_quantum_task::builders::CreateQuantumTaskFluentBuilder::set_client_token): <p>The client token associated with the request.</p>
    ///   - [`device_arn(impl Into<String>)`](crate::operation::create_quantum_task::builders::CreateQuantumTaskFluentBuilder::device_arn) / [`set_device_arn(Option<String>)`](crate::operation::create_quantum_task::builders::CreateQuantumTaskFluentBuilder::set_device_arn): <p>The ARN of the device to run the task on.</p>
    ///   - [`device_parameters(impl Into<String>)`](crate::operation::create_quantum_task::builders::CreateQuantumTaskFluentBuilder::device_parameters) / [`set_device_parameters(Option<String>)`](crate::operation::create_quantum_task::builders::CreateQuantumTaskFluentBuilder::set_device_parameters): <p>The parameters for the device to run the task on.</p>
    ///   - [`shots(i64)`](crate::operation::create_quantum_task::builders::CreateQuantumTaskFluentBuilder::shots) / [`set_shots(Option<i64>)`](crate::operation::create_quantum_task::builders::CreateQuantumTaskFluentBuilder::set_shots): <p>The number of shots to use for the task.</p>
    ///   - [`output_s3_bucket(impl Into<String>)`](crate::operation::create_quantum_task::builders::CreateQuantumTaskFluentBuilder::output_s3_bucket) / [`set_output_s3_bucket(Option<String>)`](crate::operation::create_quantum_task::builders::CreateQuantumTaskFluentBuilder::set_output_s3_bucket): <p>The S3 bucket to store task result files in.</p>
    ///   - [`output_s3_key_prefix(impl Into<String>)`](crate::operation::create_quantum_task::builders::CreateQuantumTaskFluentBuilder::output_s3_key_prefix) / [`set_output_s3_key_prefix(Option<String>)`](crate::operation::create_quantum_task::builders::CreateQuantumTaskFluentBuilder::set_output_s3_key_prefix): <p>The key prefix for the location in the S3 bucket to store task results in.</p>
    ///   - [`action(impl Into<String>)`](crate::operation::create_quantum_task::builders::CreateQuantumTaskFluentBuilder::action) / [`set_action(Option<String>)`](crate::operation::create_quantum_task::builders::CreateQuantumTaskFluentBuilder::set_action): <p>The action associated with the task.</p>
    ///   - [`tags(impl Into<String>, impl Into<String>)`](crate::operation::create_quantum_task::builders::CreateQuantumTaskFluentBuilder::tags) / [`set_tags(Option<HashMap<String, String>>)`](crate::operation::create_quantum_task::builders::CreateQuantumTaskFluentBuilder::set_tags): <p>Tags to be added to the quantum task you're creating.</p>
    ///   - [`job_token(impl Into<String>)`](crate::operation::create_quantum_task::builders::CreateQuantumTaskFluentBuilder::job_token) / [`set_job_token(Option<String>)`](crate::operation::create_quantum_task::builders::CreateQuantumTaskFluentBuilder::set_job_token): <p>The token for an Amazon Braket job that associates it with the quantum task.</p>
    /// - On success, responds with [`CreateQuantumTaskOutput`](crate::operation::create_quantum_task::CreateQuantumTaskOutput) with field(s):
    ///   - [`quantum_task_arn(Option<String>)`](crate::operation::create_quantum_task::CreateQuantumTaskOutput::quantum_task_arn): <p>The ARN of the task created by the request.</p>
    /// - On failure, responds with [`SdkError<CreateQuantumTaskError>`](crate::operation::create_quantum_task::CreateQuantumTaskError)
    pub fn create_quantum_task(&self) -> crate::operation::create_quantum_task::builders::CreateQuantumTaskFluentBuilder {
        crate::operation::create_quantum_task::builders::CreateQuantumTaskFluentBuilder::new(self.handle.clone())
    }
}
