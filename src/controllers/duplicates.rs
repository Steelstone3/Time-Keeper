use crate::models::{duplicate_finder::DuplicateFinder, new_line::NewLine};
use std::collections::{HashMap, HashSet};

impl DuplicateFinder {
    pub fn parse_editor_content(&mut self, editor_text: String) -> Vec<NewLine> {
        self.content = editor_text;

        self.content
            .lines()
            .enumerate()
            .map(|(index, content)| NewLine {
                line_number: index as u128 + 1,
                content: content.to_string(),
            })
            .collect()
    }

    pub fn find_duplicates(&mut self, editor_text: String) {
        let new_lines = self.parse_editor_content(editor_text);

        let mut content_map: HashMap<String, Vec<NewLine>> = HashMap::new();

        for line in new_lines {
            content_map
                .entry(line.content.clone())
                .or_default()
                .push(line);
        }

        self.duplicate_lines = content_map
            .into_iter()
            .flat_map(|(line, duplicates)| {
                if line.is_empty() {
                    vec![]
                } else if duplicates.len() > 1 {
                    duplicates
                } else {
                    vec![]
                }
            })
            .collect();
    }

    pub fn remove_duplicates(&mut self, editor_text: String) -> String {
        let new_lines = self.parse_editor_content(editor_text);

        let mut seen = HashSet::new();
        let mut filtered_lines = Vec::new();

        for line in new_lines {
            if seen.insert(line.content.clone()) || line.content.clone() == "" {
                filtered_lines.push(line.content.clone());
            }
        }

        filtered_lines.join("\n")
    }
}

#[cfg(test)]
mod duplicates_should {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("", vec![])]
    #[case("\n", vec![NewLine{ line_number: 1, content: "".to_string() }])]
    #[case("hi\nhi", vec![NewLine{ line_number: 1, content: "hi".to_string() }, NewLine{ line_number: 2, content: "hi".to_string() }])]
    #[case("hi\n\nthere", vec![NewLine{ line_number: 1, content: "hi".to_string() },NewLine{ line_number: 2, content: "".to_string() },  NewLine{ line_number: 3, content: "there".to_string() }])]
    #[case("\nhi\n\nthere\n", vec![NewLine { line_number: 1, content: "".to_string() }, NewLine { line_number: 2, content: "hi".to_string() }, NewLine { line_number: 3, content: "".to_string()}, NewLine { line_number: 4, content: "there".to_string() }])]
    #[case("\n\nhi\nhi\nlow\ntnt\nlow\nspade\nhi", vec![NewLine { line_number: 1, content: "".to_string() }, NewLine { line_number: 2, content: "".to_string() }, NewLine { line_number: 3, content: "hi".to_string() }, NewLine { line_number: 4, content: "hi".to_string() }, NewLine { line_number: 5, content: "low".to_string() },  NewLine { line_number: 6, content: "tnt".to_string() }, NewLine { line_number: 7, content: "low".to_string() },  NewLine { line_number: 8, content: "spade".to_string() }, NewLine { line_number: 9, content: "hi".to_string() }])]
    fn parse_editor_content(#[case] editor_text: String, #[case] expected_new_lines: Vec<NewLine>) {
        // Given
        let mut duplicate_finder = DuplicateFinder {
            ..Default::default()
        };

        // When
        let new_lines = duplicate_finder.parse_editor_content(editor_text);

        // Then
        assert_eq!(expected_new_lines.len(), new_lines.len());
        for expected_new_line in expected_new_lines {
            assert!(new_lines.contains(&expected_new_line));
        }
    }

    #[rstest]
    #[case("", vec![])]
    #[case("\n\n", vec![])]
    #[case("hi\nhi", vec![NewLine { line_number: 1, content: "hi".to_string() }, NewLine { line_number: 2, content: "hi".to_string() }])]
    #[case("hi\nhi\nlow", vec![NewLine { line_number: 1, content: "hi".to_string() }, NewLine { line_number: 2, content: "hi".to_string() }])]
    #[case("high\nhi\nhi\nlow", vec![NewLine { line_number: 2, content: "hi".to_string() }, NewLine { line_number: 3, content: "hi".to_string() }])]
    #[case("\n\nhi\nhi\nlow\ntnt\nlow\nspade\nhi", vec![NewLine { line_number: 3, content: "hi".to_string() }, NewLine { line_number: 4, content: "hi".to_string() }, NewLine { line_number: 9, content: "hi".to_string() }, NewLine { line_number: 5, content: "low".to_string() }, NewLine { line_number: 7, content: "low".to_string() }])]
    fn find_duplicates(#[case] editor_text: String, #[case] expected_duplicates: Vec<NewLine>) {
        // Given
        let mut duplicate_finder = DuplicateFinder {
            ..Default::default()
        };

        // When
        duplicate_finder.find_duplicates(editor_text);

        // Then
        assert_eq!(
            expected_duplicates.len(),
            duplicate_finder.duplicate_lines.len()
        );
        for expected_duplicate in expected_duplicates {
            assert!(
                duplicate_finder
                    .duplicate_lines
                    .contains(&expected_duplicate)
            );
        }
    }

    #[rstest]
    #[case("", "")]
    #[case("hi", "hi")]
    #[case("hi\nhi", "hi")]
    #[case("hi\nhi\nthere", "hi\nthere")]
    #[case("hi\nhi\nthere\nthere", "hi\nthere")]
    #[case("hi\nhi\nthere\nwhere", "hi\nthere\nwhere")]
    #[case("hi\n\nhi\nthere\n\nwhere", "hi\n\nthere\n\nwhere")]
    #[case("\n\nhi\nhi\nlow\ntnt\nlow\nspade\nhi", "\n\nhi\nlow\ntnt\nspade")]
    fn remove_duplicates(#[case] editor_text: String, #[case] expected_content: String) {
        // Given
        let mut duplicate_finder = DuplicateFinder {
            ..Default::default()
        };

        // When
        let content = duplicate_finder.remove_duplicates(editor_text);

        // Then
        assert_eq!(expected_content, content);
    }
}
