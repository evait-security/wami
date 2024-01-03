use crate::search::Search;

use isahc;
use isahc::ReadResponseExt;
use serde::Deserialize;
use std::error::Error;

// This is the structure that will save the information of a repositories.
#[derive(Debug, Deserialize)]
struct GitHubRepositories {
    name: String,
    html_url: String,
    description: String,
    updated_at: String,
    stargazers_count: u64,
    topics: Vec<String>,
    score: f64
}

// This is the main structure,
// it will save the individual repositories that are found.
#[derive(Debug, Deserialize)]
pub struct GithubSearch {
    items: Vec<GitHubRepositories>
}

impl GithubSearch {
    pub fn new(in_search: Search) -> Result<Self, Box<dyn Error>> {
        let base_url = "https://api.github.com/search/repositories";
        // Setting up the required url value for the search at api github.
        let in_tags = in_search.tags_get().join(",");
        let query_stage_one = format!("q={:?}&forks={}&min_stars={}&max_stars={}",
            in_tags,
            in_search.github_fork_get(),
            in_search.min_stars_get(),
            in_search.max_stars_get()
        );

        // The join string starts and ends with an semicolon they have to be removed.
        let query_stage_final = query_stage_one.replace("\"", "");

        // Add the base url to the query
        let url = format!("{}?{}", base_url, query_stage_final);

        // Get the response
        let mut response = isahc::get(url.to_owned())?;

        if response.status().is_success() {
            // First make an string
            let tmp_body: String = response.text()?;

            // Then get the response body
            let response_body: Result<GithubSearch, serde_json::Error> = serde_json::from_str(&tmp_body);

            match response_body {
                Ok(search_result) => {
                    // If everything is ok return search_result
                    Ok(search_result)
                },
                Err(err) => {
                    // By error return error
                    println!("Error: serde_json {:?}", err);
                    Err(err.into())
                },
            }
        } else {
            // If it is not successful then print an error.
            println!("Error: at GitHub API-Request");
            println!("URL: {}", url);
            println!("Base_URL: {}", base_url);
            println!("in_search: {}", in_search.title_get());
            println!("response.status(): {}", response.status());
            Err("Error: at GitHub API-Request".into())
        }
    }
    pub fn to_string(&self, in_max_list: usize, in_sort_value: &str) -> String {
        let mut out_string: String = String::new();
        let mut count: usize;

        if in_sort_value == "asc" {
            count = 1;
            let items = Box::new(self.items.iter()) as Box<dyn Iterator<Item = &GitHubRepositories>>;
            for item in items {
                if count < in_max_list + 1
                { 
                    out_string.push_str(&count.to_string());
                    out_string.push_str(". ");
                    out_string.push_str(&item.name);
                    out_string.push_str(" - ");
                    // The description is cut to an limit of 255 charters because there are description
                    // that are used to communicant with other users and there are way to long.
                    out_string.push_str(&cut_of_string(&item.description, 255).to_string());
                    out_string.push_str("\n  - url: ");
                    out_string.push_str(&item.html_url);
                    out_string.push_str("\n  - Stars: ");
                    out_string.push_str(&item.stargazers_count.to_string());
                    out_string.push_str("\n  - Topics");
                    // Topic is an array so we will loop throw it.
                    for topic in &item.topics {
                        out_string.push_str("\n    - ");
                        out_string.push_str(&topic);
                    }
                    out_string.push_str("\n  - Last update at: ");
                    out_string.push_str(&item.updated_at);
                    out_string.push_str("\n");
                    out_string.push_str("  - Score of finding: ");
                    out_string.push_str(&item.score.to_string());
                    out_string.push_str("\n");
                }
                count += 1;
            }
        } else { // desc
            count = self.items.iter().len();
            let items = Box::new(self.items.iter().rev()) as Box<dyn Iterator<Item = &GitHubRepositories>>;
            for item in items {
                if count <= in_max_list
                { 
                    out_string.push_str(&count.to_string());
                    out_string.push_str(". ");
                    out_string.push_str(&item.name);
                    out_string.push_str(" - ");
                    // The description is cut to an limit of 255 charters because there are description
                    // that are used to communicant with other users and there are way to long.
                    out_string.push_str(&cut_of_string(&item.description, 255).to_string());
                    out_string.push_str("\n  - url: ");
                    out_string.push_str(&item.html_url);
                    out_string.push_str("\n  - Stars: ");
                    out_string.push_str(&item.stargazers_count.to_string());
                    out_string.push_str("\n  - Topics");
                    // Topic is an array so we will loop throw it.
                    for topic in &item.topics {
                        out_string.push_str("\n    - ");
                        out_string.push_str(&topic);
                    }
                    out_string.push_str("\n  - Last update at: ");
                    out_string.push_str(&item.updated_at);
                    out_string.push_str("\n");
                    out_string.push_str("  - Score of finding: ");
                    out_string.push_str(&item.score.to_string());
                    out_string.push_str("\n");
                }
                count -= 1;
            }
        }
        out_string
    }
}

// This is used to cut an string to the max_length value.
fn cut_of_string(input: &str, max_length: usize) -> String {
    if input.chars().count() <= max_length {
        return input.to_string()
    } else {
        let cut_of_string: String = input.chars().take(max_length).collect();
        return cut_of_string
    }
}