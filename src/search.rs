use crate::template::Template;
use levenshtein::levenshtein;

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
    min_stars: isize,
    max_stars: isize,
    github_fork: bool,
}

impl Search {
    pub fn new_empty() -> Search {
        Search {
            id: "".to_owned(),
            title: "".to_owned(),
            tags: Vec::new(),
            description: "".to_owned(),
            references: Vec::new(),
            min_stars: 100,
            max_stars: -1, // The -1 will set the max value to an open end.
            github_fork: false,
        }
    }
    
    pub fn id_get(&self) -> &String {
        &self.id
    }

    pub fn id_set(&mut self, in_id: &String) {
        self.id = Template::convert_to_lowercase_alphanumeric_with_hyphens(&in_id);
    }

    pub fn title_get(&self) -> &str {
        &self.title
    }

    pub fn title_set(&mut self, in_title: &String) {
        self.title = in_title.to_owned();
    }

    pub fn tags_get(&self) -> &Vec<String> {
        &self.tags
    }

    pub fn tags_set(&mut self, in_tags: &Vec<String>) {
        self.tags = in_tags.to_owned();
    }

    pub fn description_get(&self) -> &String {
        &self.description
    }

    pub fn description_set(&mut self, in_description: &String) {
        self.description = in_description.to_owned();
    }

    pub fn reference_get(&self) -> &Vec<String> {
        &self.references
    }

    pub fn reference_set(&mut self, in_reference: &Vec<String>) {
        self.references = in_reference.to_owned();
    }

    pub fn min_stars_get(&self) -> &isize {
        &self.min_stars
    }

    pub fn min_stars_set(&mut self, in_min_stars: isize) {
        self.min_stars = in_min_stars.to_owned();
    }

    pub fn max_stars_get(&self) -> &isize {
        &self.max_stars
    }

    pub fn max_stars_set(&mut self, in_max_stars: isize) {
        self.max_stars = in_max_stars.to_owned();
    }

    pub fn github_fork_get(&self) -> &bool {
        &self.github_fork
    }

    pub fn github_fork_set(&mut self, in_github_fork: bool) {
        self.github_fork = in_github_fork.to_owned();
    }

    fn word_similarities(in_value: &[String], in_query: &[String]) -> f32 {
        in_value
            .iter()
            .map(|sentence| 
                Search::word_similarity(
                    &sentence, 
                    &in_query
                        .join(" ")
                )
            )
            .sum::<f32>()
            / in_value.len() as f32
    }

    pub fn word_similarity(in_value: &str, in_query: &str) -> f32 {
        let (is_empty, similarity) = Search::are_values_empty(&in_value, &in_query);
        if is_empty {
            return similarity;
        }

        let value: &str = &Template::convert_to_lowercase_alphanumeric_with_hyphens(&in_value.to_string());
        let query: &str = &Template::convert_to_lowercase_alphanumeric_with_hyphens(&in_query.to_string());

        if Search::are_values_unequal_long(&value, &query){
            return 0.0;
        }

        if Search::are_values_equal(&value, &query) {
            return 1.0
        }

        0.0
    }

    pub fn levenshtein_similarity(word1: &str, word2: &str) -> f32 {
        let (is_empty, similarity) = Search::are_values_empty(&word1, &word2);
        if is_empty {
            return similarity;
        }

        let distance = levenshtein(&word1.to_lowercase(), &word2.to_lowercase()) as f32;

        let max_length = word1.len().max(word2.len()) as f32;

        1.0 - (distance / max_length)
    }

    pub fn similarities_full(in_value: &Vec<String>, in_query: &Vec<String>) -> f32 {
        let (is_empty, similarity) = Search::are_vec_empty(&in_value, &in_query);
        if is_empty {
            return similarity;
        }
       
        let similarities_score = Search::word_similarities(&in_value, &in_query);

        let calculate_similarity_score = in_value
            .iter()
            .zip(in_query.iter())
            .map(|(word1, word2)| Search::levenshtein_similarity(&word1, &word2))
            .sum::<f32>()
            / in_value.len().max(in_query.len()) as f32;

        0.7 * similarities_score + 0.3 * calculate_similarity_score
    }

    pub fn similarity_full(in_value: &str, in_query: &str) -> f32 {
        let (is_empty, similarity) = Search::are_values_empty(&in_value, &in_query);
        if is_empty {
            return similarity;
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

    pub fn are_values_unequal_long(in_value: &str, in_query: &str) -> bool {
        if in_value.len() != in_query.len() {
            return true;
        }
        false
    }

    pub fn are_values_equal(in_value: &str, in_query: &str) -> bool {
        if in_value == in_query {
            return true;
        }
        false
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
        assert_eq!(search.min_stars, 100);
        assert_eq!(search.max_stars, -1);
        assert_eq!(search.github_fork, false);
    }

    #[test]
    fn test_search_new_just_id() {
        let search = Search{
            id: "test".to_owned(),
            title: "".to_owned(),
            tags: Vec::new(),
            description: "".to_owned(),
            references: Vec::new(),
            min_stars: 100,
            max_stars: 0,
            github_fork: true,
        };

        assert_eq!(search.id, "test");
        assert_eq!(search.title, "");
        assert_eq!(search.tags, Vec::<String>::new());
        assert_eq!(search.description, "");
        assert_eq!(search.references, Vec::<String>::new());
        assert_eq!(search.min_stars, 100);
        assert_eq!(search.max_stars, 0);
        assert_eq!(search.github_fork, true);
    }

    #[test]
    fn test_search_new_just_title() {
        let search = Search{
            id: "".to_owned(),
            title: "test".to_owned(),
            tags: Vec::new(),
            description: "".to_owned(),
            references: Vec::new(),
            min_stars: 100,
            max_stars: 0,
            github_fork: true,
        };

        assert_eq!(search.id, "");
        assert_eq!(search.title, "test");
        assert_eq!(search.tags, Vec::<String>::new());
        assert_eq!(search.description, "");
        assert_eq!(search.references, Vec::<String>::new());
        assert_eq!(search.min_stars, 100);
        assert_eq!(search.max_stars, 0);
        assert_eq!(search.github_fork, true);
    }

    #[test]
    fn test_search_new_just_tags() {
        let search = Search{
            id: "".to_owned(),
            title: "".to_owned(),
            tags: vec!["test".to_owned()],
            description: "".to_owned(),
            references: Vec::new(),
            min_stars: 100,
            max_stars: -1,
            github_fork: true,
        };

        assert_eq!(search.id, "");
        assert_eq!(search.title, "");
        assert_eq!(search.tags, vec![String::from("test")]);    
        assert_eq!(search.description, "");
        assert_eq!(search.references, Vec::<String>::new());
        assert_eq!(search.min_stars, 100);
        assert_eq!(search.max_stars, -1);
        assert_eq!(search.github_fork, true);
    }

    #[test]
    fn test_search_new_just_description() {
        let search = Search{
            id: "".to_owned(),
            title: "".to_owned(),
            tags: Vec::new(),
            description: "test".to_owned(),
            references: Vec::new(),
            min_stars: 100,
            max_stars: -1,
            github_fork: true,
        };

        assert_eq!(search.id, "");
        assert_eq!(search.title, "");
        assert_eq!(search.tags, Vec::<String>::new());
        assert_eq!(search.description, "test");
        assert_eq!(search.references, Vec::<String>::new());
        assert_eq!(search.min_stars, 100);
        assert_eq!(search.max_stars, -1);   
        assert_eq!(search.github_fork, true); 
    }

    #[test]
    fn test_search_new_just_references() {
        let search = Search{
            id: "".to_owned(),
            title: "".to_owned(),
            tags: Vec::new(),
            description: "".to_owned(),
            references: vec!["test".to_owned()],
            min_stars: 100,
            max_stars: -1,
            github_fork: true,
        };

        assert_eq!(search.id, "");
        assert_eq!(search.title, "");
        assert_eq!(search.tags, Vec::<String>::new());
        assert_eq!(search.description, "");
        assert_eq!(search.references, vec![String::from("test")]);
        assert_eq!(search.min_stars, 100);
        assert_eq!(search.max_stars, -1);
        assert_eq!(search.github_fork, true);
    }

    #[test]
    fn test_search_new_just_min_stars() {
        let search = Search{
            id: "".to_owned(),
            title: "".to_owned(),
            tags: Vec::new(),
            description: "".to_owned(),
            references: Vec::new(),
            min_stars: 1000,
            max_stars: -1,
            github_fork: true,
        };

        assert_eq!(search.id, "");
        assert_eq!(search.title, "");
        assert_eq!(search.tags, Vec::<String>::new());
        assert_eq!(search.description, "".to_owned());
        assert_eq!(search.references, Vec::<String>::new());
        assert_eq!(search.min_stars, 1000);
        assert_eq!(search.max_stars, -1);
        assert_eq!(search.github_fork, true);
    }

    #[test]
    fn test_search_new_just_max_stars() {
        let search = Search{
            id: "".to_owned(),
            title: "".to_owned(),
            tags: Vec::new(),
            description: "".to_owned(),
            references: Vec::new(),
            min_stars: 100,
            max_stars: 10,
            github_fork: true,
        };

        assert_eq!(search.id, "");
        assert_eq!(search.title, "");
        assert_eq!(search.tags, Vec::<String>::new());
        assert_eq!(search.description, "".to_owned());
        assert_eq!(search.references, Vec::<String>::new());
        assert_eq!(search.min_stars, 100);
        assert_eq!(search.max_stars, 10);
        assert_eq!(search.github_fork, true);
    }

    #[test]
    fn test_search_new_just_github_fork() {
        let search = Search{
            id: "".to_owned(),
            title: "".to_owned(),
            tags: Vec::new(),
            description: "".to_owned(),
            references: Vec::new(),
            min_stars: 100,
            max_stars: -1,
            github_fork: false,
        };

        assert_eq!(search.id, "");
        assert_eq!(search.title, "");
        assert_eq!(search.tags, Vec::<String>::new());
        assert_eq!(search.description, "".to_owned());
        assert_eq!(search.references, Vec::<String>::new());
        assert_eq!(search.min_stars, 100);
        assert_eq!(search.max_stars, -1);
        assert_eq!(search.github_fork, false);
    }

    #[test]
    fn test_search_new_just_github_last_push() {
        let search = Search{
            id: "".to_owned(),
            title: "".to_owned(),
            tags: Vec::new(),
            description: "".to_owned(),
            references: Vec::new(),
            min_stars: 100,
            max_stars: -1,
            github_fork: true,
        };

        assert_eq!(search.id, "");
        assert_eq!(search.title, "");
        assert_eq!(search.tags, Vec::<String>::new());
        assert_eq!(search.description, "".to_owned());
        assert_eq!(search.references, Vec::<String>::new());
        assert_eq!(search.min_stars, 100);
        assert_eq!(search.max_stars, -1);
        assert_eq!(search.github_fork, true);
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
        search.id_set(&"template1".to_owned());

        assert_eq!(search.id, "template1");
    }

    #[test]
    fn test_search_id_set_get() {
        let mut search = Search::new_empty();
        search.id_set(&"template1".to_owned());

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
        search.title_set(&"Template 1".to_owned());

        assert_eq!(search.title, "Template 1");
    }

    #[test]
    fn test_search_title_set_get() {
        let mut search = Search::new_empty();
        search.title_set(&"Template 1".to_owned());

        let out_title = search.title_get();

        assert_eq!(out_title, "Template 1");
    }

    #[test]
    fn test_search_tags_get() {
        let mut search = Search::new_empty();
        search.tags = vec!["tag1".to_owned(), "tag2".to_owned()];

        assert_eq!(search.tags_get(), &vec!["tag1", "tag2"]);
    }

    #[test]
    fn test_search_tags_set() {
        let mut search = Search::new_empty();
        search.tags_set(&vec!["tag1".to_owned(), "tag2".to_owned()]);

        assert_eq!(search.tags, vec!["tag1", "tag2"]);
    }

    #[test]
    fn test_search_tags_set_get() {
        let mut search = Search::new_empty();
        search.tags_set(&vec!["test".to_owned(), "test2".to_owned()]);

        let out_tags = search.tags_get();

        assert_eq!(out_tags, &vec!["test", "test2"]);
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
        search.description_set(&"This is a sample description.".to_owned());

        assert_eq!(search.description, "This is a sample description.");
    }

    #[test]
    fn test_search_description_set_get() {
        let mut search = Search::new_empty();
        search.description_set(&"$$$Bill$$$".to_owned());

        let out_description = search.description_get();

        assert_eq!(out_description, "$$$Bill$$$");
    }

    #[test]
    fn test_search_reference_get() {
        let mut search = Search::new_empty();
        search.references = vec!["https://example.com".to_owned()];

        assert_eq!(search.reference_get(), &vec!["https://example.com"]);
    }

    #[test]
    fn test_search_reference_set() {
        let mut search = Search::new_empty();
        search.reference_set(&vec!["https://example.com".to_owned()]);

        assert_eq!(search.references, vec!["https://example.com"]);
    }

    #[test]
    fn test_search_reference_set_get() {
        let mut search = Search::new_empty();
        search.reference_set(&vec!["https://loler_gmbh.com".to_owned(), "script<alert()>".to_owned()]);

        let out_tags = search.reference_get();

        assert_eq!(out_tags, &vec!["https://loler_gmbh.com", "script<alert()>"]);
    }

    #[test]
    fn test_search_min_stars_get() {
        let mut search = Search::new_empty();
        search.min_stars = 1000;

        assert_eq!(search.min_stars, 1000);
    }

    #[test]
    fn test_search_min_stars_set() {
        let mut search = Search::new_empty();
        search.min_stars_set(1000);

        assert_eq!(search.min_stars, 1000);
    }

    #[test]
    fn test_search_min_stars_set_get() {
        let mut search = Search::new_empty();
        search.min_stars_set(1000);
        let out_min_stars = search.min_stars_get();

        assert_eq!(out_min_stars, &1000);
    }

    #[test]
    fn test_search_max_stars_get() {
        let mut search = Search::new_empty();
        search.max_stars = 10;

        assert_eq!(search.max_stars_get(), &10);
    }

    #[test]
    fn test_search_max_stars_set() {
        let mut search = Search::new_empty();
        search.max_stars_set(10);

        assert_eq!(search.max_stars, 10);
    }

    #[test]
    fn test_search_max_stars_set_get() {
        let mut search = Search::new_empty();
        search.max_stars_set(10);
        let out_max_stars = search.max_stars_get();

        assert_eq!(out_max_stars, &10);
    }

    #[test]
    fn test_search_github_fork_get() {
        let mut search = Search::new_empty();
        search.github_fork = false;

        assert_eq!(search.github_fork_get(), &false);
    }
    
    #[test]
    fn test_search_github_fork_set() {
        let mut search = Search::new_empty();
        search.github_fork_set(false);

        assert_eq!(search.github_fork, false);
    }

    #[test]
    fn test_search_github_fork_set_get() {
        let mut search = Search::new_empty();
        search.github_fork_set(false);

        assert_eq!(search.github_fork_get(), &false);
    }

    #[test]
    fn test_search_similarities() {
        let value = vec!["This is a sample text.".to_owned()];
        let query = vec!["sample text".to_owned()];

        let similarity = Search::word_similarities(&value, &query);

        assert_eq!(similarity, 0.0);
    }

    #[test]
    fn test_search_similarity() {
        let value = "This is a sample text.";
        let query = "sample text";

        let similarity = Search::word_similarity(value, query);
        
        assert_eq!(similarity, 0.0);
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
            "Lol this is some type of work.".to_owned(),
            "To talk about this and that.".to_owned(),
            "Do not read this, it is not good for you.".to_owned(),
        ];
        let query = vec![
            "sample text".to_owned(),
            "This is a sample text.".to_owned(),
            "Another sentence.".to_owned(),
            "One more sentence.".to_owned(),
            "And some other text".to_owned(),
            "Why not like this.".to_owned()
        ];

        let similarity = Search::similarities_full(&value, &query);

        // Expected range of similarity.
        let expected_similarity_range = 0.0..=1.0;

        assert!(expected_similarity_range.contains(&similarity));
    }
}
