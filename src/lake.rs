use crate::{config::Config, search::Search, template::Template, yaml_template};
use colored::Colorize;
use reqwest::Client;
use sha2::{Digest, Sha256};
use std::{
    fs::{self, File},
    io::{self, Read},
    path::PathBuf,
};
use tokio::fs::create_dir_all;
use zip::ZipArchive;

#[derive(Debug)]
struct MyError {
    message: String,
}

impl std::fmt::Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for MyError {}

pub struct Lake {
    _config: Config,
    templates: Vec<Template>,
}

impl Lake {
    // Initializing the lake.
    pub fn new(
        in_url: String,
        in_update: bool,
        in_search: Search,
    ) -> Result<Lake, Box<dyn std::error::Error>> {
        let mut out_config: Config = Config::new()?;

        // If in_url is not empty, del the lake dir and set the config url to in_url.
        // Then do a reload of the config struct.
        if in_url != "" {
            out_config.del_lake_dir();
            out_config.set_new_url(in_url.to_owned());
            out_config.hash = "".to_string();
            if let Err(e) = Config::save_to_config_yaml(&out_config.url, &out_config.hash) {
                return Err(e);
            }

            //match Config::save_to_config_yaml(&out_config.url, &out_config.hash) {
            //    Ok(_) => Config::new(),
            //    Err(e) => return Err(e),
            //};
        }

        // If in_update is set to true, then delete the lake folder.
        // and a reload the config struct.
        if in_update {
            out_config.del_lake_dir();
            out_config.hash = "".to_string();
            if let Err(e) = Config::save_to_config_yaml(&out_config.url, &out_config.hash) {
                return Err(e);
            }
            out_config = Config::new()?;
            //Config::save_to_config_yaml(&out_config.url, &out_config.hash);
            //out_config = Config::new()?;
        }

        // If the ~/.config/wami/dir_to_lake is not set then load it from the url.
        if !Config::is_dir_present(out_config.get_lake_dir()) {
            let runtime = match tokio::runtime::Runtime::new() {
                Ok(rt) => rt,
                Err(e) => return Err(Box::new(e)),
            };

            let result = runtime.block_on(async { Lake::load_zip_from_url(&out_config).await });

            if let Err(e) = result {
                return Err(e);
            }
        }

        // Creating the lake struct.
        Ok(Lake {
            _config: out_config.clone(),
            templates: Lake::load_lake_from_config_dir(out_config, in_search),
        })
    }

    // Sort the template vector in descending order based on distance.
    pub fn print_top_hits(&mut self, how_many_max: usize, in_sort_value: String) {
        let _ = &self
            .templates
            .sort_by(|a, b| b.distance().partial_cmp(&a.distance()).unwrap());

        // Take as many we want form the top of the sorted templates.
        let max_hits_templates = &self.templates[..how_many_max.min(self.templates.len())];

        // Out put in the desired order.
        match in_sort_value.as_str() {
            "desc" => {
                for (index, template) in max_hits_templates.iter().rev().enumerate() {
                    let reverse_index = max_hits_templates.len() - index;
                    println!("Number: {}", reverse_index);
                    println!("Program: {}", template.to_string());
                }
            },
            _ => {
                for (index, template) in max_hits_templates.iter().enumerate() {
                    println!("Number: {}", index + 1);
                    println!("Program: {}", template.to_string());
                }
            }
        }
    }

    // Sort the template vector in descending order based on distance.
    pub fn print_top_short_list(&mut self, how_many_max: usize, in_sort_value: String) {
        let _ = &self
            .templates
            .sort_by(|a, b| b.distance().partial_cmp(&a.distance()).unwrap());

        // Take as many we want form the top of the sorted templates.
        let max_hits_templates = &self.templates[..how_many_max.min(self.templates.len())];

        match in_sort_value.as_str() {
            "desc" => {
                for (index, template) in max_hits_templates.iter().rev().enumerate() {
                    let reverse_index = max_hits_templates.len() - index;
                    println!("{}: {}", reverse_index, template.to_short_string());
                }
            }
            _ => {
                for (index, template) in max_hits_templates.iter().enumerate() {
                    println!("{}: {}", index + 1, template.to_short_string());
                }
            }
        }
    }

    // Read the yaml file in the given path.
    fn read_yaml_file(file_path: &str) -> Result<String, io::Error> {
        // Open the file
        let mut file = File::open(file_path)?;

        // Creating string for the content.
        let mut contents = String::new();

        // Try to read the file content.
        match file.read_to_string(&mut contents) {
            Ok(_) => Ok(contents),
            Err(err) => {
                eprintln!(
                    "Error reading the yaml file at path: {} \nError: {}",
                    file_path, err
                );
                Err(err)
            }
        }
    }

    // Load the lake using the config struct.
    fn load_lake_from_config_dir(in_config: Config, in_search: Search) -> Vec<Template> {
        let mut out_templates: Vec<Template> = Vec::<Template>::new();

        // Trying to load the dir.
        match fs::read_dir(in_config.get_lake_dir()) {
            Ok(entries) => {
                for entry in entries {
                    if let Ok(entry) = entry {
                        // Got the path of a file.
                        let path = entry.path();

                        // If the file has an extension.
                        if let Some(extension) = path.extension() {
                            // If the file has a yaml extension.
                            if extension == "yaml" {
                                // Try to read the yaml file
                                match Lake::read_yaml_file(path.to_str().unwrap()) {
                                    Ok(yaml_string) => {
                                        match serde_yaml::from_str::<yaml_template::YamlTemplate>(
                                            &yaml_string,
                                        ) {
                                            Ok(in_yaml_template) => {
                                                // Use the new operator because there is an string formatting function integrated.
                                                // If you would use the deserializing method, it would be easier but maybe not correct.
                                                out_templates.push(Template::new(
                                                    in_yaml_template.id,
                                                    in_search.id_get(),
                                                    in_yaml_template.title,
                                                    in_search.title_get(),
                                                    in_yaml_template.tags,
                                                    in_search.tags_get(),
                                                    in_yaml_template.description,
                                                    in_search.description_get(),
                                                    in_yaml_template.references,
                                                    in_search.reference_get(),
                                                ));
                                            }
                                            Err(err) => {
                                                eprintln!("Failed to deserialize YAML: {}", err);
                                                eprintln!("{:#?}", path.to_str().unwrap());
                                                continue; // Skip this file and continue.
                                            }
                                        }
                                    }
                                    Err(err) => {
                                        eprintln!("Failed to read file: {}", err);
                                        eprintln!("{:#?}", path.to_str().unwrap());
                                        continue; // Skip this file and continue.
                                    }
                                }
                            }
                        }
                    }
                }

                out_templates
            }

            // Error there is no way to read the dir.
            Err(err) => {
                eprintln!("Failed to read directory: {}", err);
                out_templates
            }
        }
    }

    // Loading the zip file from the url, using the config struct.
    pub async fn load_zip_from_url(in_config: &Config) -> Result<(), Box<dyn std::error::Error>> {
        let client = Client::new();

        // Check if we can connect to the url.
        Lake::check_connection_to_url(in_config.url.to_owned()).await?;

        // Send a request to get the zip.
        let response = client.get(in_config.url.to_owned()).send().await?;

        // If this request fails, the return an error.
        if !response.status().is_success() {
            return Err("Failed to fetch the zip file at Lake::load_zip_form_url".into());
        }

        // If the request is ok read the bytes in the archive
        let bytes = response.bytes().await?;

        // Generate the hash for the config.yaml file.
        let hash_hex = Lake::generate_hash(&bytes);

        // Save the url and hash in the config.yaml
        if let Err(e) = Config::save_to_config_yaml(&in_config.url, &hash_hex) {
            return Err(e);
        }

        //Config::save_to_config_yaml(&in_config.url, &hash_hex);

        let reader = std::io::Cursor::new(bytes);
        let mut archive = ZipArchive::new(reader)?;

        // loop throw the archive
        for i in 0..archive.len() {
            // Get the file with the index.
            let mut file = archive.by_index(i)?;

            // configure the path to save the files.
            let mut out_path: PathBuf = Config::get_config_path().to_owned();
            out_path.push(file.mangled_name());

            // If the file is an directory
            if file.name().ends_with('/') {
                // Create a directory if it doesn't exist.
                tokio::fs::create_dir_all(&out_path).await?;
            } else {
                // Ensure the parent directory exists
                if let Some(p) = out_path.parent() {
                    if !p.exists() {
                        create_dir_all(&p).await?;
                    }
                }

                // Create the file
                let mut out_file = File::create(&out_path)?;

                // Write the file
                std::io::copy(&mut file, &mut out_file)?;
            }
        }
        Ok(())
    }

    // // Create the hash of the zip file from the given url
    // // and print information to the user if the hash is different to the saved hash.
    // pub async fn get_zip_hash_of_url_lake(in_config: &Config)
    //  -> Result<String, Box<dyn std::error::Error>> {
    //     let client = Client::new();

    //     if !Lake::check_connection_to_url(in_config.url.to_owned()).await {
    //         println!("{}",
    //             format!("{}",
    //                  "Can not check for updates the connection to the url is failing.".bold().red()
    //             )
    //         );
    //         return Ok("".to_string());
    //     }

    //     // Send a request to get the zip.
    //     let response = client.get(in_config.url.to_owned()).send().await?;

    //     // If this request fails, the return an error.
    //     if !response.status().is_success() {
    //         return Err("Failed to fetch the zip file at Lake::load_zip_form_url".into());
    //     }

    //     // If the request is ok read the bytes in the archive
    //     let bytes = response.bytes().await?;

    //     // Generate the hash for the config.yaml file.
    //     let hash_hex = Lake::generate_hash(&bytes);

    //     if in_config.hash != hash_hex && in_config.hash != ""{
    //         println!("{}",
    //             format!("{}",
    //                  "There is a new update.".bold().red()
    //             )
    //         );
    //         println!("Use: {}",
    //             format!("{}",
    //                 "wami -u".bold().green()
    //             )
    //         )
    //     }

    //     Ok(hash_hex)
    // }

    // Create the hash of the zip file from the given url
    // and print information to the user if the hash is different to the saved hash.
    pub async fn get_zip_hash_of_url_lake(
        in_config: &Config,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let client = Client::new();

        // Check the connection to the url
        Lake::check_connection_to_url(in_config.url.to_owned()).await?;

        // Send a request to get the zip.
        let response = client.get(in_config.url.to_owned()).send().await?;

        // If this request fails, the return an error.
        if !response.status().is_success() {
            return Err("Failed to fetch the zip file at Lake::load_zip_form_url".into());
        }

        // If the request is ok read the bytes in the archive
        let bytes = response.bytes().await?;

        // Generate the hash for the config.yaml file.
        let hash_hex = Lake::generate_hash(&bytes);

        if in_config.hash != hash_hex && in_config.hash != "" {
            println!("{}", format!("{}", "There is a new update.".bold().red()));
            println!("Use: {}", format!("{}", "wami -u".bold().green()))
        }

        Ok(hash_hex)
    }

    pub fn generate_hash(in_data: &[u8]) -> String {
        // Generate the hash for the config.yaml file.
        let hash = Sha256::digest(in_data);
        let hash_hex = format!("{:x}", hash);
        hash_hex
    }

    // This will check if it is possible to connect to the url.
    // pub async fn check_connection_to_url(in_url: String) -> bool {
    //     match reqwest::get(in_url).await {
    //         Ok(response) => response.status().is_success(),
    //         Err(_) => false,
    //     }
    // }

    pub async fn check_connection_to_url(in_url: String) -> Result<(), Box<dyn std::error::Error>> {
        match reqwest::get(&in_url).await {
            Ok(response) => {
                if response.status().is_success() {
                    Ok(())
                } else {
                    Err(format!("Error connecting to the URL: {}", in_url).into())
                }
            }
            Err(e) => Err(format!("Error connecting to the URL: {} {}", in_url, e).into()),
        }
    }
}
