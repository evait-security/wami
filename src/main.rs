mod config;
mod lake;
mod search;
mod template;
mod yaml_template;

use clap::{App, Arg};
use colored::Colorize;

fn main() {
    // Define the command-line arguments using clap
    let matches = App::new(format!("{} - What am I", "WAMI".bold().green()))
        .version("\tVersion: 0.1.0\n")
        .author("evait security GmbH\nNxtTAB <wami@evait.de>\n\n")
        .about(&*format!("{} is a user-friendly tool designed in Rust, powered by Cargo, to assist individuals who struggle with remembering the names of the various programs they utilize. This open-source program aims to simplify the process of finding the most suitable programs for specific tasks.\n\nCreated at 10.07.2023", "WAMI".bold().green()))
        .arg(
            Arg::with_name("strings")
                .value_name("STRING")
                .required(false)
                .multiple(true),
        )
        .arg(
            Arg::with_name("search-all")
                .short("s")
                .long("search-all")
                .value_name("SEARCH_ALL")
                .help("The search all functionality will search throw all fields with the same search criteria.")
                .required(false)
                .multiple(true)
        )
        .arg(
            Arg::with_name("search-unique-name")
                .short("n")
                .long("search-unique-name")
                .value_name("SEARCH_UNIQUE_NAME")
                .help("This will search throw all the unique names with the values of SEARCH_UNIQUE_NAME.")
                .required(false)
                .multiple(true)
        )
        .arg(
            Arg::with_name("search-title")
                .short("t")
                .long("search-title")
                .value_name("SEARCH_TITLE")
                .help("This will search throw all the title with the values of SEARCH_TITLE.")
                .required(false) .multiple(true) )
        .arg(
            Arg::with_name("search-tags")
                .short("g")
                .long("search-tags")
                .value_name("SEARCH_TAGS")
                .help("This will search throw all the tags with the values.")
                .required(false)
                .multiple(true)
        )
        .arg(
            Arg::with_name("search-description")
                .short("d")
                .long("search-description")
                .value_name("SEARCH_DESCRIPTION")
                .help("This will search throw all the description with the values of SEARCH_DESCRIPTION.")
                .required(false)
                .multiple(true)
        )
        .arg(
            Arg::with_name("search-references")
                .short("r")
                .long("search-references")
                .value_name("SEARCH_REFERENCES")
                .help("This will search throw all the references with the values of SEARCH_REFERENCES.")
                .required(false)
                .multiple(true)
        )
        .arg(
            Arg::with_name("sort")
                .short("S")
                .long("sort")
                .value_name("SORT")
                .help("This will determine the sorting direction asc or desc")
                .required(false)
                .multiple(false)
        )
        .arg(
            Arg::with_name("url")
                .short("U")
                .long("url")
                .value_name("URL")
                .help("Set an alternative url for the lake download.")
                .required(false)
                .multiple(false)
        )
        .arg(
            Arg::with_name("update")
                .short("u")
                .long("update")
                .value_name("UPDATE")
                .help("Make an update of the lake.")
                .required(false)
                .multiple(false)
                .takes_value(false)
        )
        .arg(
            Arg::with_name("max")
                .short("M")
                .long("max")
                .value_name("MAX")
                .help("Set the maximum of listed programs default (MAX = 10).")
                .required(false)
                .multiple(false)
        )
        .arg(
            Arg::with_name("show-all")
                .short("a")
                .long("show-all")
                .takes_value(false)
                .help("Display detailed list of all available information.")
                .required(false)
                .multiple(false)
        )
        .get_matches();

    // using the search struct to define the search parameters.
    let mut search: search::Search 
        = search::Search::new_empty();

    // Is default search set by entering just strings,
    // then we will search for tags
    if let Some(search_names) 
        = matches.values_of("strings") 
    {
        search
            .tags_set(
                &search_names
                    .map(|tag| 
                        tag
                            .to_string()
                    )
                    .collect()
        );
    }

    // Is search all set?
    if let Some(search_names) 
        = matches.values_of("search-all") 
    {
        let search_all_string: String = 
            search_names
                .clone()
                .collect::<Vec<_>>()
                .join(" ");
        
        let search_all_vec: Vec<String>=
            search_names
                .map(|name|
                    name
                        .to_string()
                )
                .collect();   

        search
            .id_set(
                &search_all_string
                    // .clone()
            );

        search
            .title_set(
                &search_all_string
                    // .clone()
            );
        
        // It is possible that the search.tags is not empty,
        // because of default search is tags.
        let mut search_tags_vec: Vec<String> =
            search_all_vec
                .clone();

        search_tags_vec
            .push(
                search
                    .tags_get()
                        .iter()
                        .map(|tag| tag
                            .to_string() 
                        )
                        .collect()
            );

        search
            .tags_set(
                &search_tags_vec
                    //.clone()
        );
        
        search
            .description_set(
                &search_all_string
                    //.clone()
        );
        
        search.reference_set(
            &search_all_vec
                //.clone()
        );
    }

    // Is search unique names set?
    if let Some(search_names) 
        = matches.values_of("search-unique-name") 
    {
        search.id_set(
            &(
                search
                    .id_get()
                        .clone()
                        .to_owned()
                + " "
                + &search_names
                    //.clone()
                    .collect::<Vec<_>>()
                    .join(" ")
            )
        );
    }

    // Is search title set?
    if let Some(search_names) = matches.values_of("search-title") {
        search.title_set(
            &(
                search
                    .title_get()
                        // .clone()
                        .to_owned()
                + " "
                + &search_names
                    //.clone()
                    .collect::<Vec<_>>()
                    .join(" ")
            )
        );
    }

    // Is search tags set?
    if let Some(search_names) = matches.values_of("search-tags") {
        let mut in_search_tags_vec: Vec<String> = 
            search_names
                .map(|search_name| search_name
                    .to_string()
                )
                .collect();        
        
        in_search_tags_vec
            .push(
                search
                    .tags_get()
                    .iter()
                    .map(|tag| tag
                        .to_string())
                    .collect());
        search.tags_set(&in_search_tags_vec);
    }

    // Is search description set?
    if let Some(search_names) = matches.values_of("search-description") {
        search.description_set(
            &(
                search
                    .description_get()
                        // .clone()
                        .to_owned()
                + " "
                + &search_names
                    // .clone()
                    .collect::<Vec<_>>()
                    .join(" ")
            )
        );
    }

    // Is search reference set?
    if let Some(search_names) = matches.values_of("search-references") {
        let mut in_search_references_vec: Vec<String> = 
            search_names
                .map(|search_name| search_name
                    .to_string())
                .collect();        
        in_search_references_vec
            .push(
                search
                    .reference_get()
                    .iter()
                    .map(|refe| refe
                        .to_string())
                    .collect());
        search.reference_set(&in_search_references_vec);
    }

    // let mut lake_result: lake::Lake;
    let mut update = false;

    // If the update flag is set, set update.
    if let Some(_search_name) = matches.values_of("update") {
        update = true;
    }

    let mut url: String = "".to_string();

    // Is the url set?
    if let Some(in_url) = matches
        .values_of("url")
        .and_then(|mut values| values.next())
    {
        url = in_url.to_string();
    }

    // The default value of max item to list is 10.
    let mut max_list = 10;

    // In sort the default value for the sort value is asc
    let mut sort_value = "asc".to_string();

    if let Some(sort) = matches
        .value_of("sort")
    {
        sort_value = sort.to_string();
    }
    // Is the max value set?
    if let Some(search_names) = matches.value_of("max") {
        let result: Result<usize, _> = search_names.parse();
        match result {
            Ok(value) => {
                max_list = value;
            }
            Err(_) => {
                // Parsing failed
                println!("Failed to parse the max value please enter a valid number.");
            }
        }
    }

    // Create the lake an instance of the lake
    // We have the url if it has changed
    // We have the update boolean
    // And we have all the search parameters
    let lake_result = lake::Lake::new(url, update, search);

    match lake_result {
        Ok(mut lake) => {
            // Now you have a valid Lake instance in the lake variable.
            if matches.is_present("show-all") {
                lake.print_top_hits(max_list, sort_value);
            } else {
                lake.print_top_short_list(max_list, sort_value);
            }
        }
        Err(e) => {
            println!("Failed to create the Lake: {}", e);
            std::process::exit(1);
        }
    }

    // Check for updates.
    tokio::runtime::Runtime::new().unwrap().block_on(async {
        match config::Config::new() {
            Ok(config) => {
                lake::Lake::get_zip_hash_of_url_lake(&config)
                    .await
                    .expect("Failed to load zip at lake::Lake::new");
            }
            Err(e) => {
                println!("Can not load check for updates: {}", e);
            }
        }
    })
}
