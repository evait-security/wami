use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct YamlTemplate {
    pub id: String, // A unique name in lower case letters.
    pub title: String, // The real name of the tool / software.
    pub tags: Vec<String>, // An vector of lower case tags for the search function.
    pub description: String, // A longer text describing the too and what it does. 
    pub references: Vec<String>, // Links to websites, repositories, or other resources.
}