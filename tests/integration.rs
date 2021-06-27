#[test]
fn it_parses_standard_example() {
    let input = String::from("***\nLorem ipsum");
    let expected = String::from("<hr /><p>Lorem ipsum</p>");

    assert_eq!(rustdown::parse(input), expected);
}

#[test]
fn it_parses_variant_example() {}

#[test]
fn it_prioritizes_code_block_over_inline_code_span() {
    let code_block_input = String::from("```Code block, not span```");
    let expected_code_block = String::from("<pre><code>Code block, not span</code></pre>");

    let code_span_input = String::from("``Code span, not block``");
    let expected_code_span = String::from("<p><code>Code span, not block</code></p>");

    assert_eq!(rustdown::parse(code_block_input), expected_code_block);
    assert_eq!(rustdown::parse(code_span_input), expected_code_span);
}
