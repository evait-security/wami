use serde::Deserialize;
use crate::search;

#[derive(Deserialize, Debug)]
pub struct Template {
    id: String, // A unique name in lower case letters.
    title: String, // The real name of the tool / software.
    tags: Vec<String>, // An vector of lower case tags for the search function.
    description: String, // A longer text describing the too and what it does. 
    references: Vec<String>, // Links to websites, repositories, or other resources.
    distance: f32, // This will set the average similarity to the search value.
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
        in_references_search: Vec<String>
    ) -> Template{
        // Convert the incoming unique name to lowercase alphanumeric letters with hyphens.
        let out_id: String = Template::convert_to_lowercase_alphanumeric_with_hyphens(&in_id);
        
        // Convert search unique name to lowercase alphanumeric letters with hyphens.
        let out_id_search: String = Template::convert_to_lowercase_alphanumeric_with_hyphens(&in_id_search);
        
        // Calculate the similarity from the unique name to the unique search name.
        let out_id_distance: f32 = search::Search::similarity_full(&out_id, &out_id_search);
        
        // Calculate the similarity from the real title to the search title.
        let out_title_distance: f32 = search::Search::similarity_full(&in_title, &in_title_search);
        
        // Convert the incoming tags to lowercase alphanumeric letters with hyphens.
        let out_tags: Vec<String> = Template::convert_tags_to_excepted_format(&in_tags);

        // Convert the incoming search tags to lowercase alphanumeric letter with hyphens.
        let out_tags_search: Vec<String> = Template::convert_tags_to_excepted_format(&in_tags_search);
        
        // Calculate the similarities from the tags to the search tags.
        let out_tags_distance: f32 = search::Search::similarities_full(&out_tags, &out_tags_search);
        
        // Calculate the similarity from the description to the search description.
        let out_description_distance: f32 = search::Search::similarity_full(&in_description, &in_description_search);
        
        // Calculate the similarity from the references to the search references.
        let out_references_distance: f32 = search::Search::similarities_full(&in_references, &in_references_search);
        
        // Calculate the over all similarity. This is done by adding all 5 distances together and multiplying them by 0.2.
        let out_distance: f32 = (out_id_distance + out_title_distance + out_tags_distance + out_description_distance + out_references_distance) * 0.2;
        
        // Creating and returning the new Template
        Template {
            id: out_id,                                         // The unique name of the template
            // id_distance: out_id_distance,                       // The similarity to the search unique name
            title: in_title,                                    // The title of the template
            // title_distance: out_title_distance,                 // The similarity to the search title
            tags: out_tags,                                     // The tags of the template
            // tags_distance: out_tags_distance,                   // The similarities to the search tags
            description: in_description,                        // The description of the template
            // description_distance: out_description_distance,     // The similarity to the search description
            references: in_references,                          // The references of the template
            // references_distance: out_references_distance,       // The similarity to the search references
            distance: out_distance                              // The average similarity of all similarities
        }
    }

    pub fn distance(&self) -> f32 {
        self.distance
    }

    // This will return a string, for the console.
    pub fn to_string(&self) -> String {
        let mut out_string: String = String::new();
        out_string.push_str(&self.id);
        out_string.push_str("\n  Name: ");
        out_string.push_str(&self.title);
        out_string.push_str("\n  Tags: ");
        out_string.push_str(&Template::tags_to_string(&self));
        out_string.push_str("\n  Description: ");
        out_string.push_str(&self.description);
        out_string.push_str("\n  References: \n");
        out_string.push_str(&Template::references_to_string(&self));
        out_string.push_str("\n");

        out_string.to_owned()
    }

    pub fn to_short_string(&self) -> String {
        let mut out_string: String = String::new();
        out_string.push_str(&self.title);
        out_string.push_str("\n");
        out_string.push_str(&self.references_to_string());
        out_string.to_owned()
    }

    // Put all tags in a line and separate them with an , except the last own.
    fn tags_to_string(&self) -> String {
        let mut out_string: String = String::new();
        let tags_len = self.tags.len();
        for (index, tag) in self.tags.iter().enumerate() {
            out_string.push_str(tag);
            if index < tags_len {
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

    // // Make the tags vec robust
    // // Will make all tags uniform, this simplifies the search algorithmic.
    // pub fn convert_tags_to_excepted_format(in_tags: Vec<String>) -> Vec<String> {
    //     let mut out_tags: Vec<String> = Vec::<String>::new();
    //     for tag in in_tags {
    //         out_tags.push(
    //             Template::convert_to_lowercase_alphanumeric_with_hyphens(&tag)
    //         );
    //     }

    //     out_tags
    // }

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
            .map(|c| c
                .to_ascii_lowercase()
                .to_string())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Tests for the Template structure
    #[test]
    fn test_template_structure() {
        let tags = vec!["tag1".to_string(), "tag2".to_string()];
        let references = vec!["ref1".to_string(), "ref2".to_string()];
        let search_vec: Vec<String> = vec!["search2".to_string(), "search3".to_string()];
        let template = Template::new(
            "id".to_string(),
            "id_search".to_string(),
            "title".to_string(),
            "title_search".to_string(),
            tags.clone(),
            search_vec.clone(),
            "description".to_string(),
            "description_search".to_string(),
            references.clone(),
            search_vec.clone()
        );
        
        assert_eq!(template.id, "id");
        assert_eq!(template.title, "title");
        assert_eq!(template.tags, tags);
        assert_eq!(template.description, "description");
        assert_eq!(template.references, references);
    }

    // Test for the new function
    #[test]
    fn test_new() {
        let tags = vec!["tag1".to_string(), "tag2".to_string()];
        let references = vec!["ref1".to_string(), "ref2".to_string()];
        let search_vec: Vec<String> = vec!["search2".to_string(), "search3".to_string()];
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
            search_vec.clone()
        );

        // The id and tags should be in lowercase
        assert_eq!(template.id, "id");
        assert_eq!(template.title, "Title");
        assert_eq!(template.tags, vec!["tag1", "tag2"]);
        assert_eq!(template.description, "Description");
        assert_eq!(template.references, vec!["ref1", "ref2"]);
    }

    // Tests for the to_string function
    #[test]
    fn test_to_string() {
        let tags = vec!["tag1".to_string(), "tag2".to_string()];
        let references = vec!["ref1".to_string(), "ref2".to_string()];
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
            references.clone()
        );

        // Compare the template to string output with the expected output
        let expected_output = "id\n  Name: title\n  Tags: tag1, tag2, \n  Description: description\n  References: \n    ref1\n    ref2\n";
        assert_eq!(template.to_string(), expected_output);
    }

    // Test the function convert_to_lowercase_alphanumeric_with_hyphens
    #[test]
    fn test_convert_to_lowercase_alphanumeric_with_hyphens() {
        let input_string = "Convert-ME)into*Lowercase!".to_string();
        let expected_output = "convert-meintolowercase".to_string();

        assert_eq!(Template::convert_to_lowercase_alphanumeric_with_hyphens(&input_string), expected_output);
    }

    // Test the function convert_tags_to_excepted_format
    #[test]
    fn test_convert_tags_to_excepted_format() {
        let input_tags = vec!["ConvertME".to_string(), "Into-Lowercase!".to_string()];
        let expected_output = vec!["convertme".to_string(), "into-lowercase".to_string()];

        assert_eq!(Template::convert_tags_to_excepted_format(&input_tags), expected_output);
    }
}