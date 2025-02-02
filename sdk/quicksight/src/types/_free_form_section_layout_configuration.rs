// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The free-form layout configuration of a section.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct FreeFormSectionLayoutConfiguration {
    /// <p>The elements that are included in the free-form layout.</p>
    pub elements: ::std::option::Option<::std::vec::Vec<crate::types::FreeFormLayoutElement>>,
}
impl FreeFormSectionLayoutConfiguration {
    /// <p>The elements that are included in the free-form layout.</p>
    pub fn elements(&self) -> ::std::option::Option<&[crate::types::FreeFormLayoutElement]> {
        self.elements.as_deref()
    }
}
impl FreeFormSectionLayoutConfiguration {
    /// Creates a new builder-style object to manufacture [`FreeFormSectionLayoutConfiguration`](crate::types::FreeFormSectionLayoutConfiguration).
    pub fn builder() -> crate::types::builders::FreeFormSectionLayoutConfigurationBuilder {
        crate::types::builders::FreeFormSectionLayoutConfigurationBuilder::default()
    }
}

/// A builder for [`FreeFormSectionLayoutConfiguration`](crate::types::FreeFormSectionLayoutConfiguration).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct FreeFormSectionLayoutConfigurationBuilder {
    pub(crate) elements: ::std::option::Option<::std::vec::Vec<crate::types::FreeFormLayoutElement>>,
}
impl FreeFormSectionLayoutConfigurationBuilder {
    /// Appends an item to `elements`.
    ///
    /// To override the contents of this collection use [`set_elements`](Self::set_elements).
    ///
    /// <p>The elements that are included in the free-form layout.</p>
    pub fn elements(mut self, input: crate::types::FreeFormLayoutElement) -> Self {
        let mut v = self.elements.unwrap_or_default();
        v.push(input);
        self.elements = ::std::option::Option::Some(v);
        self
    }
    /// <p>The elements that are included in the free-form layout.</p>
    pub fn set_elements(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::FreeFormLayoutElement>>) -> Self {
        self.elements = input;
        self
    }
    /// <p>The elements that are included in the free-form layout.</p>
    pub fn get_elements(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::FreeFormLayoutElement>> {
        &self.elements
    }
    /// Consumes the builder and constructs a [`FreeFormSectionLayoutConfiguration`](crate::types::FreeFormSectionLayoutConfiguration).
    pub fn build(self) -> crate::types::FreeFormSectionLayoutConfiguration {
        crate::types::FreeFormSectionLayoutConfiguration { elements: self.elements }
    }
}
