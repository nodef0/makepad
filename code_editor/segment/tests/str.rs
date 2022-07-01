mod test_data;

#[test]
fn test_graphemes() {
    for test in test_data::grapheme::TEST_DATA {
        use makepad_segment::str::StrExt;

        assert_eq!(
            test.join("")
                .as_str()
                .graphemes()
                .collect::<Vec<_>>()
                .as_slice(),
            test,
        );
    }
}

#[test]
fn test_word() {
    for test in test_data::word::TEST_DATA {
        use makepad_segment::str::StrExt;

        assert_eq!(
            test.join("")
                .as_str()
                .words()
                .collect::<Vec<_>>()
                .as_slice(),
            test,
        );
    }
}
