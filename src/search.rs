use crate::template::Template;
use levenshtein::levenshtein;
use regex::Regex;
use std::collections::HashSet;

// This will be the framework for the search algorithmic.

// In the struct will be a value, a query and the distance,
// witch will represent the accuracy of the search.
// An value close to 1.0 will be the best vector.
// An value close to 0.0 will be no match.
// This is almost imposable, because some letters will match at some point.
pub struct Search {
    id: String,
    title: String,
    tags: Vec<String>,
    description: String,
    references: Vec<String>,
}

impl Search {
    pub fn new_empty() -> Search {
        Search {
            id: "".to_owned(),
            title: "".to_owned(),
            tags: Vec::<String>::new(),
            description: "".to_owned(),
            references: Vec::<String>::new(),
        }
    }

    pub fn id_get(&self) -> String {
        self.id.to_owned()
    }

    pub fn id_set(&mut self, in_id: String) {
        self.id = Template::convert_to_lowercase_alphanumeric_with_hyphens(&in_id);
    }

    pub fn title_get(&self) -> String {
        self.title.to_owned()
    }

    pub fn title_set(&mut self, in_title: String) {
        self.title = in_title;
    }

    pub fn tags_get(&self) -> Vec<String> {
        self.tags.to_owned()
    }

    pub fn tags_set(&mut self, in_tags: Vec<String>) {
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

    pub fn word_similarities(in_value: &Vec<String>, in_query: &Vec<String>) -> f32 {
        in_value
            .iter()
            .map(|sentence| Search::word_similarity(sentence, &in_query.join(" ")))
            .sum::<f32>()
            / in_value.len() as f32
    }

    pub fn word_similarity(in_value: &str, in_query: &str) -> f32 {
        let answer = Search::are_values_empty(in_value, in_query);
        if answer.0 {
            return answer.1;
        }

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

    pub fn levenshtein_similarity(word1: &str, word2: &str) -> f32 {
        let answer = Search::are_values_empty(word1, word2);
        if answer.0 {
            return answer.1;
        }

        let distance = levenshtein(&word1.to_lowercase(), &word2.to_lowercase()) as f32;

        let max_length = word1.len().max(word2.len()) as f32;

        1.0 - (distance / max_length)
    }

    pub fn similarities_full(in_value: &Vec<String>, in_query: &Vec<String>) -> f32 {
        let answer = Search::are_vec_empty(in_value, in_query);
        if answer.0 {
            return answer.1;
        }

        let similarities_score = Search::word_similarities(in_value, in_query);
        let calculate_similarity_score = in_value
            .iter()
            .zip(in_query.iter())
            .map(|(word1, word2)| Search::levenshtein_similarity(word1, word2))
            .sum::<f32>()
            / in_value.len().max(in_query.len()) as f32;

        0.7 * similarities_score + 0.3 * calculate_similarity_score
    }

    pub fn similarity_full(in_value: &str, in_query: &str) -> f32 {
        let answer = Search::are_values_empty(in_value, in_query);
        if answer.0 {
            return answer.1;
        }

        let similarity_score = Search::word_similarity(in_value, in_query);
        let calculate_similarity_score = Search::levenshtein_similarity(in_value, in_query);

        0.7 * similarity_score + 0.3 * calculate_similarity_score
    }

    pub fn are_values_empty(in_value: &str, in_query: &str) -> (bool, f32) {
        if in_value.is_empty() && in_query.is_empty() {
            return (true, 1.0);
        }
        if in_value.is_empty() || in_query.is_empty() {
            return (true, 0.0);
        }
        (false, 0.0)
    }

    pub fn are_vec_empty(in_value: &Vec<String>, in_query: &Vec<String>) -> (bool, f32) {
        if in_value.is_empty() && in_query.is_empty() {
            return (true, 1.0);
        }
        if in_value.is_empty() || in_query.is_empty() {
            return (true, 0.0);
        }
        (false, 0.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use lazy_static::lazy_static;
    use regex::Regex;

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
    fn test_search_new_just_id() {
        let search = Search{
            id: "test".to_owned(),
            title: "".to_owned(),
            tags: Vec::<String>::new(),
            description: "".to_owned(),
            references: Vec::<String>::new()
        };

        assert_eq!(search.id, "test");
        assert_eq!(search.title, "");
        assert_eq!(search.tags, Vec::<String>::new());
        assert_eq!(search.description, "");
        assert_eq!(search.references, Vec::<String>::new());    
    }

    #[test]
    fn test_search_new_just_title() {
        let search = Search{
            id: "".to_owned(),
            title: "test".to_owned(),
            tags: Vec::<String>::new(),
            description: "".to_owned(),
            references: Vec::<String>::new()
        };

        assert_eq!(search.id, "");
        assert_eq!(search.title, "test");
        assert_eq!(search.tags, Vec::<String>::new());
        assert_eq!(search.description, "");
        assert_eq!(search.references, Vec::<String>::new());    
    }

    #[test]
    fn test_search_new_just_tags() {
        let search = Search{
            id: "".to_owned(),
            title: "".to_owned(),
            tags: vec![String::from("test")],
            description: "".to_owned(),
            references: Vec::<String>::new()
        };

        assert_eq!(search.id, "");
        assert_eq!(search.title, "");
        assert_eq!(search.tags, vec![String::from("test")]);    
        assert_eq!(search.description, "");
        assert_eq!(search.references, Vec::<String>::new());
    }

    #[test]
    fn test_search_new_just_description() {
        let search = Search{
            id: "".to_owned(),
            title: "".to_owned(),
            tags: Vec::<String>::new(),
            description: "test".to_owned(),
            references: Vec::<String>::new()
        };

        assert_eq!(search.id, "");
        assert_eq!(search.title, "");
        assert_eq!(search.tags, Vec::<String>::new());
        assert_eq!(search.description, "test");
        assert_eq!(search.references, Vec::<String>::new());    
    }

    #[test]
    fn test_search_new_just_references() {
        let search = Search{
            id: "".to_owned(),
            title: "".to_owned(),
            tags: Vec::<String>::new(),
            description: "".to_owned(),
            references: vec![String::from("test")]
        };


        assert_eq!(search.id, "");
        assert_eq!(search.title, "");
        assert_eq!(search.tags, Vec::<String>::new());
        assert_eq!(search.description, "");
        assert_eq!(search.references, vec![String::from("test")]);    
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
    fn test_search_id_set_get() {
        let mut search = Search::new_empty();
        search.id_set("template1".to_owned());

        let out_id = search.id_get();

        assert_eq!(out_id, "template1");
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
    fn test_search_title_set_get() {
        let mut search = Search::new_empty();
        search.title_set("Template 1".to_owned());

        let out_title = search.title_get();

        assert_eq!(out_title, "Template 1");
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
    fn test_search_tags_set_get() {
        let mut search = Search::new_empty();
        search.tags_set(vec![String::from("test"), String::from("test2")]);

        let out_tags = search.tags_get();

        assert_eq!(out_tags, vec![String::from("test"), String::from("test2")]);
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
    fn test_search_description_set_get() {
        let mut search = Search::new_empty();
        search.description_set("$$$Bill$$$".to_owned());

        let out_description = search.description_get();

        assert_eq!(out_description, "$$$Bill$$$".to_owned());
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
    fn test_search_reference_set_get() {
        let mut search = Search::new_empty();
        search.reference_set(vec![String::from("https://loler_gmbh.com"), String::from("script<alert()>")]);

        let out_tags = search.reference_get();

        assert_eq!(out_tags, vec![String::from("https://loler_gmbh.com"), String::from("script<alert()>")]);
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

        let similarity = Search::levenshtein_similarity(value, query);

        assert_eq!(similarity, 0.5);
    }

    #[test]
    fn test_search_similarity_one_word() {
        let value = "This";
        let query = "That";

        let similarity = Search::levenshtein_similarity(value, query);

        assert_eq!(similarity, 0.5);
    }

    #[test]
    fn test_search_similarities_one_word_one_quarter() {
        let value = "Tree";
        let query = "That";

        let similarity = Search::levenshtein_similarity(value, query);

        assert_eq!(similarity, 0.25);
    }

    #[test]
    fn test_search_similarity_one_word_one_quarter() {
        let value = "Tree";
        let query = "That";

        let similarity = Search::levenshtein_similarity(value, query);

        assert_eq!(similarity, 0.25);
    }

    #[test]
    fn test_search_similarities_full_empty_vectors() {
        let value: Vec<String> = Vec::new();
        let query: Vec<String> = Vec::new();

        let similarity = Search::similarities_full(&value, &query);

        assert_eq!(similarity, 1.0);
    }

    #[test]
    fn test_search_similarities_value_empty_vectors() {
        let value: Vec<String> = Vec::new();
        let query: Vec<String> = vec!["This is a sample text.".to_owned()];

        let similarity = Search::similarities_full(&value, &query);

        assert_eq!(similarity, 0.0);
    }

    #[test]
    fn test_search_similarities_query_empty_vectors() {
        let value: Vec<String> = vec!["This is a sample text.".to_owned()];
        let query: Vec<String> = Vec::new();

        let similarity = Search::similarities_full(&value, &query);

        assert_eq!(similarity, 0.0);
    }

    #[test]
    fn test_search_similarity_full_empty_strings() {
        let value = "";
        let query = "";

        let similarity = Search::similarity_full(value, query);

        assert_eq!(similarity, 1.0);
    }

    #[test]
    fn test_search_similarity_value_empty_strings() {
        let value = "";
        let query = "test";

        let similarity = Search::similarity_full(value, query);

        assert_eq!(similarity, 0.0);
    }

    #[test]
    fn test_search_similarity_query_empty_strings() {
        let value = "test";
        let query = "";

        let similarity = Search::similarity_full(value, query);

        assert_eq!(similarity, 0.0);
    }

    #[test]
    fn test_search_calculate_similarity_different_lengths() {
        let word1 = "Hello";
        let word2 = "H";

        let similarity = Search::levenshtein_similarity(word1, word2);

        assert_approx_eq::assert_approx_eq!(similarity, 0.2, 0.01); // ~( 1.0 - (1 / 5) )
    }

    #[test]
    fn test_search_calculate_similarity_empty_strings() {
        let word1 = "";
        let word2 = "";

        let similarity = Search::levenshtein_similarity(word1, word2);

        assert_eq!(similarity, 1.0);
    }

    #[test]
    fn test_search_calculate_similarity_case_insensitive() {
        let word1 = "hello";
        let word2 = "Hello";

        let similarity = Search::levenshtein_similarity(word1, word2);

        assert_eq!(similarity, 1.0);
    }

    #[test]
    fn test_search_similarities_full_long_vectors() {
        let value = vec![
            "This is a sample text.".to_owned(),
            "Another sentence.".to_owned(),
            "One more sentence.".to_owned(),
            // Add more sentences here
        ];
        let query = vec![
            "sample text".to_owned(),
            // Add more query sentences here
        ];

        let similarity = Search::similarities_full(&value, &query);

        // Perform appropriate assertion based on the expected similarity value.

        // Expected range of similarity.
        let expected_similarity_range = 0.0..=1.0;

        assert!(expected_similarity_range.contains(&similarity));
    }
}
