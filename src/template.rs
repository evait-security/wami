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
    // because I will trance form the id and tags fields to the right form.
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
        let out_id: String = Template::convert_to_lowercase_alphanumeric_with_hyphens(&in_id.to_owned());
        
        // Convert search unique name to lowercase alphanumeric letters with hyphens.
        let out_id_search: String = Template::convert_to_lowercase_alphanumeric_with_hyphens(&in_id_search);
        
        // Calculate the similarity from the unique name to the unique search name.
        let out_id_distance: f32 = search::Search::similarity(&out_id, &out_id_search);
        
        // Calculate the similarity from the real title to the search title.
        let out_title_distance: f32 = search::Search::similarity(&in_title, &in_title_search);
        
        // Convert the incoming tags to lowercase alphanumeric letters with hyphens.
        let out_tags: Vec<String> = Template::convert_tags_to_excepted_format(in_tags);

        // Convert the incoming search tags to lowercase alphanumeric letter with hyphens.
        let out_tags_search: Vec<String> = Template::convert_tags_to_excepted_format(in_tags_search);
        
        // Calculate the similarities from the tags to the search tags.
        let out_tags_distance: f32 = search::Search::similarities(&out_tags, &out_tags_search);
        
        // Calculate the similarity from the description to the search description.
        let out_description_distance: f32 = search::Search::similarity(&in_description, &in_description_search);
        
        // Calculate the similarity from the references to the search references.
        let out_references_distance: f32 = search::Search::similarities(&in_references, &in_references_search);
        
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
            if index < reference_len {
                out_string.push_str("\n");
            }
        }

        out_string
    }

    // Make the tags vec robust
    // Will make all tags uniform, this simplifies the search algorithmic.
    pub fn convert_tags_to_excepted_format(in_tags: Vec<String>) -> Vec<String> {
        let mut out_tags: Vec<String> = Vec::<String>::new();
        for tag in in_tags {
            out_tags.push(
                Template::convert_to_lowercase_alphanumeric_with_hyphens(&tag)
            );
        }

        out_tags
    }

    // Make the string robust.
    // If there is an error in the String, it will be converted or deleted.
    // See the requirements for the unique name and tags fields.
    // Only lowercase alphanumeric letters or hyphens are allowed.
    pub fn convert_to_lowercase_alphanumeric_with_hyphens(in_str: &String) -> String {
        let mut out_string = String::new();

        for c in in_str.chars() {
            if c.is_ascii_alphanumeric() || c == '-' {
                out_string.push(c.to_ascii_lowercase());
            }
        }

        out_string
    }
}