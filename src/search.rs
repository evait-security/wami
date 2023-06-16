use std::collections::HashSet;
use regex::Regex;

use crate::template::Template;


// This will be the framework for the search algorithmic.

// In the struct will be a value, a query and the distance, 
// witch will represent the accuracy of the search.
// An value close to 1.0 will be the best vector.
// An value close to 0.0 will be no match.
// This is almost imposable, because some letters will match at some point.
pub struct Search<> {
    id: String,
    title: String,
    tags: Vec<String>,
    description: String,
    references: Vec<String>
}

impl Search {

    // pub fn new(
    //     in_id: String,
    //     in_title: String,
    //     in_tags: Vec<String>,
    //     in_description: String,
    //     in_references: Vec<String>
    // ) -> Search {
    //     Search{
    //         id: Template::convert_to_lowercase_alphanumeric_with_hyphens(&in_id),
    //         title: in_title.to_owned(),
    //         tags: Template::convert_tags_to_excepted_format(in_tags),
    //         description: in_description.to_owned(),
    //         references: in_references.to_owned()
    //     }
    // }

    pub fn new_empty() -> Search {
        Search { 
            id: "".to_owned(),
            title: "".to_owned(),
            tags: Vec::<String>::new(),
            description: "".to_owned(),
            references: Vec::<String>::new()
        }
    }

    pub fn id_get(&self) -> String{
        self.id.to_owned()
    }

    pub fn id_set(&mut self, in_id: String){
        self.id = Template::convert_to_lowercase_alphanumeric_with_hyphens(&in_id);
    }

    pub fn title_get(&self) -> String{
        self.title.to_owned()
    }

    pub fn title_set(&mut self, in_title: String) {
        self.title = in_title;
    }

    pub fn tags_get(&self) -> Vec<String> {
        self.tags.to_owned()
    }

    pub fn tags_set(&mut self, in_tags: Vec<String>){
        self.tags = Template::convert_tags_to_excepted_format(in_tags);
    }

    pub fn description_get(&self) -> String {
        self.description.to_owned()
    }

    pub fn description_set(&mut self, in_description: String) {
        self.description = in_description.to_owned();
    }

    pub fn reference_get(&self) -> Vec<String> {
        self.references.to_owned()
    }

    pub fn reference_set(&mut self, in_reference: Vec<String>) {
        self.references = in_reference.to_owned();
    }

    pub fn similarities(in_value: &Vec<String>, in_query: &Vec<String>) -> f32 {
        let word_regex = Regex::new(r"\b(\w+)\b").unwrap();
    
        let words_value: HashSet<&str> = in_value.iter()
            .flat_map(|sentence| word_regex.captures_iter(sentence))
            .map(|captures| captures.get(1).unwrap().as_str())
            .collect();
    
        let words_query: HashSet<&str> = in_query.iter()
            .flat_map(|sentence| word_regex.captures_iter(sentence))
            .map(|captures| captures.get(1).unwrap().as_str())
            .collect();
    
        let intersection = words_value.intersection(&words_query).count();
        let union = words_value.len() + words_query.len() - intersection;
    
        let similarity = intersection as f32 / union as f32;
    
        similarity
    }

    pub fn similarity(in_value: &str, in_query: &str) -> f32 {
        let word_regex = Regex::new(r"\b(\w+)\b").unwrap();
    
        let words_value: HashSet<&str> = word_regex
            .captures_iter(in_value)
            .map(|captures| captures.get(1).unwrap().as_str())
            .collect();
    
        let words_query: HashSet<&str> = word_regex
            .captures_iter(in_query)
            .map(|captures| captures.get(1).unwrap().as_str())
            .collect();
    
        let intersection = words_value.intersection(&words_query).count();
        let union = words_value.len() + words_query.len() - intersection;
    
        let similarity = intersection as f32 / union as f32;
    
        similarity
    }
}