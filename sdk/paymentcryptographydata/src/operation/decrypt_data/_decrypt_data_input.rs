// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq)]
pub struct DecryptDataInput {
    /// <p>The <code>keyARN</code> of the encryption key that Amazon Web Services Payment Cryptography uses for ciphertext decryption.</p>
    pub key_identifier: ::std::option::Option<::std::string::String>,
    /// <p>The ciphertext to decrypt.</p>
    pub cipher_text: ::std::option::Option<::std::string::String>,
    /// <p>The encryption key type and attributes for ciphertext decryption.</p>
    pub decryption_attributes: ::std::option::Option<crate::types::EncryptionDecryptionAttributes>,
}
impl DecryptDataInput {
    /// <p>The <code>keyARN</code> of the encryption key that Amazon Web Services Payment Cryptography uses for ciphertext decryption.</p>
    pub fn key_identifier(&self) -> ::std::option::Option<&str> {
        self.key_identifier.as_deref()
    }
    /// <p>The ciphertext to decrypt.</p>
    pub fn cipher_text(&self) -> ::std::option::Option<&str> {
        self.cipher_text.as_deref()
    }
    /// <p>The encryption key type and attributes for ciphertext decryption.</p>
    pub fn decryption_attributes(&self) -> ::std::option::Option<&crate::types::EncryptionDecryptionAttributes> {
        self.decryption_attributes.as_ref()
    }
}
impl ::std::fmt::Debug for DecryptDataInput {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("DecryptDataInput");
        formatter.field("key_identifier", &self.key_identifier);
        formatter.field("cipher_text", &"*** Sensitive Data Redacted ***");
        formatter.field("decryption_attributes", &self.decryption_attributes);
        formatter.finish()
    }
}
impl DecryptDataInput {
    /// Creates a new builder-style object to manufacture [`DecryptDataInput`](crate::operation::decrypt_data::DecryptDataInput).
    pub fn builder() -> crate::operation::decrypt_data::builders::DecryptDataInputBuilder {
        crate::operation::decrypt_data::builders::DecryptDataInputBuilder::default()
    }
}

/// A builder for [`DecryptDataInput`](crate::operation::decrypt_data::DecryptDataInput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default)]
pub struct DecryptDataInputBuilder {
    pub(crate) key_identifier: ::std::option::Option<::std::string::String>,
    pub(crate) cipher_text: ::std::option::Option<::std::string::String>,
    pub(crate) decryption_attributes: ::std::option::Option<crate::types::EncryptionDecryptionAttributes>,
}
impl DecryptDataInputBuilder {
    /// <p>The <code>keyARN</code> of the encryption key that Amazon Web Services Payment Cryptography uses for ciphertext decryption.</p>
    pub fn key_identifier(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.key_identifier = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The <code>keyARN</code> of the encryption key that Amazon Web Services Payment Cryptography uses for ciphertext decryption.</p>
    pub fn set_key_identifier(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.key_identifier = input;
        self
    }
    /// <p>The <code>keyARN</code> of the encryption key that Amazon Web Services Payment Cryptography uses for ciphertext decryption.</p>
    pub fn get_key_identifier(&self) -> &::std::option::Option<::std::string::String> {
        &self.key_identifier
    }
    /// <p>The ciphertext to decrypt.</p>
    pub fn cipher_text(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.cipher_text = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ciphertext to decrypt.</p>
    pub fn set_cipher_text(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.cipher_text = input;
        self
    }
    /// <p>The ciphertext to decrypt.</p>
    pub fn get_cipher_text(&self) -> &::std::option::Option<::std::string::String> {
        &self.cipher_text
    }
    /// <p>The encryption key type and attributes for ciphertext decryption.</p>
    pub fn decryption_attributes(mut self, input: crate::types::EncryptionDecryptionAttributes) -> Self {
        self.decryption_attributes = ::std::option::Option::Some(input);
        self
    }
    /// <p>The encryption key type and attributes for ciphertext decryption.</p>
    pub fn set_decryption_attributes(mut self, input: ::std::option::Option<crate::types::EncryptionDecryptionAttributes>) -> Self {
        self.decryption_attributes = input;
        self
    }
    /// <p>The encryption key type and attributes for ciphertext decryption.</p>
    pub fn get_decryption_attributes(&self) -> &::std::option::Option<crate::types::EncryptionDecryptionAttributes> {
        &self.decryption_attributes
    }
    /// Consumes the builder and constructs a [`DecryptDataInput`](crate::operation::decrypt_data::DecryptDataInput).
    pub fn build(self) -> ::std::result::Result<crate::operation::decrypt_data::DecryptDataInput, ::aws_smithy_http::operation::error::BuildError> {
        ::std::result::Result::Ok(crate::operation::decrypt_data::DecryptDataInput {
            key_identifier: self.key_identifier,
            cipher_text: self.cipher_text,
            decryption_attributes: self.decryption_attributes,
        })
    }
}
impl ::std::fmt::Debug for DecryptDataInputBuilder {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("DecryptDataInputBuilder");
        formatter.field("key_identifier", &self.key_identifier);
        formatter.field("cipher_text", &"*** Sensitive Data Redacted ***");
        formatter.field("decryption_attributes", &self.decryption_attributes);
        formatter.finish()
    }
}
