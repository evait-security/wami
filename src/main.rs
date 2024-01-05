mod config;
mod lake;
mod search;
mod template;
mod yaml_template;
mod github_topic;
mod github_search;

use clap::{App, Arg};
use colored::Colorize;
use std::io::{self};


fn main() {
    // Define the command-line arguments
    let help_flag_github= "--github".green();
    let help_flag_list_topics = "--list-topics".green();
    let help_flag_show_all = "-a".green();
    let help_flag_search_all = "-s".green();
    let help_flag_search_order = "-S".green();
    let help_value_search_order_desc = "desc".truecolor(200,200,200);
    let example_text = format!("Example:\n  {} {} {} {} {} {}\n    {}\n\n  {} {} {} {} {} {}\n    {}\n\n  {} {} {}\n    {}",
                                    "wami".magenta(),
                                    help_flag_show_all,
                                    help_flag_search_order,
                                    help_value_search_order_desc,
                                    help_flag_search_all,
                                    "pentest".truecolor(90,90,255),
                                    "This example will search in the lake with extended output in descending order in all categories for the word pentest.",
                                    "wami".magenta(),
                                    help_flag_show_all,
                                    help_flag_search_order,
                                    help_value_search_order_desc,
                                    help_flag_github,
                                    "pentest".truecolor(90,90,255),
                                    "This example will search in GitHub with extended output in descending order for the word pentest.",
                                    "wami".magenta(),
                                    help_flag_list_topics,
                                    "pentest".truecolor(90,90,255),
                                    "This example will search in GitHub for topics with the word pentest."
                                );
    let about_text = format!("{} is a user-friendly tool designed in Rust, powered by Cargo, to assist individuals who struggle with remembering the names of the various programs they utilize. This open-source program aims to simplify the process of finding the most suitable programs for specific tasks.\n\nCreated at 10.07.2023", "WAMI".bold().green());
    let app = App::new(format!("{} - What am I", "WAMI".bold().green()))
        .version("\tVersion: 0.1.0\n")
        .author("evait security GmbH\nNxtTAB <wami@evait.de>\n\n")
        .about(&*about_text)
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
                .short("T")
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
            Arg::with_name("list-topics")
                .long("list-topics")
                .value_name("LIST-TOPICS")
                .help("This will use the GitHub API to list topics. This will exclude searching in the lake.")
                .required(false)
                .multiple(true)
        )
        .arg(
            Arg::with_name("sort")
                .short("S")
                .long("sort")
                .value_name("SORT")
                .help("This will determine the sorting direction asc or desc, desc is the default direction.")
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
            Arg::with_name("offline")
                .short("o")
                .long("offline")
                .value_name("OFFLINE")
                .help("Set this flag if you do not want the online check for the updated lake file.")
                .required(false)
                .multiple(false)
                .takes_value(false)
        )
        .arg(
            Arg::with_name("min-stars")
                .long("min-stars")
                .value_name("MIN-STARS")
                .help("Set the minimum of stars that have to be present in the Github repos. The default value is 100")
                .required(false)
                .multiple(false)
        )
        .arg(
            Arg::with_name("max-stars")
                .long("max-stars")
                .value_name("MAX-STARS")
                .help("Set the maximum of stars that have to be present in the GitHub repos. The default value is set to 0 which means that there is no limit.")
                .required(false)
                .multiple(false)
        )
        .arg(
            Arg::with_name("github-fork")
                .long("github-fork")
                .help("This will set the search option for GitHub to the forks of an project. The default value is set to false.")
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
        .arg(
            Arg::with_name("why_not")
                .short("W")
                .long("why_not")
                .value_name("WHY_NOT")
                .takes_value(false)
                .help("Set the why not flag to see other programs, witch are better for this use case.")
                .required(false)
                .multiple(false)
        )
        .arg(
            Arg::with_name("github")
                .long("github")
                .takes_value(false)
                .help("This will set the search to the GitHub API.")
                .required(false)
                .multiple(false)
        ).after_help(&*example_text);

        let matches = app.clone().get_matches();
       
    // If there are no arguments show the help.
    if std::env::args().len() <= 1 {
        app.write_help(&mut io::stdout()).unwrap();
        println!(); // New line at the end of the help output
        return;
    }
    
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
            );

        search
            .title_set(
                &search_all_string
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
        );
        
        search
            .description_set(
                &search_all_string
        );
        
        search.reference_set(
            &search_all_vec
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
                        .to_owned()
                + " "
                + &search_names
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
                        .to_owned()
                + " "
                + &search_names
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

    // Is min-stars set for github search?
    // Set it anyway because it will be ingnored, if the github search is not active.
    if let Some(min_stars_str) = matches.value_of("min-stars") {
        if let Ok(min_stars) = min_stars_str.parse::<isize>() {
            if min_stars > 100 {
                search.min_stars_set(min_stars);
            } else {
                search.min_stars_set(100);
            }
        } else {
            search.min_stars_set(100);
            println!("Failed to pares the min stars value. Please enter a vialed unsigned positive number.");
            println!("Using the default of main 100 stars to search on github.");
        }
    }

    // Is max-stars set for github search?
    // If not, set it to zero or if it is zero,  
    // set it to unlimited.
    if let Some(max_stars_str) = matches.value_of("max-stars") {
        if let Ok(max_stars) = max_stars_str.parse::<isize>() {
            if  max_stars <= search.min_stars_get().to_owned() {
                search.max_stars_set(0);
            } else {
                search.max_stars_set(max_stars);
            }
        } else {
            // Prasing faild
            search.max_stars_set(0);
            println!("Failed to parse the max stars value please enter a valid number.");
            println!("Using the default value 0, there will be no max value");

        }
    }

    // Is the github fork search option set to true?
    // If it is not set, the standard initialization of the search struct is github-fork = false.
    // This will search the forks of an project on github.
    if matches.is_present("github-fork") {
        search.github_fork_set(true);
    }
    
    if let Some(search_topics) = matches.values_of("list-topics") {
        let out_search_topics_vec: Vec<String> =
            search_topics
                .map(|search_topic| search_topic
                    .to_string())
                .collect();
        let _ = github_topic::get_github_topics(out_search_topics_vec);
        std::process::exit(0); 
    }

    // let mut update flag
    let mut update = false;

    // If the update flag is set, set update.
    if let Some(_search_name) = matches.values_of("update") {
        update = true;
    }

    // offline flag is set default to false so there will be an check if the lake is up to date
    let mut offline: bool = false;

    // if the offline flag is set, set the flag to true
    if let Some(_search_name) = matches.values_of("offline") {
        offline = true;
    }

    let mut url: String = "".to_string();

    // Is the url set?
    if let Some(in_url) = matches
        .values_of("url")
        .and_then(|mut values| values.next())
    {
        url = in_url.to_string();
    }

    // In sort the default value for the sort value is asc
    let mut sort_value = "desc".to_string();

    if let Some(sort) = matches
        .value_of("sort")
    {
        sort_value = sort.to_string();
    }
    
    // The default value of max item to list is 10.
    let mut max_list: usize = 10;

    // Is the max value set?
    if let Some(search_names) = matches.value_of("max") {
        if let Ok(max) = search_names.parse::<usize>() {
            max_list = max;
        } else {
            // Praseing failed
            println!("Failed to parse the max value please enter a vailid not negative number.");
            println!("Using the default of 10 Items to list.");
        }
    }

    // Is the why_not value set?
    let mut why_not_option: bool = false;
    if let Some(_search_name) = matches.values_of("why_not") {
        why_not_option = true;
    }

    // Set the default value for the GitHub search.
    let mut github: bool = false;

    // Check if the GitHub search value is set to true.
    if let Some(_search_name) = matches.values_of("github") {
        github = true;
    }   

    // If github is not set
    // Create the lake an instance of the lake
    // We have the url if it has changed
    // We have the update boolean
    // And we have all the search parameters
    if !github {
        let lake_result = 
            lake::Lake::new(
                url, 
                update, 
                search
            );
        match lake_result {
            Ok(mut lake) => {
                // Now you have a valid Lake instance in the lake variable.
                if matches.is_present("show-all") {
                    lake.print_top_hits(max_list, sort_value, why_not_option);
                } else {
                    lake.print_top_short_list(max_list, sort_value, why_not_option);
                }

                if !update && !offline {
                    match config::Config::get_git_hash(&lake.get_config_url()) {
                        Ok(hash) => {
                            if hash != lake.get_config_hash() {
                                let message = format!("{}", "Please update the lake, it is outdated.".bold().red());
                                println!("{}", message);
                            }
                        },
                        Err(_err) => println!("Version of lake can not be downloaded.")
                    }
                }
            }
            Err(e) => {
                println!("Failed to create the Lake: {}", e);
                std::process::exit(1);
            }
        }
    } else {
        let github_result = 
            github_search::GithubSearch::new(
                search
            );
        match github_result {
            Ok(search_result) => {
                if matches.is_present("show-all"){
                    let result_string = search_result.to_string(max_list, &sort_value, true);
                    println!("{}", result_string);
                } else {
                    let result_string = search_result.to_string(max_list, &sort_value, false);
                    println!("{}", result_string);
                }
            }
            Err(err) => {
                println!("The response form GitHub is empty");
                println!("Error: {:?}", err);
            }
        }
    }
}