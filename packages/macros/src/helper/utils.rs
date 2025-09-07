use syn::Attribute;

pub fn attr_of<'a>(attrs: &'a [Attribute], name: &str) -> Option<&'a Attribute> {
    attrs
        .iter()
        .find(|&a| a.path().get_ident().filter(|i| *i == name).is_some())
}
