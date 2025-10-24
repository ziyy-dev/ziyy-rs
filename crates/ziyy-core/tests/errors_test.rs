use ziyy_core::try_style;

#[test]
pub fn it_errors_on_mismatched_tags() {
    let styled = try_style("<c>styled</x>");
    assert!(styled.is_err());
}

#[test]
pub fn it_errors_on_invalid_tag_name() {
    let styled = try_style("<12 >");
    assert!(styled.is_err());
}

#[test]
pub fn it_errors_on_unterminated_string_in_tag() {
    let styled = try_style("<div n=\">");
    assert!(styled.is_err());
    let styled = try_style("<div n='>");
    assert!(styled.is_err());
}

#[test]
pub fn it_errors_on_non_string_tag_atrribute_value() {
    let styled = try_style("<div b=l>");
    assert!(styled.is_err());
}

#[test]
pub fn it_errors_on_invalid_number() {
    let styled = try_style("<c rgb='f,2,3'>");
    assert!(styled.is_err());
}

#[test]
pub fn it_errors_on_invalid_color() {
    let styled = try_style("<b c='gold'>");
    assert!(styled.is_err());
}
