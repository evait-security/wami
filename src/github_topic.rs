use colored::Colorize;
use isahc;
use isahc::ReadResponseExt;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Body {
    items: Vec<GitHubTopic>
}
#[derive(Debug, Deserialize)]
struct GitHubTopic {
    name: String,
}

pub fn get_github_topics(in_list: Vec<String>) -> Result<(), isahc::Error> {
    let query: String = String::from("q=").to_owned() + &in_list.join(",");
    let url = format!("https://api.github.com/search/topics?{}", query);

    let mut response = isahc::get(&url.to_string())?;

    if response.status().is_success() {
        let body = response.text()?;

        let response_body: Result<Body, serde_json::Error> = serde_json::from_str(&body);

        match response_body {
            Ok(response_body) => {
                let mut count = 1;
                for item in response_body.items {
                    let tmp_out_string=format!("{} {}", count.to_string().magenta(), item.name.green() );
                    println!("{}", tmp_out_string);
                    count += 1;
                }
            }
            Err(err) => {
                println!("Error deserializing JSON: {}", err);
            }
        }
        
    } else {
        println!("Request was not successful. Status code: {:?}", response.status());
    }

    Ok(())
}