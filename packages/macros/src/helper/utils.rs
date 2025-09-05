use syn::Attribute;

pub fn attr_of<'a>(attrs: &'a Vec<Attribute>, name: &str) -> Option<&'a Attribute> {
    attrs.iter().find(|&a| {
        a.path()
            .get_ident()
            .filter(|i| i.to_string() == name)
            .is_some()
    })
}
