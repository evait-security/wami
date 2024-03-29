use crate::search;
use colored::Colorize;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Template {
    id: String,              // A unique name in lower case letters.
    title: String,           // The real name of the tool / software.
    tags: Vec<String>,       // An vector of lower case tags for the search function.
    description: String,     // A longer text describing the why and what it does.
    references: Vec<String>, // Links to websites, repositories, or other resources.
    why_not: Vec<String>,    // Links to other programs or sites as an alternative.
    distance: f32,           // This will set the average similarity to the search value.
}

impl Template {
    // Create a new Template an return it.
    // I am using the new method,
    // because I will transform the id and tags fields to the right form.
    // and I will calculate the distance to the search string.
    pub fn new(
        in_id: String,
        in_id_search: String,
        in_title: String,
        in_title_search: String,
        in_tags: Vec<String>,
        in_tags_search: Vec<String>,
        in_description: String,
        in_description_search: String,
        in_references: Vec<String>,
        in_references_search: Vec<String>,
        in_why_not: Vec<String>,
    ) -> Template {
        // Convert the incoming unique name to lowercase alphanumeric letters with hyphens.
        let out_id: String = Template::convert_to_lowercase_alphanumeric_with_hyphens(&in_id);

        // Convert search unique name to lowercase alphanumeric letters with hyphens.
        let out_id_search: String =
            Template::convert_to_lowercase_alphanumeric_with_hyphens(&in_id_search);

        // Calculate the similarity from the unique name to the unique search name.
        let out_id_distance: f32 = search::Search::cosine_similarity(&out_id, &out_id_search);

        // Calculate the similarity from the real title to the search title.
        let out_title_distance: f32 = search::Search::cosine_similarity(&in_title, &in_title_search);

        // Convert the incoming tags to lowercase alphanumeric letters with hyphens.
        let out_tags: Vec<String> = Template::convert_tags_to_excepted_format(&in_tags);

        // Convert the incoming search tags to lowercase alphanumeric letter with hyphens.
        let out_tags_search: Vec<String> =
            Template::convert_tags_to_excepted_format(&in_tags_search);

        // Calculate the similarities from the tags to the search tags.
        let out_tags_distance: f32 = search::Search::cosine_similarities(&out_tags, &out_tags_search);

        // Calculate the similarity from the description to the search description.
        let out_description_distance: f32 =
            search::Search::cosine_similarity(&in_description, &in_description_search);

        // Calculate the similarity from the references to the search references.
        let out_references_distance: f32 =
            search::Search::cosine_similarities(&in_references, &in_references_search);

        // Calculate the over all similarity. This is done by adding all 5 distances together and multiplying them by 0.2.
        let out_distance: f32 = (out_id_distance
            + out_title_distance
            + out_tags_distance
            + out_description_distance
            + out_references_distance)
            * 0.2;

        // Creating and returning the new Template
        Template {
            id: out_id,                  // The unique name of the template
            title: in_title,             // The title of the template
            tags: out_tags,              // The tags of the template
            description: in_description, // The description of the template
            references: in_references,   // The references of the template
            why_not: in_why_not,         // This will save the alternative programmes.
            distance: out_distance,      // The average similarity of all similarities
        }
    }

    pub fn distance(&self) -> f32 {
        self.distance
    }

    // This will return a string, for the console.
    pub fn to_string(&self, why_not: bool) -> String {
        let mut out_string = format!("{} {}\n    {}\n    {}\n{}",
                                        &self.id.truecolor(90,90, 255),
                                        &self.title.green(),
                                        &Template::tags_to_string(&self).truecolor(200,200,150),
                                        &self.description.truecolor(150,150,200),
                                        &Template::references_to_string(&self).truecolor(200,200,200),
                                    );
        if why_not && self.why_not.is_empty(){
            let why_not_out = format!("/n    {}", &self.why_not_to_string().bold().green());
            out_string.push_str(&why_not_out);
        }
        out_string.to_owned()
    }

    pub fn to_short_string(&self, why_not: bool) -> String {
        let mut out_string = format!("{}\n{}",&self.title.green(),&self.references_to_string().truecolor(200, 200, 200));
        if why_not && self.why_not.is_empty() {
            let why_not_out = format!("\n    {}", &self.why_not_to_string().bold().green());
            out_string.push_str(&why_not_out);
        }
        out_string.to_owned()
    }

    // Put all tags in a line and separate them with an , except the last own.
    fn tags_to_string(&self) -> String {
        let mut out_string: String = String::new();
        let tags_len = self.tags.len();
        for (index, tag) in self.tags.iter().enumerate() {
            out_string.push_str(tag);
            if index < tags_len - 1 {
                out_string.push_str(", ");
            }
        }

        out_string
    }

    // Put the references in a string, one reference per line
    fn references_to_string(&self) -> String {
        let mut out_string: String = String::new();
        let reference_len = self.references.len();
        for (index, reference) in self.references.iter().enumerate() {
            out_string.push_str("    ");
            out_string.push_str(reference);
            if index < reference_len - 1 {
                out_string.push_str("\n");
            }
        }

        out_string
    }

    // Put the why_not in a string, one why_not per line
    fn why_not_to_string(&self) -> String {
        let mut out_string: String = String::new();
        let why_not_len = self.why_not.len();
        for (index, why_not) in self.why_not.iter().enumerate() {
            out_string.push_str("Why Not ");
            out_string.push_str(why_not);
            if index < why_not_len - 1 {
                out_string.push_str("\n");
            }
        }
        out_string
    }

    // Make the tags vec robust
    // Will make all tags uniform, this simplifies the search algorithmic.
    pub fn convert_tags_to_excepted_format(in_tags: &Vec<String>) -> Vec<String> {
        in_tags
            .iter()
            .map(|tag| Template::convert_to_lowercase_alphanumeric_with_hyphens(&tag))
            .collect()
    }

    // Make the string robust.
    // If there is an error in the String, it will be converted or deleted.
    // See the requirements for the unique name and tags fields.
    // Only lowercase alphanumeric letters or hyphens are allowed.
    pub fn convert_to_lowercase_alphanumeric_with_hyphens(in_str: &String) -> String {
        in_str
            .chars()
            .filter(|c| c.is_ascii_alphanumeric() || *c == '-' || c.is_whitespace())
            .map(|c| c.to_ascii_lowercase().to_string())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Tests the Template function new
    #[test]
    fn test_template_structure() {
        let tags_vec: Vec<String> = vec!["tag1".to_string(), "tag2".to_string()];
        let references_vec: Vec<String> = vec!["ref1".to_string(), "ref2".to_string()];
        let search_vec: Vec<String> = vec!["search2".to_string(), "search3".to_string()];
        let why_not_vec: Vec<String> = vec!["why_not1".to_string(), "why_not2".to_string()];
        let template: Template = Template::new(
            "id".to_string(),
            "id_search".to_string(),
            "title".to_string(),
            "title_search".to_string(),
            tags_vec.clone(),
            search_vec.clone(),
            "description".to_string(),
            "description_search".to_string(),
            references_vec.clone(),
            search_vec.clone(),
            why_not_vec.clone(),
        );

        assert_eq!(template.id, "id");
        assert_ne!(template.id, "no_id");
        assert_eq!(template.title, "title");
        assert_ne!(template.title, "no_title");
        assert_eq!(template.tags[0], "tag1");
        assert_eq!(template.tags[1], "tag2");
        assert_eq!(template.tags, tags_vec);
        assert_ne!(template.tags[0], "tag2");
        assert_ne!(template.tags[1], "tag1");
        assert_ne!(template.tags, why_not_vec);
        assert_eq!(template.description, "description");
        assert_ne!(template.description, "no_description");
        assert_eq!(template.references[0], "ref1");
        assert_eq!(template.references[1], "ref2");
        assert_eq!(template.references, references_vec);
        assert_ne!(template.references[0], "ref2");
        assert_ne!(template.references[1], "ref1");
        assert_ne!(template.references, tags_vec);
        assert_eq!(template.why_not[0], "why_not1");
        assert_eq!(template.why_not[1], "why_not2");
        assert_eq!(template.why_not, why_not_vec);
        assert_ne!(template.why_not[0], "why_not2");
        assert_ne!(template.why_not[1], "why_not1");
        assert_ne!(template.why_not, references_vec);
    }

    // Test the Template function new, specially the converting to lowercase at the id and tag values.
    #[test]
    fn test_new() {
        let tags = vec!["Tag1".to_string(), "Tag2".to_string()];
        let references = vec!["ref1".to_string(), "ref2".to_string()];
        let search_vec: Vec<String> = vec!["search2".to_string(), "search3".to_string()];
        let why_not_vec: Vec<String> = vec!["why_not1".to_string(), "why_not2".to_string()];
        let template = Template::new(
            "Id".to_string(),
            "id_search".to_string(),
            "Title".to_string(),
            "title_search".to_string(),
            tags.clone(),
            search_vec.clone(),
            "Description".to_string(),
            "description_search".to_string(),
            references.clone(),
            search_vec.clone(),
            why_not_vec.clone(),
        );

        // The id and tags should be in lowercase
        assert_eq!(template.id, "id");
        assert_ne!(template.id, "Id");
        assert_eq!(template.title, "Title");
        assert_ne!(template.title, "title");
        assert_eq!(template.tags, vec!["tag1", "tag2"]);
        assert_eq!(template.tags[0], "tag1");
        assert_ne!(template.tags[0], "Tag1");
        assert_eq!(template.tags[1], "tag2");
        assert_ne!(template.tags[1], "Tag2");
        assert_ne!(template.tags, vec!["Tag1", "Tag2"]);
        assert_ne!(template.tags, vec!["tag1", "Tag2"]);
        assert_ne!(template.tags, vec!["Tag1", "tag2"]);
        assert_eq!(template.description, "Description");
        assert_ne!(template.description, "description");
        assert_eq!(template.references, vec!["ref1", "ref2"]);
        assert_eq!(template.references[0], "ref1");
        assert_eq!(template.references[1], "ref2");
        assert_ne!(template.references[0], "Ref1");
        assert_ne!(template.references[1], "Ref2");
        assert_ne!(template.references, vec!["Ref1", "Ref2"]);
        assert_ne!(template.references, vec!["ref1", "Ref2"]);
        assert_ne!(template.references, vec!["Ref1", "ref2"]);
        assert_eq!(template.why_not, vec!["why_not1", "why_not2"]);
        assert_eq!(template.why_not[0], "why_not1");
        assert_eq!(template.why_not[1], "why_not2");
        assert_ne!(template.why_not, vec!["Why_not1", "why_not2"]);
        assert_ne!(template.why_not, vec!["why_not1", "Why_not2"]);
        assert_ne!(template.why_not[0], "Why_not1");
        assert_ne!(template.why_not[1], "Why_not2");
    }

    // Tests for the to_string function with why_not = true
    #[test]
    fn test_to_string() {
        let in_why_not_bool = true;
        let tags: Vec<String> = vec!["tag1".to_string(), "tag2".to_string()];
        let references: Vec<String> = vec!["ref1".to_string(), "ref2".to_string()];
        let why_not: Vec<String> = vec!["why_not1".to_string(), "why_not2".to_string()];
        let template = Template::new(
            "id".to_string(),
            "id_search".to_string(),
            "title".to_string(),
            "title_search".to_string(),
            tags.clone(),
            tags.clone(),
            "description".to_string(),
            "description_search".to_string(),
            references.clone(),
            references.clone(),
            why_not.clone(),
        );

        // Compare the template to string output with the expected output
        let expected_output = "id title\n    tag1, tag2\n    description\n    ref1\n    ref2";
        assert_eq!(template.to_string(in_why_not_bool), expected_output);
    }

    // Tests for the to_string function with why_not = false
    #[test]
    fn test_to_string_why_not_false() {
        let in_why_not_bool = false;
        let tags: Vec<String> = vec!["tag1".to_string(), "tag2".to_string()];
        let references: Vec<String> = vec!["ref1".to_string(), "ref2".to_string()];
        let why_not: Vec<String> = vec!["why_not1".to_string(), "why_not2".to_string()];
        let template = Template::new(
            "id".to_string(),
            "id_search".to_string(),
            "title".to_string(),
            "title_search".to_string(),
            tags.clone(),
            tags.clone(),
            "description".to_string(),
            "description_search".to_string(),
            references.clone(),
            references.clone(),
            why_not.clone(),
        );

        // Compare the template to string output with the expected output
        let expected_output = "id title\n    tag1, tag2\n    description\n    ref1\n    ref2";
        assert_eq!(template.to_string(in_why_not_bool), expected_output);
    }


    // Test for the to_short_string function why_not = true
    #[test]
    fn test_to_short_string() {
        let in_why_not_bool = true;
        let tags: Vec<String> = vec!["tag1".to_string(), "tag2".to_string()];
        let references: Vec<String> = vec!["ref1".to_string(), "ref2".to_string()];
        let why_not: Vec<String> = vec!["why_not1".to_string(), "why_not2".to_string()];
        let template = Template::new(
            "id".to_string(),
            "id_search".to_string(),
            "title".to_string(),
            "title".to_string(),
            tags.clone(),
            tags.clone(),
            "description".to_string(),
            "description_search".to_string(),
            references.clone(),
            references.clone(),
            why_not.clone(),
        );

        // Compare the template to string output with the expected output
        let expected_output: &str = "id title\n    tag1, tag2\n    description\n    ref1\n    ref2";
        assert_eq!(template.to_string(in_why_not_bool), expected_output);
    }

    // Test for the to_short_string function why_not = false
    #[test]
    fn test_to_short_string_why_not_false() {
        let in_why_not_bool = false;
        let tags: Vec<String> = vec!["tag1".to_string(), "tag2".to_string()];
        let references: Vec<String> = vec!["ref1".to_string(), "ref2".to_string()];
        let why_not: Vec<String> = vec!["why_not1".to_string(), "why_not2".to_string()];
        let template = Template::new(
            "id".to_string(),
            "id_search".to_string(),
            "title".to_string(),
            "title".to_string(),
            tags.clone(),
            tags.clone(),
            "description".to_string(),
            "description_search".to_string(),
            references.clone(),
            references.clone(),
            why_not.clone(),
        );

        // Compare the template to string output with the expected output
        let expected_output: &str = "id title\n    tag1, tag2\n    description\n    ref1\n    ref2";
        assert_eq!(template.to_string(in_why_not_bool), expected_output);
    }

    // Test for the to_short_string_with_why_not why_not = false
    #[test]
    fn test_to_short_string_with_why_not() {
        let in_why_not_bool = false;
        let tags: Vec<String> = vec!["tag1".to_string(), "tag2".to_string()];
        let references: Vec<String> = vec!["ref1".to_string(), "ref2".to_string()];
        let why_not: Vec<String> = vec!["why_not1".to_string(), "why_not2".to_string()];
        let template = Template::new(
            "id".to_string(),
            "id_search".to_string(),
            "title".to_string(),
            "title_search".to_string(),
            tags.clone(),
            tags.clone(),
            "description".to_string(),
            "description_search".to_string(),
            references.clone(),
            references.clone(),
            why_not.clone(),
        );

        // Compare the template to string output with the expected output
        let expected_output: &str =
            "id title\n    tag1, tag2\n    description\n    ref1\n    ref2";
        assert_eq!(template.to_string(in_why_not_bool), expected_output);
    }

    // Test for the to_short_string_with_why_not why_not = true.
    #[test]
    fn test_to_short_string_with_why_not_why_not_true() {
        let in_why_not_bool = true;
        let tags: Vec<String> = vec!["tag1".to_string(), "tag2".to_string()];
        let references: Vec<String> = vec!["ref1".to_string(), "ref2".to_string()];
        let why_not: Vec<String> = vec!["why_not1".to_string(), "why_not2".to_string()];
        let template = Template::new(
            "id".to_string(),
            "id_search".to_string(),
            "title".to_string(),
            "title_search".to_string(),
            tags.clone(),
            tags.clone(),
            "description".to_string(),
            "description_search".to_string(),
            references.clone(),
            references.clone(),
            why_not.clone(),
        );

        // Compare the template to string output with the expected output
        let expected_output: &str =
            "id title\n    tag1, tag2\n    description\n    ref1\n    ref2";
        assert_eq!(template.to_string(in_why_not_bool), expected_output);
    }

    // Test the tags_to_string function against the expected values.
    #[test]
    fn test_tags_to_string() {
        let tags: Vec<String> = vec!["tag1".to_string(), "tag2".to_string()];
        let references: Vec<String> = vec!["ref1".to_string(), "ref2".to_string()];
        let why_not: Vec<String> = vec!["why_not1".to_string(), "why_not2".to_string()];
        let template = Template::new(
            "id".to_string(),
            "id_search".to_string(),
            "title".to_string(),
            "title_search".to_string(),
            tags.clone(),
            tags.clone(),
            "description".to_string(),
            "description_search".to_string(),
            references.clone(),
            references.clone(),
            why_not.clone(),
        );

        // Compare the return string with the expected string.
        let expected_output: &str = "tag1, tag2";
        assert_eq!(template.tags_to_string(), expected_output);
    }

    // Test the references_to_string function against the expected values.
    #[test]
    fn test_references_to_string() {
        let tags: Vec<String> = vec!["tag1".to_string(), "tag2".to_string()];
        let references: Vec<String> = vec!["ref1".to_string(), "ref2".to_string()];
        let why_not: Vec<String> = vec!["why_not1".to_string(), "why_not2".to_string()];
        let template = Template::new(
            "id".to_string(),
            "id_search".to_string(),
            "title".to_string(),
            "title_search".to_string(),
            tags.clone(),
            tags.clone(),
            "description".to_string(),
            "description_search".to_string(),
            references.clone(),
            references.clone(),
            why_not.clone(),
        );

        // Compare the return string with the expected string.
        let expected_output: &str = "    ref1\n    ref2";
        assert_eq!(template.references_to_string(), expected_output);
    }

    #[test]
    fn test_why_not_to_string() {
        let tags: Vec<String> = vec!["tag1".to_string(), "tag2".to_string()];
        let references: Vec<String> = vec!["ref1".to_string(), "ref2".to_string()];
        let why_not: Vec<String> = vec!["why_not1".to_string(), "why_not2".to_string()];
        let template = Template::new(
            "id".to_string(),
            "id_search".to_string(),
            "title".to_string(),
            "title_search".to_string(),
            tags.clone(),
            tags.clone(),
            "description".to_string(),
            "description_search".to_string(),
            references.clone(),
            references.clone(),
            why_not.clone(),
        );

        // Compare the return string with the expected string.
        let expected_output: &str = "Why Not why_not1\nWhy Not why_not2";
        assert_eq!(template.why_not_to_string(), expected_output);
    }

    // Test the function convert_to_lowercase_alphanumeric_with_hyphens
    #[test]
    fn test_convert_to_lowercase_alphanumeric_with_hyphens() {
        let input_string = "Convert-ME)into*Lowercase!".to_string();
        let expected_output = "convert-meintolowercase".to_string();

        assert_eq!(
            Template::convert_to_lowercase_alphanumeric_with_hyphens(&input_string),
            expected_output
        );
    }

    // Test the function convert_tags_to_excepted_format
    #[test]
    fn test_convert_tags_to_excepted_format() {
        let input_tags = vec!["ConvertME".to_string(), "Into-Lowercase!".to_string()];
        let expected_output = vec!["convertme".to_string(), "into-lowercase".to_string()];

        assert_eq!(
            Template::convert_tags_to_excepted_format(&input_tags),
            expected_output
        );
    }
}
