// next step is to implement the load url and read folder function.

use crate::{template::Template, yaml_template, search::Search};
use std::{fs::{self, File}, process, io::{self, Read}};
use reqwest::Client;
use tokio::{fs::create_dir_all};
use url::Url;
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
    _url: String,
    templates: Vec<Template>,
}

impl Lake {
    
    // Setting the default path of the lake.
    pub fn default( in_search: Search
        
    ) -> Lake {
        Lake::new(
            "https://github.com/evait-security/wami-templates/archive/refs/heads/main.zip",
            in_search
        )
    }
    
    // Initializing the lake.
    pub fn new(
        in_url: &str,
        in_search: Search
    ) -> Lake {
        let temp_dir: String = Lake::dir_extract(in_url);
        if !Lake::dir_exists(temp_dir.to_owned()) {
            // Loading the lake from the URL.
            tokio::runtime::Runtime::new().unwrap().block_on(async {
                Lake::load_from_url(&in_url).await.unwrap();
            });
        }
        Lake { 
            _url: in_url.to_owned(),
            templates: Lake::load_the_lake_from_dir(
                temp_dir,
                in_search
            )
        }
    }

    pub fn print_top_hits(&mut self, how_many_max: usize){
        // Sort the vector in descending order based on distance.
        let _ = &self.templates.sort_by(|a, b| b.distance().partial_cmp(&a.distance()).unwrap());

        // Take the as many we want form the top of the sorted templates.
        let max_hits_templates = &self.templates[..how_many_max.min(self.templates.len())];

        for (index, template) in max_hits_templates.iter().enumerate() {
            println!("Number: {}", index + 1);
            println!("Program: {}", template.to_string());
        }
    }

    fn read_yaml_file(file_path: &str) -> Result<String, io::Error> {
        let mut file = File::open(file_path)?;
        let mut contents = String::new();
        match file.read_to_string(&mut contents) {
            Ok(_) => {
                Ok(contents)
            }
            Err(err) => {
                eprintln!("Error reading the yaml file at path: {} \nError: {}", file_path, err);
                Err(err)
            }
        }
    }
    
    fn load_the_lake_from_dir(
        in_dir_path: String,
        in_search: Search
    ) -> Vec<Template>{
        let mut out_templates: Vec<Template> = Vec::<Template>::new();
                
        // Trying to load the dir.
        match fs::read_dir(in_dir_path){
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
                                match Lake::read_yaml_file(path.to_str().unwrap()){
                                    Ok(yaml_string) =>{
                                        let in_yaml_template: yaml_template::YamlTemplate = 
                                            serde_yaml::from_str(&yaml_string)
                                            .expect("Failed to deserialize YAML");
                                        
                                        // println!("Name: {:#?}", in_template );
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
                                            in_search.reference_get()
                                        ));
                                    }
                                    Err(err) => {
                                        eprintln!("Failed to read file: {}", err);
                                        eprintln!("{:#?}", path.to_str().unwrap());
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

    pub async fn load_from_url(url: &str) -> Result<(), Box<dyn std::error::Error>> {

        let client = Client::new();
    
        // Send a request to get the zip.
        let response = client.get(url).send().await?;
        
        // If this request fails, the return an error.
        if !response.status().is_success() {
            return Err("Failed to fetch the zip file".into());
        }

        // If the request is ok read the bytes in the archive
        let bytes = response.bytes().await?;    
        let reader = std::io::Cursor::new(bytes);
        let mut archive = ZipArchive::new(reader)?;
    
        for i in 0..archive.len() {
            
            // loop throw the archive
            let mut file = archive.by_index(i)?;
            let out_path = file.mangled_name();
    
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

    fn dir_exists(in_dir: String) -> bool {

        // Check if the path exists.
        match fs::metadata(in_dir.to_owned()){
            Ok(metadata) => {
                if metadata.is_dir() {
                    
                    // The dir is there, we will read it.
                    return true
                } else {
                                        
                    // Error there is a problem with the dir path,
                    // maybe there is a file that hast the same name as the dir path.
                    // I will not delete it, because I do not now for what that file is.
                    // Maybe there are no user rights and I am not able to read at the path.
                    // I will print out the the dir path, so there is an possibility to find the error.
                    eprintln!("Error Path is not a dir: {}", in_dir);
                    eprintln!("Try to delete the file with the same name or add the needed rights to write.");
                    
                    let error_code = 1;
                    process::exit(error_code);
                }
            }
            Err(_) => {       
              
                // The dir is not there, we will load its from the URL and then we will read it.
                return false
            }
        }
    }

    fn dir_extract(in_url: &str) -> String {
        
        // Extract the path segments
        let parsed_url = Url::parse(&in_url).expect("Failed to parse URL");
        
        // Create segments out of the url
        let url_segments: Vec<_> = parsed_url.path_segments().unwrap().collect();

        // Get the repository name form the segment
        let repository = url_segments[1];

        // Get the branch name form the segment
        let branch = url_segments[5].strip_suffix(".zip").unwrap_or(url_segments[5]);
        
        // Create the path name
        repository.to_owned() + "-" + branch + "/lake/"
    }
}
