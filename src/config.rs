use isahc;
use isahc::ReadResponseExt;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::env;
use std::fs;
use std::io::{BufReader, Write};
use std::path::PathBuf;
use url::Url;

// This is the configuration file of the rust program.

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub url: String,
    pub hash: String,
}

impl Config {
    pub fn new() -> Result<Config, Box<dyn std::error::Error>> {
        let config_path: PathBuf = Config::get_config_path();
        Ok(Config::load_config_yaml(config_path)?)
    }

    // Load the config.yaml file.
    pub fn load_config_yaml(config_path: PathBuf) -> Result<Config, Box<dyn std::error::Error>> {
        let file = fs::File::open(Config::init_config_yaml(config_path)?)?;

        let reader = BufReader::new(file);

        let config: Config = serde_yaml::from_reader(reader)?;

        Ok(config)
    }

    pub fn init_config_yaml(config_path: PathBuf) -> Result<PathBuf, Box<dyn std::error::Error>> {
        // If the path to $HOME/.config/wami is not present then create it.
        if !Config::is_dir_present(config_path.to_owned()) {
            Config::create_config_path(config_path.to_owned())?;
        }

        // If the config.yaml file is not present then create it.
        let config_file_path: PathBuf = Config::get_config_file_path(config_path.clone());
        if !Config::is_config_yaml_present(config_file_path.clone()) {
            // Creating an Config struct.
            let config_yaml;
            let in_url: &String = &"https://github.com/evait-security/wami-templates/archive/refs/heads/main.zip".to_string();
            match Config::get_git_hash(in_url) {
                Ok(hash) => {
                    config_yaml = Config {
                        url: in_url.to_owned(),
                        hash: hash.to_string(),
                    };    
                },
                Err(_err) => {
                    println!("Version of the lake can not be downloaded. Setting the hash to empty string.");
                    config_yaml = Config {
                        url: in_url.to_owned(),
                        hash: "".to_string(),
                    };
                }
            }

            let yaml_content = serde_yaml::to_string(&config_yaml)?;

            let mut file = fs::File::create(config_file_path.clone())?;

            file.write_all(yaml_content.as_bytes())?;
        }

        Ok(config_file_path)
    }

    // This is a setter function, for the url value of the struct.
    pub fn set_new_url(&mut self, in_url: String) {
        if Url::parse(&in_url)
            .map(|url| !url.path().contains(".."))
            .unwrap_or(false)
        {
            self.url = in_url;
        } else {
            panic! {"Invalid URL: {}", in_url};
        }
    }

    // This will return the path of the config.yaml file.
    fn get_config_file_path(in_path_buffer: PathBuf) -> PathBuf {
        let mut out_file_buffer = in_path_buffer;
        out_file_buffer.push("config.yaml");
        out_file_buffer
    }

    // Check if config.yaml is present.
    fn is_config_yaml_present(in_file_buffer: PathBuf) -> bool {
        in_file_buffer.exists()
    }

    // Check if config path is present.
    pub fn is_dir_present(in_path_buffer: PathBuf) -> bool {
        in_path_buffer.exists()
    }

    // Get the path to the wami config section on the file system.
    pub fn get_config_path() -> PathBuf {
        if let Ok(home_dir) = env::var("HOME") {
            PathBuf::from(home_dir + "/.config/wami")
        } else if let Ok(user_profile) = env::var("USERPROFILE") {
            PathBuf::from(user_profile + "/.config/wami")
        } else {
            // Error: There is no possibility to get the home or userprofile dir,
            // so I will use the root dir of the program to save the config and lake information.
            PathBuf::from(".")
        }
    }

    // This will create the dir for the config folder of Wami.
    fn create_config_path(in_path_buffer: PathBuf) -> Result<(), std::io::Error> {
        fs::create_dir(in_path_buffer)
    }

    // This will generate the path of the lake dir.
    pub fn get_lake_dir(&self) -> PathBuf {
        // Extract the path segments
        let parsed_url =
            Url::parse(&self.url).expect("Failed to parse URL at at config::Config::get_lake_dir.");

        // Create segments out of the url
        let url_segments: Vec<_> = parsed_url.path_segments().unwrap().collect();

        // Get the repository name form the segment
        let repository = url_segments[1];

        // Get the branch name form the segment
        let branch = url_segments[5]
            .strip_suffix(".zip")
            .unwrap_or(url_segments[5]);

        // Create the path name
        let mut lake_path: PathBuf = Config::get_config_path();
        lake_path.push(repository.to_owned() + "-" + branch + "/lake/");
        lake_path
    }

    // This function is used by the update function.
    // If there is no lake dir, the lake will be downloaded from the url.
    pub fn del_lake_dir(&self) {
        match fs::remove_dir_all(&self.get_lake_dir().to_owned()) {
            Ok(()) => { /* Do nothing, there is no reason to be noisy. */ }
            Err(err) => eprintln!("Failed to delete directory: {}", err),
        }
    }

    // Save the config.yaml
    pub fn save_to_config_yaml(
        in_url: &String,
        in_hash: &String,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // get the config path
        let config_path: PathBuf = Config::get_config_path();

        // Creating an Config struct
        let config_yaml = Config {
            url: in_url.to_owned(),
            hash: in_hash.to_owned(),
        };

        // Serialize the config struct
        let yaml_content = serde_yaml::to_string(&config_yaml)?;

        // Create the config file
        let config_file_path = Config::init_config_yaml(config_path)?;
        let mut file = fs::File::create(config_file_path)?;

        // Write the content to the file.
        file.write_all(yaml_content.as_bytes())?;

        Ok(())
    }

    pub fn get_git_hash(in_url: &String) -> Result<String, Box<dyn std::error::Error>> {
        if let Ok(url) = Url::parse(&in_url) {
            if let Some(branch_name) = url.path_segments().unwrap().last() {
                if branch_name.ends_with(".zip") {
                    let repo_url = url.clone();
                    let repo_parts: Vec<&str> = repo_url.path_segments().unwrap().take(3).collect();
    
                    let mut api_url = Url::parse("https://api.github.com").unwrap();
                    api_url
                        .path_segments_mut()
                        .unwrap()
                        .extend(&["repos", repo_parts[0], repo_parts[1], "git", "refs", "heads", branch_name.trim_end_matches(".zip")]);
    
                    let mut response = isahc::get(&api_url.to_string())?;
                    let response_text = response.text()?;
                    let parsed_response: Value = serde_json::from_str(&response_text)?;
                    if let Some(sha_value) = parsed_response["object"]["sha"].as_str() {
                        Ok(sha_value.to_string()) 
                    } else {
                        Err("SHA value not found in JSON response".into())
                    }
                } else {
                    Err("Invalid URL structure".into())
                }
            } else {
                Err("Invalid URL structure".into())
            }
        } else {
            Err("Invalid URL".into())
        }
    }
}
