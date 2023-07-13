use serde::{Serialize, Deserialize};
use std::env;
use std::fs;
use std::io::{Write, BufReader};
use std::path::PathBuf;
use url::Url;


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub url: String,
}

impl Config {
    pub fn new() -> Config {
        Config::get_config()
    }
    
    pub fn _to_string(&self) -> String {
        self.url.to_owned() + &"\n".to_string()
    }

    // This function will load the config values.
    // If there is no value saved at the config path then it will create a standard config.
    fn get_config() -> Config {
        let config_path: PathBuf = Config::get_config_path();
        
        // If the path is not present then create it.
        if !Config::is_dir_present(config_path.to_owned()) {
            Config::create_config_path(config_path.to_owned())
                .expect("Failed to create the config dir at config::Config::get_config is_path_present no");
        }

        // If the config.yaml file is not present then create it.
        let config_file_path: PathBuf = Config::get_config_file_path(config_path);
        if !Config::is_config_yaml_present(config_file_path.to_owned()) {
            
            // Creating an Config struct.
            let config_yaml = Config {
                url: "https://github.com/evait-security/wami-templates/archive/refs/heads/main.zip".to_string(),
            };

            let yaml_content = serde_yaml::to_string(&config_yaml)
                .expect("Failed to serialize YAML content at config::Config::get_config is_config_yaml_present no.");

            let mut file = fs::File::create(config_file_path.to_owned())
                .expect("Failed to create config.yaml file at config::Config::get_config is_config_yaml_present no");

            file.write_all(yaml_content.as_bytes())
                .expect("Failed to write YAML content to config.yaml file at config::Config::get_config is_config_yaml_present no");
        }

        // Load the config.yaml file.
        let file = fs::File::open(config_file_path)
            .expect("Failed to open the config.yaml file at config::Config::get_config");
        let reader = BufReader::new(file);
        let config: Config = serde_yaml::from_reader(reader)
            .expect("Failed to deserialize the file at config::Config::get_config.");
        
        // Return the configuration
        config
    }

    pub fn set_new_url(&mut self, in_url: String){
        self.url = in_url;
    }

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
        let parsed_url = Url::parse(&self.url).expect("Failed to parse URL at at config::Config::get_lake_dir.");
        
        // Create segments out of the url
        let url_segments: Vec<_> = parsed_url.path_segments().unwrap().collect();

        // Get the repository name form the segment
        let repository = url_segments[1];

        // Get the branch name form the segment
        let branch = url_segments[5].strip_suffix(".zip").unwrap_or(url_segments[5]);
        
        // Create the path name
        let mut lake_path: PathBuf = Config::get_config_path();
        lake_path.push(repository.to_owned() + "-" + branch + "/lake/");
        lake_path
    }

    // This function is used by the update function.
    // If there is no lake dir, the lake will be downloaded for the url.
    pub fn del_lake_dir(&self) {
        match fs::remove_dir_all(&self.get_lake_dir().to_owned()) {
            Ok(()) => { /* Do nothing, there is no reason to be noisy. */ },
            Err(err) => eprintln!("Failed to delete directory: {}", err),
        }
    }
}
