use crate::template::Template;
use std::collections::HashMap;
// use levenshtein::levenshtein;

// This will be the framework for the search algorithmic.

// In the struct will be a word, a query and the distance,
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

    // Function to calculate the average cosine similarity between pairs of strings from two vectors.
    // This function iterates over every combination of strings between 'words' and 'queries',
    // computes the cosine similarity for each pair, and then returns the average of these similarities.
    pub fn cosine_similarities(words: &Vec<String>, queries: &Vec<String>) -> f32 {
        // Check if either of the vectors is empty and return 1.0 if both are empty.
        if words.is_empty() && queries.is_empty() {
            return 1.0;
        }

        // Check if either of the vectors is empty and return 0.0 in that case.
        if words.is_empty() || queries.is_empty() {
            return 0.0;
        }
        
        // Initialize total similarity and count.
        let mut dist: f32 = 0.0;
        let mut count: i32 = 0;
    
        // Iterate over each word in the words vector.
        for v in words {
            // For each word, iterate over each query in the queries vector.
            for q in queries {
                // Add the cosine similarity between the current word and query to the total.
                dist += Search::cosine_similarity(&v.to_string(), &q.to_string());
                
                // Increment the count for each word-query pair.
                count += 1;
            }
        }
    
        // If there were comparisons made (count > 0), return the average similarity.
        // Otherwise, return 0.0 to avoid division by zero.
        if count > 0 {
            dist / count as f32
        } else {
            0.0 as f32
        }
    }

    // Function to create bigrams from a given string.
    // A bigram is a pair of consecutive characters.
    // This function returns a HashMap where each key is a bigram and its value is its relative frequency in the string.
    fn create_bigrams(word: &str) -> HashMap<String, f64> {
        let mut ngrams = HashMap::new();
        let chars: Vec<char> = word.chars().collect();
        let total = (chars.len() - 1) as f64;

        // Iterate through the string to create bigrams.
        for i in 0..chars.len() - 1 {
            // Increment the frequency of each bigram, normalized by the total number of bigrams.
            let ngram = chars[i..=i + 1].iter().collect::<String>();
            *ngrams.entry(ngram).or_insert(0.0) += 1.0 / total;
        }
    
        ngrams
    }
    
    // Function to calculate the cosine similarity between two strings using bigrams.
    // This version uses the frequencies of bigrams to capture more contextual information compared to individual characters.
    pub fn cosine_similarity(word: &str, query: &str) -> f32 {
        // Check if either of the strings is empty and return 1.0 if both are empty.
        if word.is_empty() && query.is_empty() {
            return 1.0;
        }

        // Check if either of the strings is empty and return 0.0 in that case.
        if word.is_empty() || query.is_empty() {
            return 0.0;
        }
        
        // Create bigrams for each string.
        let freq1 = Search::create_bigrams(word);
        let freq2 = Search::create_bigrams(query);
    
        // Calculate the dot product of the two bigram frequency vectors.
        let dot_product: f64 = freq1.iter()
            .filter_map(|(k, v)| freq2.get(k).map(|v2| *v * *v2))
            .sum();
    
        // Calculate the magnitude (Euclidean norm) of each vector.
        let magnitude1 = freq1.values().map(|v| v.powi(2)).sum::<f64>().sqrt();
        let magnitude2 = freq2.values().map(|v| v.powi(2)).sum::<f64>().sqrt();
    
        // Calculate and return the cosine similarity.
        // If either vector is zero, return 0.0 to avoid division by zero.
        if magnitude1 * magnitude2 == 0.0 {
            0.0 as f32
        } else {
            (dot_product / (magnitude1 * magnitude2)) as f32
        }
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

    // This test function is designed to verify both setting and getting functionalities of a Search object.
    #[test]
    fn test_search_new_set_get() {
        // Initialize a new Search object with specific values.
        // Fields `id`, `title`, `tags`, `description`, `references`, `min_stars`, `max_stars`, and `github_fork` are set.
        let mut search = Search{
            id: "id1".to_owned(),
            title: "title1".to_owned(),
            tags:  vec![String::from("tags1")] ,
            description: "description1".to_owned(),
            references: vec![String::from("references1")],
            min_stars: 100,
            max_stars: 1000,
            github_fork: true,
        };

        // Assert that the initial values of each field are set as expected.
        assert_eq!(search.id, "id1");
        assert_eq!(search.title, "title1");
        assert_eq!(search.tags, vec![String::from("tags1")]);
        assert_eq!(search.description, "description1");
        assert_eq!(search.references, vec![String::from("references1")]);
        assert_eq!(search.min_stars, 100);
        assert_eq!(search.max_stars, 1000);
        assert_eq!(search.github_fork, true);
        
        // Set new values to each field of the Search object using respective setter methods.
        search.id_set(&"id2".to_string());
        search.title_set(&"title2".to_string());
        search.tags_set(&vec![String::from("tags2")]);
        search.description_set(&"description2".to_string());
        search.reference_set(&vec![String::from("reference2")]);
        search.min_stars_set(200);
        search.max_stars_set(2000);
        search.github_fork_set(false);

        // Assert that the values of each field are updated to the new values.
        assert_eq!(search.id_get(), "id2");
        assert_eq!(search.title_get(), "title2");
        assert_eq!(search.tags_get(), &vec![String::from("tags2")]);
        assert_eq!(search.description_get(), "description2");
        assert_eq!(search.reference_get(), &vec![String::from("reference2")]);
        // Note: `*` is used to dereference the values returned by `get` methods for `min_stars`, `max_stars`, and `github_fork`,
        // since these methods return references.
        assert_eq!(*search.min_stars_get(), 200 );
        assert_eq!(*search.max_stars_get(), 2000);
        assert_eq!(*search.github_fork_get(), false);
    }

    #[test]
    fn test_search_cosine_similarities_full_empty_vectors() {
        // Define two empty vectors 'word' and 'query' for testing.
        let word: Vec<String> = Vec::new();
        let query: Vec<String> = Vec::new();

        // Calculate the average cosine similarity between all pairs of strings from the two vectors.
        let similarity = Search::cosine_similarities(&word, &query);

        // This test checks the behavior of the cosine_similarities function when both input vectors are empty.
        // The expected behavior in this case is that the similarity should be 1.0.
        // This is because, conceptually, two empty vectors are identical in that they both lack any elements.
        // Since cosine similarity measures the degree of overlap in features (here represented by strings) between two vectors,
        // two empty vectors have complete overlap (i.e., neither vector has any elements that the other lacks),
        // which leads to a similarity score of 1.0, indicating perfect similarity.

        // The assertion checks if the calculated average similarity is indeed 1.0 as expected.
        assert_eq!(similarity, 1.0);
    }

    #[test]
    fn test_search_cosine_similarities_word_empty_vectors() {
        // Define an empty vector 'word' and a vector 'query' with a single string for testing.
        let word: Vec<String> = Vec::new();
        let query: Vec<String> = vec!["This is a sample text.".to_owned()];

        // Calculate the average cosine similarity between all pairs of strings from the two vectors.
        let similarity = Search::cosine_similarities(&word, &query);

        // This test checks the behavior of the cosine_similarities function when one of the input vectors is empty.
        // In this scenario, 'word' is an empty vector, while 'query' contains a single string.
        // The expected behavior in this case is that the similarity should be 0.0.
        // This is because with an empty 'word' vector, there are no strings to compare against the strings in 'query'.
        // Consequently, there are no pairs of strings to evaluate for similarity, and the function should correctly return 0.0.
        // This indicates that there is no similarity when one of the vectors is empty, as there are no data points for comparison.

        // The assertion checks if the calculated average similarity is indeed 0.0 as expected.
        assert_eq!(similarity, 0.0);
}

    #[test]
    fn test_search_cosine_similarities_query_empty_vectors() {
        // Define a vector 'word' with a single string and an empty vector 'query' for testing.
        let word: Vec<String> = vec!["This is a sample text.".to_owned()];
        let query: Vec<String> = Vec::new();

        // Calculate the average cosine similarity between all pairs of strings from the two vectors.
        let similarity = Search::cosine_similarities(&word, &query);

        // This test checks the behavior of the cosine_similarities function when one of the input vectors is empty.
        // In this scenario, 'word' is a vector with a single string, while 'query' is an empty vector.
        // The expected behavior in this case is that the similarity should be 0.0.
        // This is because with an empty 'query' vector, there are no strings to compare against the strings in 'value'.
        // As a result, there are no pairs of strings to evaluate for similarity, and the function should correctly return 0.0.
        // This indicates that there is no similarity when one of the vectors is empty, as there are no data points for comparison.

        // The assertion checks if the calculated average similarity is indeed 0.0 as expected.
        assert_eq!(similarity, 0.0);
    }

    #[test]
    fn test_search_cosine_similarity_full_empty_strings() {
        // Define two empty strings 'word' and 'query' for testing.
        let word = "";
        let query = "";

        // Calculate the cosine similarity between 'word' and 'query'.
        let similarity = Search::cosine_similarity(word, query);

        // This test checks the behavior of the cosine_similarity function when both input strings are empty.
        // The expected behavior in this scenario is that the similarity should be 1.0.
        // This is because, conceptually, two empty strings are identical in that they both lack any characters or features.
        // Since cosine similarity measures the overlap in features (bigrams) between two strings,
        // two empty strings have complete overlap (i.e., neither has any features that the other lacks),
        // which leads to a similarity score of 1.0, indicating perfect similarity.

        // The assertion checks if the calculated similarity is indeed 1.0 as expected.
        assert_eq!(similarity, 1.0);
}

    #[test]
    fn test_search_cosine_similarity_empty_word_with_query_strings() {
        // Define an empty string 'word' and a non-empty string 'query' for testing.
        let word = "";
        let query = "test";

        // Calculate the cosine similarity between 'word' and 'query'.
        let similarity = Search::cosine_similarity(word, query);

        // This test checks the behavior of the cosine_similarity function when one of the input strings is empty.
        // In this scenario, the 'word' string is empty, while 'query' is a regular non-empty string.
        // The expected behavior in this case is that the similarity should be 0.0.
        // This is because an empty string, having no characters, also has no bigrams, 
        // and therefore shares no common features with any non-empty string.
        // Since cosine similarity calculates the degree of overlap in features (bigrams) between two strings,
        // the lack of any common features means the similarity score should be 0.0, indicating no similarity.

        // The assertion checks if the calculated similarity is indeed 0.0 as expected.
        assert_eq!(similarity, 0.0);
    }


    #[test]
    fn test_search_cosine_similarity_word_with_empty_query() {
        // Define a non-empty string 'word' and an empty string 'query' for testing.
        let word = "test";
        let query = "";
    
        // Calculate the cosine similarity between 'word' and 'query'.
        let similarity = Search::cosine_similarity(word, query);
    
        // This test checks the behavior of the cosine_similarity function when one of the input strings is empty.
        // The expected behavior in this case is that the similarity should be 0.0.
        // This is because an empty string has no characters (and hence no bigrams),
        // so it shares no common features with any other string, including a non-empty string.
        // As a result, the cosine similarity, which measures the degree of overlap in features (bigrams) between two strings,
        // should be 0.0, indicating no similarity.
    
        // The assertion checks if the calculated similarity is indeed 0.0 as expected.
        assert_eq!(similarity, 0.0);
    }
    

    #[test]
    fn test_search_cosine_similarities_full_long_vectors_equal() {
        // Define two vectors of strings for testing.
        let value = vec![
            "aaaa".to_owned(),
            "aaab".to_owned(),
            "aabc".to_owned(),
            "abcd".to_owned(),
        ];
        let query = vec![
            "aaaa".to_owned(),
            "aaab".to_owned(),
            "aabc".to_owned(),
            "abcd".to_owned(),
        ];
    
        // Calculate the average cosine similarity between all pairs of strings from the two vectors.
        let similarity = Search::cosine_similarities(&value, &query);
    
        // Define the expected similarity value based on manual calculation or prior knowledge.
        let expected_similarity = 0.646405;
        // Define a tolerance range to account for potential minor deviations in floating-point calculations.
        let tolerance = 0.001;
    
        // The assertion checks whether the calculated similarity is within the expected range.
        // This approach is used because floating-point calculations can have small rounding errors,
        // so exact equality might not be a reliable condition.
        // The tolerance range allows for slight numerical variations while still ensuring the accuracy of the calculation.
        assert!(
            (similarity - expected_similarity).abs() < tolerance,
            "The similarity value is not within the expected range"
        );
    }

    #[test]
    fn test_search_cosine_similarities_full_long_reversed_vectors() {
        // Define two vectors of strings for testing, with the second vector being a reverse of the first.
        let value = vec![
            "aaaa".to_owned(),
            "aaab".to_owned(),
            "aabc".to_owned(),
            "abcd".to_owned(),
        ];
        let query = vec![
            "abcd".to_owned(), // Reversed order compared to 'value'
            "aabc".to_owned(),
            "aaab".to_owned(),
            "aaaa".to_owned(),
        ];
    
        // Calculate the average cosine similarity between all pairs of strings from the two vectors.
        let similarity = Search::cosine_similarities(&value, &query);
    
        // Define the expected similarity value and a tolerance range for the test.
        let expected_similarity = 0.646405;
        let tolerance = 0.001;
    
        // The assertion checks whether the calculated similarity is within the tolerance range of the expected value.
        // This approach is necessary due to potential minor variations in floating-point calculations.
        assert!(
            (similarity - expected_similarity).abs() < tolerance,
            "The similarity value is not within the expected range"
        );
    }

    #[test]
    fn test_search_cosine_similarities_1_long_vectors_equal() {
        // Define two vectors 'word' and 'query', each containing a single identical string "aa".
        let word = vec![
            "aa".to_owned(),
        ];
        let query = vec![
            "aa".to_owned(),
        ];
    
        // Calculate the average cosine similarity between all pairs of strings from the two vectors.
        // In this case, since each vector has only one string and both strings are identical,
        // the cosine similarity is calculated for this single pair of identical strings.
        let similarity = Search::cosine_similarities(&word, &query);
    
        // This test checks the behavior of the cosine_similarities function when the input vectors contain identical strings.
        // Since the only pair of strings to compare is identical ("aa" from both vectors),
        // the expected result is a cosine similarity of 1.0, indicating perfect similarity.
    
        // The assertion verifies if the calculated average similarity for this pair is indeed 1.0 as expected,
        // confirming that the function correctly calculates the similarity for identical strings,
        // recognizing them as perfectly similar.
        assert_eq!(similarity, 1.0);
    }
    
    
    #[test]
    fn test_search_cosine_similarities_1_long_vectors_not_equal() {
        // Define two vectors 'word' and 'query', each containing a single string.
        // 'word' contains the string "aa", and 'query' contains the string "cb".
        let word = vec![
            "aa".to_owned(),
        ];
        let query = vec![
            "cb".to_owned(),
        ];
    
        // Calculate the average cosine similarity between all pairs of strings from the two vectors.
        // In this case, there is only one pair of strings to compare, and these strings are "aa" and "cb".
        let similarity = Search::cosine_similarities(&word, &query);
    
        // This test checks the behavior of the cosine_similarities function with two completely different strings.
        // Since "aa" and "cb" do not share any common characters, they also do not have any common bigrams.
        // Therefore, the expected result is a cosine similarity of 0.0, indicating no similarity.
    
        // The assertion verifies if the calculated average similarity for this pair is indeed 0.0 as expected,
        // confirming that the function correctly identifies the lack of similarity between completely different strings.
        assert_eq!(similarity, 0.0);
    }    

    #[test]
    fn test_search_cosine_similarities_2_char_strings_vectors_have_not_equal_values() {
        let value = vec![
            "aa".to_owned(),
        ];
        let query = vec![
            "ab".to_owned(),
        ];

        let similarity = Search::cosine_similarities(&value, &query);

        // In this test case, we calculate the cosine similarity between the strings "aa" and "ab".
        // Since the cosine similarity algorithm uses bigrams, we consider the bigrams in each string.
        // "aa" contains the bigram "aa".
        // "ab" contains the bigram "ab".
        // As there are no common bigrams between the two strings, the similarity is 0.0.
        // This result is confirmed by the formula for calculating cosine similarity,
        // which is based on the number of shared features (here bigrams) between the two strings.
    
        // The test checks if the calculated similarity is 0.0.
        assert_eq!(similarity, 0.0);
        // assert!(expected_similarity_range.contains(&similarity));
    }

    #[test]
    fn test_search_cosine_similarities_2_char_strings_vectors_have_not_equal_value_revers() {
        let value = vec![
            "aa".to_owned(),
        ];
        let query = vec![
            "ba".to_owned(),
        ];

        let similarity = Search::cosine_similarities(&value, &query);

        // In this test case, we calculate the cosine similarity between the strings "aa" and "ab".
        // Since the cosine similarity algorithm uses bigrams, we consider the bigrams in each string.
        // "aa" contains the bigram "aa".
        // "ab" contains the bigram "ab".
        // As there are no common bigrams between the two strings, the similarity is 0.0.
        // This result is confirmed by the formula for calculating cosine similarity,
        // which is based on the number of shared features (here bigrams) between the two strings.
    
        // The test checks if the calculated similarity is 0.0.
        assert_eq!(similarity, 0.0);
        // assert!(expected_similarity_range.contains(&similarity));
    }

    #[test]
    fn test_search_cosine_similarities_1_long_vectors_2_by_3_1_equal() {
        // Define two vectors 'word' and 'query', each containing a single identical string "baa".
        let word = vec![
            "baa".to_owned(),
        ];
        let query = vec![
            "baa".to_owned(),
        ];
    
        // Calculate the average cosine similarity between all pairs of strings from the two vectors.
        // In this case, since each vector has only one string and both strings are identical,
        // the cosine similarity is calculated for this single pair of identical strings.
        let similarity = Search::cosine_similarities(&word, &query);
    
        // This test checks the behavior of the cosine_similarities function when the input vectors contain identical strings.
        // Since the only pair of strings to compare is identical ("baa" from both vectors),
        // the expected result is a cosine similarity of 1.0, indicating perfect similarity.
    
        // The assertion verifies if the calculated average similarity for this pair is indeed 1.0 as expected.
        // This confirms that the function correctly calculates the similarity for identical strings,
        // recognizing them as perfectly similar.
        assert_eq!(similarity, 1.0);
    }    

    #[test]
    fn test_search_cosine_similarity_string_1_by_1_equal() {
        // Define two strings 'word' and 'query' for testing, where both strings are identical.
        // Both 'word' and 'query' consist of the character 'a' repeated three times.
        let word: &str = "aaa";
        let query: &str = "aaa";
    
        // Calculate the cosine similarity between 'word' and 'query'.
        let similarity = Search::cosine_similarity(word, query);
    
        // This test checks the behavior of the cosine_similarity function when the two input strings are identical.
        // In this case, since 'word' and 'query' are exactly the same, they share all their characters and, consequently, all their bigrams.
        // Therefore, the expected outcome is that the cosine similarity should be 1.0,
        // indicating perfect similarity as per the cosine similarity metric.
    
        // The assertion verifies if the calculated similarity is indeed 1.0 as expected,
        // confirming that the function accurately identifies the complete similarity between two identical strings.
        assert_eq!(similarity, 1.0);
    }
    
    #[test]
    fn test_search_cosine_similarity_string_1_by_1_not_equal() {
        // Define two strings 'word' and 'query' for testing.
        // 'word' consists of the character 'a' repeated three times.
        // 'query' consists of the character 'b' repeated three times.
        let word: &str = "aaa";
        let query: &str = "bbb";
    
        // Calculate the cosine similarity between 'word' and 'query'.
        let similarity = Search::cosine_similarity(word, query);
    
        // This test checks the behavior of the cosine_similarity function with two strings that are different.
        // In this case, 'word' and 'query' do not share any common characters, which means they have no common bigrams.
        // The expected outcome is that the cosine similarity between these two strings should be 0.0,
        // indicating that they are completely dissimilar according to the cosine similarity metric.
    
        // The assertion verifies if the calculated similarity is indeed 0.0 as expected, 
        // confirming that the two strings are recognized as completely dissimilar.
        assert_eq!(similarity, 0.0);
    }
    
    #[test]
    fn test_search_cosine_similarity_string_1_4_by_1_2_not_equal() {
        // Define two strings 'word' and 'query' for testing.
        // 'word' consists of the character 'a' repeated four times.
        // 'query' consists of the character 'b' repeated two times.
        let word: &str = "aaaa";
        let query: &str = "bb";
    
        // Calculate the cosine similarity between 'word' and 'query'.
        let similarity = Search::cosine_similarity(word, query);
    
        // This test checks the behavior of the cosine_similarity function with two entirely different strings.
        // In this case, 'word' and 'query' do not share any common characters. Therefore, they do not have any common bigrams.
        // The cosine similarity should reflect this lack of commonality by returning a similarity score of 0.0.
        // This would mean that the two strings are completely dissimilar as per the cosine similarity metric.
    
        // The assertion checks if the calculated similarity is indeed 0.0 as expected.
        assert_eq!(similarity, 0.0);
    }
    

    #[test]
    fn test_search_cosine_similarity_string_1_4_by_1_2_05_equal() {
        // Define two strings 'word' and 'query' for testing.
        // 'word' is a string with repeated character 'a' four times.
        // 'query' is a string with repeated character 'a' two times.
        let word: &str = "aaaa";
        let query: &str = "aa";
    
        // Calculate the cosine similarity between 'word' and 'query'.
        let similarity = Search::cosine_similarity(word, query);
    
        // This test checks the behavior of the cosine_similarity function with two strings of different lengths,
        // but with repetitive characters. The test case is designed to see how the function handles strings where
        // one is a subset of the other.
        // The expected behavior in this case is that the similarity should be 1.0 because the shorter string 'aa'
        // is entirely contained within the longer string 'aaaa'. In terms of bigrams, both strings will have
        // the bigram 'aa' repeated, leading to a perfect similarity score.
    
        // The assertion checks if the calculated similarity is indeed 1.0 as expected.
        assert_eq!(similarity, 1.0);
    }
    
    
}
