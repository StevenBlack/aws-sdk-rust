// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`PutEncryptionConfiguration`](crate::operation::put_encryption_configuration::builders::PutEncryptionConfigurationFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`kms_key_id(impl Into<String>)`](crate::operation::put_encryption_configuration::builders::PutEncryptionConfigurationFluentBuilder::kms_key_id) / [`set_kms_key_id(Option<String>)`](crate::operation::put_encryption_configuration::builders::PutEncryptionConfigurationFluentBuilder::set_kms_key_id): <p>The ID of the KMS key that is used for encryption.</p>
    ///   - [`encryption_type(EncryptionType)`](crate::operation::put_encryption_configuration::builders::PutEncryptionConfigurationFluentBuilder::encryption_type) / [`set_encryption_type(Option<EncryptionType>)`](crate::operation::put_encryption_configuration::builders::PutEncryptionConfigurationFluentBuilder::set_encryption_type): <p>The type of encryption. Choose <code>KMS_BASED_ENCRYPTION</code> to use a KMS key or <code>FLEETWISE_DEFAULT_ENCRYPTION</code> to use an Amazon Web Services managed key.</p>
    /// - On success, responds with [`PutEncryptionConfigurationOutput`](crate::operation::put_encryption_configuration::PutEncryptionConfigurationOutput) with field(s):
    ///   - [`kms_key_id(Option<String>)`](crate::operation::put_encryption_configuration::PutEncryptionConfigurationOutput::kms_key_id): <p>The ID of the KMS key that is used for encryption.</p>
    ///   - [`encryption_status(Option<EncryptionStatus>)`](crate::operation::put_encryption_configuration::PutEncryptionConfigurationOutput::encryption_status): <p>The encryption status.</p>
    ///   - [`encryption_type(Option<EncryptionType>)`](crate::operation::put_encryption_configuration::PutEncryptionConfigurationOutput::encryption_type): <p>The type of encryption. Set to <code>KMS_BASED_ENCRYPTION</code> to use an KMS key that you own and manage. Set to <code>FLEETWISE_DEFAULT_ENCRYPTION</code> to use an Amazon Web Services managed key that is owned by the Amazon Web Services IoT FleetWise service account.</p>
    /// - On failure, responds with [`SdkError<PutEncryptionConfigurationError>`](crate::operation::put_encryption_configuration::PutEncryptionConfigurationError)
    pub fn put_encryption_configuration(&self) -> crate::operation::put_encryption_configuration::builders::PutEncryptionConfigurationFluentBuilder {
        crate::operation::put_encryption_configuration::builders::PutEncryptionConfigurationFluentBuilder::new(self.handle.clone())
    }
}
