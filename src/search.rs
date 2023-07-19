use std::collections::HashSet;
use regex::Regex;
use levenshtein::levenshtein;
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

    pub fn calculate_similarity(word1: &str, word2: &str) -> f32 {
        let distance = levenshtein(word1, word2) as f32;
        let max_length = word1.len().max(word2.len()) as f32;

        1.0 - (distance / max_length)
    }

    pub fn similarities_full(in_value: &Vec<String>, in_query: &Vec<String>) -> f32 {
        let similarities_score = Search::similarities(in_value, in_query);
        let calculate_similarity_score = in_value.iter().zip(in_query.iter())
            .map(|(word1, word2)| Search::calculate_similarity(word1, word2))
            .sum::<f32>() / in_value.len().max(in_query.len()) as f32;
    
        0.7 * similarities_score + 0.3 * calculate_similarity_score
    }
    
    pub fn similarity_full(in_value: &str, in_query: &str) -> f32 {
        let similarity_score = Search::similarity(in_value, in_query);
        let calculate_similarity_score = Search::calculate_similarity(in_value, in_query);
    
        0.7 * similarity_score + 0.3 * calculate_similarity_score
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use regex::Regex;
    use lazy_static::lazy_static;

    lazy_static! {
        static ref WORD_REGEX: Regex = Regex::new(r"\b(\w+)\b").unwrap();
    }
    
    #[test]
    fn test_search_new_empty() {
        let search = Search::new_empty();
        
        assert_eq!(search.id, "");
        assert_eq!(search.title, "");
        assert_eq!(search.tags, Vec::<String>::new());
        assert_eq!(search.description, "");
        assert_eq!(search.references, Vec::<String>::new());
    }
    
    #[test]
    fn test_search_id_get() {
        let mut search = Search::new_empty();
        search.id = "template1".to_owned();
        
        assert_eq!(search.id_get(), "template1");
    }
    
    #[test]
    fn test_search_id_set() {
        let mut search = Search::new_empty();
        search.id_set("template1".to_owned());
        
        assert_eq!(search.id, "template1");
    }
    
    #[test]
    fn test_search_title_get() {
        let mut search = Search::new_empty();
        search.title = "Template 1".to_owned();
        
        assert_eq!(search.title_get(), "Template 1");
    }
    
    #[test]
    fn test_search_title_set() {
        let mut search = Search::new_empty();
        search.title_set("Template 1".to_owned());
        
        assert_eq!(search.title, "Template 1");
    }
    
    #[test]
    fn test_search_tags_get() {
        let mut search = Search::new_empty();
        search.tags = vec!["tag1".to_owned(), "tag2".to_owned()];
        
        assert_eq!(search.tags_get(), vec!["tag1", "tag2"]);
    }
    
    #[test]
    fn test_search_tags_set() {
        let mut search = Search::new_empty();
        search.tags_set(vec!["tag1".to_owned(), "tag2".to_owned()]);
        
        assert_eq!(search.tags, vec!["tag1", "tag2"]);
    }
    
    #[test]
    fn test_search_description_get() {
        let mut search = Search::new_empty();
        search.description = "This is a sample description.".to_owned();
        
        assert_eq!(search.description_get(), "This is a sample description.");
    }
    
    #[test]
    fn test_search_description_set() {
        let mut search = Search::new_empty();
        search.description_set("This is a sample description.".to_owned());
        
        assert_eq!(search.description, "This is a sample description.");
    }
    
    #[test]
    fn test_search_reference_get() {
        let mut search = Search::new_empty();
        search.references = vec!["https://example.com".to_owned()];
        
        assert_eq!(search.reference_get(), vec!["https://example.com"]);
    }
    
    #[test]
    fn test_search_reference_set() {
        let mut search = Search::new_empty();
        search.reference_set(vec!["https://example.com".to_owned()]);
        
        assert_eq!(search.references, vec!["https://example.com"]);
    }
    
    #[test]
    fn test_search_similarities() {
        let value = vec!["This is a sample text.".to_owned()];
        let query = vec!["sample text".to_owned()];

        let words_value: HashSet<&str> = value
            .iter()
            .flat_map(|sentence| WORD_REGEX.captures_iter(sentence))
            .map(|captures| captures.get(1).unwrap().as_str())
            .collect();

        let words_query: HashSet<&str> = query
            .iter()
            .flat_map(|sentence| WORD_REGEX.captures_iter(sentence))
            .map(|captures| captures.get(1).unwrap().as_str())
            .collect();

        println!("Words in value: {:?}", words_value);
        println!("Words in query: {:?}", words_query);

        let intersection = words_value.intersection(&words_query).count();
        let union = words_value.len() + words_query.len() - intersection;

        println!("Intersection: {}", intersection);
        println!("Union: {}", union);

        let similarity = intersection as f32 / union as f32;

        assert_eq!(similarity, 0.4);
    }

    #[test]
    fn test_search_similarity() {
        let value = "This is a sample text.";
        let query = "sample text";

        let words_value: HashSet<&str> = WORD_REGEX
            .captures_iter(value)
            .map(|captures| captures.get(1).unwrap().as_str())
            .collect();

        let words_query: HashSet<&str> = WORD_REGEX
            .captures_iter(query)
            .map(|captures| captures.get(1).unwrap().as_str())
            .collect();

        println!("Words in value: {:?}", words_value);
        println!("Words in query: {:?}", words_query);

        let intersection = words_value.intersection(&words_query).count();
        let union = words_value.len() + words_query.len() - intersection;

        println!("Intersection: {}", intersection);
        println!("Union: {}", union);

        let similarity = intersection as f32 / union as f32;

        assert_eq!(similarity, 0.4);
    }

    #[test]
    fn test_search_similarities_one_word() {
        let value = "This";
        let query = "That";

        let similarity = Search::calculate_similarity(value, query);

        assert_eq!(similarity, 0.5);
    }

    #[test]
    fn test_search_similarity_one_word() {
        let value = "This";
        let query = "That";

        let similarity = Search::calculate_similarity(value, query);

        assert_eq!(similarity, 0.5);
    }

    #[test]
    fn test_search_similarities_one_word_one_quarter() {
        let value = "Tree";
        let query = "That";

        let similarity = Search::calculate_similarity(value, query);

        assert_eq!(similarity, 0.25);
    }

    #[test]
    fn test_search_similarity_one_word_one_quarter() {
        let value = "Tree";
        let query = "That";

        let similarity = Search::calculate_similarity(value, query);

        assert_eq!(similarity, 0.25);
    }
}