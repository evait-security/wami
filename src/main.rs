mod lake;
mod template;
mod search;
mod yaml_template;

use clap::{App, Arg};

fn main() {

    // Define the command-line arguments using clap
    let matches = App::new("WAMI - What am I")
        .version("0.1.0")
        .author("evait security GmbH")
        .about("WAMI is a user-friendly tool designed in Rust language, powered by Cargo, to assist individuals who struggle with remembering the names of the various programs they utilize. This open-source program aims to simplify the process of finding the most suitable programs for specific tasks.")
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
                .short("u")
                .long("search-unique-name")
                .value_name("SEARCH_UNIQUE_NAME")
                .help("This will search throw all the unique names with the values of SEARCH_UNIQUE_NAME.")
                .required(false)
                .multiple(true)
        )
        .arg(
            Arg::with_name("search_tags")
                .short("t")
                .long("search-tags")
                .value_name("SEARCH_TAGS")
                .help("This will search throw all the tags with the values.")
                .required(false)
                .multiple(true)
        )
        .arg(
            Arg::with_name("search_description")
                .short("d")
                .long("search-description")
                .value_name("SEARCH_DESCRIPTION")
                .help("This will search throw all the description with the values of SEARCH_DESCRIPTION.")
                .required(false)
                .multiple(true)
        )
        .arg(
            Arg::with_name("search_references")
                .short("r")
                .long("search-references")
                .value_name("SEARCH_REFERENCES")
                .help("This will search throw all the references with the values of SEARCH_REFERENCES.")
                .required(false)
                .multiple(true)
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
            Arg::with_name("max")
                .short("M")
                .long("max")
                .value_name("MAX")
                .help("Set the maximum of listed programs default (MAX = 10).")
                .required(false)
                .multiple(false)
        )
        .author("NxtTAB <wami@evait.de>")
        .about("Created at 10.07.2023")
        .get_matches();
    
    let mut search: search::Search = search::Search::new_empty();

    if let Some(search_names) = matches.values_of("search-all") {
        let in_search_all_string: String = search_names.collect::<Vec<_>>().join(" ");
        let mut in_search_all_vec: Vec<String> = Vec::<String>::new();
        in_search_all_vec.push(in_search_all_string.to_owned());
        // println!("Search names specified: {:?}", in_search_all_string.to_owned());
        search.id_set(in_search_all_string.to_owned());
        search.title_set(in_search_all_string.to_owned());
        search.tags_set(in_search_all_vec.to_owned());
        search.description_set(in_search_all_string.to_owned());
        search.reference_set(in_search_all_vec.to_owned());
    }

    if let Some(search_names) = matches.values_of("search-unique-name") {
        let in_search_unique_name: String = search_names.collect::<Vec<_>>().join(" ");
        search.id_set(search.id_get() + &in_search_unique_name);
    }

    if let Some(search_names) = matches.values_of("search-title") {
        let in_search_title: String = search_names.collect::<Vec<_>>().join(" ");
        search.title_set(search.title_get() + &in_search_title);
    }

    if let Some(search_names) = matches.values_of("search-tags") {
        let in_search_tags: String = search_names.collect::<Vec<_>>().join(" ");
        let mut in_search_tags_vec: Vec<String> = Vec::<String>::new();
        in_search_tags_vec.push(in_search_tags.to_owned());
        let mut tag: Vec<String> = search.tags_get();
        tag.extend(in_search_tags_vec);
        search.tags_set(tag);
    }

    if let Some(search_names) = matches.values_of("search-description") {
        let in_search_description: String = search_names.collect::<Vec<_>>().join(" ");
        search.description_set(search.description_get() + &in_search_description);
    }

    if let Some(search_names) = matches.values_of("search-references") {
        let in_search_references: String = search_names.collect::<Vec<_>>().join(" ");
        let mut in_search_references_vec: Vec<String> = Vec::<String>::new();
        in_search_references_vec.push(in_search_references.to_owned());
        let mut references: Vec<String> = search.reference_get();
        references.extend(in_search_references_vec);
        search.reference_set(references);
    }

    let mut lake: lake::Lake;

    if let Some(url) = matches.values_of("url").and_then(|mut values| values.next()) {
        lake = lake::Lake::new(
            url,
             search
        );
    } else {
        lake = lake::Lake::default(
            search
        );
    }

    if let Some(search_names) = matches.value_of("max") {
        let result: Result<usize, _> = search_names.parse();
        match result {
            Ok(value) => {
                lake.print_top_hits(value);
            }
            Err(_) => {
                // Parsing failed
                println!("Failed to parse the max value please enter a vialed number.");
            }
        }
    } else {
        lake.print_top_hits(10); // print the sorted top lake templates.
    }

}
