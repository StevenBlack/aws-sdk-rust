// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct AssociatePhoneNumbersWithVoiceConnectorInput {
    /// <p>The Amazon Chime Voice Connector ID.</p>
    pub voice_connector_id: ::std::option::Option<::std::string::String>,
    /// <p>List of phone numbers, in E.164 format.</p>
    pub e164_phone_numbers: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    /// <p>If true, associates the provided phone numbers with the provided Amazon Chime Voice Connector and removes any previously existing associations. If false, does not associate any phone numbers that have previously existing associations.</p>
    pub force_associate: ::std::option::Option<bool>,
}
impl AssociatePhoneNumbersWithVoiceConnectorInput {
    /// <p>The Amazon Chime Voice Connector ID.</p>
    pub fn voice_connector_id(&self) -> ::std::option::Option<&str> {
        self.voice_connector_id.as_deref()
    }
    /// <p>List of phone numbers, in E.164 format.</p>
    pub fn e164_phone_numbers(&self) -> ::std::option::Option<&[::std::string::String]> {
        self.e164_phone_numbers.as_deref()
    }
    /// <p>If true, associates the provided phone numbers with the provided Amazon Chime Voice Connector and removes any previously existing associations. If false, does not associate any phone numbers that have previously existing associations.</p>
    pub fn force_associate(&self) -> ::std::option::Option<bool> {
        self.force_associate
    }
}
impl AssociatePhoneNumbersWithVoiceConnectorInput {
    /// Creates a new builder-style object to manufacture [`AssociatePhoneNumbersWithVoiceConnectorInput`](crate::operation::associate_phone_numbers_with_voice_connector::AssociatePhoneNumbersWithVoiceConnectorInput).
    pub fn builder() -> crate::operation::associate_phone_numbers_with_voice_connector::builders::AssociatePhoneNumbersWithVoiceConnectorInputBuilder
    {
        crate::operation::associate_phone_numbers_with_voice_connector::builders::AssociatePhoneNumbersWithVoiceConnectorInputBuilder::default()
    }
}

/// A builder for [`AssociatePhoneNumbersWithVoiceConnectorInput`](crate::operation::associate_phone_numbers_with_voice_connector::AssociatePhoneNumbersWithVoiceConnectorInput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct AssociatePhoneNumbersWithVoiceConnectorInputBuilder {
    pub(crate) voice_connector_id: ::std::option::Option<::std::string::String>,
    pub(crate) e164_phone_numbers: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    pub(crate) force_associate: ::std::option::Option<bool>,
}
impl AssociatePhoneNumbersWithVoiceConnectorInputBuilder {
    /// <p>The Amazon Chime Voice Connector ID.</p>
    pub fn voice_connector_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.voice_connector_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Chime Voice Connector ID.</p>
    pub fn set_voice_connector_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.voice_connector_id = input;
        self
    }
    /// <p>The Amazon Chime Voice Connector ID.</p>
    pub fn get_voice_connector_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.voice_connector_id
    }
    /// Appends an item to `e164_phone_numbers`.
    ///
    /// To override the contents of this collection use [`set_e164_phone_numbers`](Self::set_e164_phone_numbers).
    ///
    /// <p>List of phone numbers, in E.164 format.</p>
    pub fn e164_phone_numbers(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        let mut v = self.e164_phone_numbers.unwrap_or_default();
        v.push(input.into());
        self.e164_phone_numbers = ::std::option::Option::Some(v);
        self
    }
    /// <p>List of phone numbers, in E.164 format.</p>
    pub fn set_e164_phone_numbers(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.e164_phone_numbers = input;
        self
    }
    /// <p>List of phone numbers, in E.164 format.</p>
    pub fn get_e164_phone_numbers(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        &self.e164_phone_numbers
    }
    /// <p>If true, associates the provided phone numbers with the provided Amazon Chime Voice Connector and removes any previously existing associations. If false, does not associate any phone numbers that have previously existing associations.</p>
    pub fn force_associate(mut self, input: bool) -> Self {
        self.force_associate = ::std::option::Option::Some(input);
        self
    }
    /// <p>If true, associates the provided phone numbers with the provided Amazon Chime Voice Connector and removes any previously existing associations. If false, does not associate any phone numbers that have previously existing associations.</p>
    pub fn set_force_associate(mut self, input: ::std::option::Option<bool>) -> Self {
        self.force_associate = input;
        self
    }
    /// <p>If true, associates the provided phone numbers with the provided Amazon Chime Voice Connector and removes any previously existing associations. If false, does not associate any phone numbers that have previously existing associations.</p>
    pub fn get_force_associate(&self) -> &::std::option::Option<bool> {
        &self.force_associate
    }
    /// Consumes the builder and constructs a [`AssociatePhoneNumbersWithVoiceConnectorInput`](crate::operation::associate_phone_numbers_with_voice_connector::AssociatePhoneNumbersWithVoiceConnectorInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::associate_phone_numbers_with_voice_connector::AssociatePhoneNumbersWithVoiceConnectorInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::associate_phone_numbers_with_voice_connector::AssociatePhoneNumbersWithVoiceConnectorInput {
                voice_connector_id: self.voice_connector_id,
                e164_phone_numbers: self.e164_phone_numbers,
                force_associate: self.force_associate,
            },
        )
    }
}
