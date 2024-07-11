mod compose;
mod service;
mod select;
mod catalog;



mod generic{
    use serde::{Deserialize, Serialize};
    use std::collections::HashMap;
    use std::fs::{File, OpenOptions};
    use std::io::{Read, Error, ErrorKind, BufReader, Write};
    use serde_json::{Value, json};
    use std::env;


    /// `Question` is a struct that represents a question in the JSON data.
    ///
    /// It contains the following fields:
    /// * `question`: A `String` that represents the text of the question.
    /// * `variable`: A `String` that represents the variable associated with the question.
    /// * `answer`: An `Option<String>` that represents the answer to the question. This field is not present in the JSON data and is used to store the user's answer.
    ///
    /// This struct is used to parse the JSON data and store the information about a question.
    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct Question {
        pub question: String,
        pub variable: String,
        pub answer: Option<String>,
    }

    /// `Service` is a struct that represents a service in the JSON data.
    ///
    /// It contains the following fields:
    /// * `name`: A `String` that represents the name of the service.
    /// * `description`: A `String` that describes the service.
    /// * `current_version`: A `String` that represents the current version of the service.
    /// * `is_modified`: A `bool` that indicates if the Docker image has been modified before being added to the catalog.
    /// * `last_update`: A `String` that represents the date of the last update to the service.
    /// * `developers`: A `String` that represents the name of the developers of the service.
    /// * `links`: A `HashMap<String, String>` that contains links to the service's documentation, source code, and other resources.
    /// * `tags`: A `Vec<String>` that contains the service's tags.
    /// * `template_path`: A `String` that represents the path to the template for the service.
    /// * `variables`: A `Vec<String>` that contains the variables associated with the service.
    /// * `questions`: A `Vec<Question>` that contains the questions associated with the service.
    ///
    /// This struct is used to parse the JSON data and store the information about a service.
    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct Service {
        pub name: String,
        pub description: String,
        pub current_version: String,
        pub is_modified: bool,
        pub last_update: String,
        pub developers: String,
        pub links: HashMap<String, String>,
        pub tags: Vec<String>,
        pub template_path: String,
        pub variables: Vec<String>,
        pub questions: Vec<Question>,
    }

    /// `ReadmePartial` is a struct that represents a part of a README file.
    ///
    /// It contains the following fields:
    /// * `service`: A `String` that represents the name of the service.
    /// * `informations_readme`: A `String` that contains the information section of the README.
    /// * `configuration_readme`: A `String` that contains the configuration section of the README.
    ///
    /// This struct is used to generate a README file for a service by combining these parts.
    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct ReadmePartial {
        pub service: String,
        pub informations_readme: String,
        pub configuration_readme: String,
    }

    /// `Services` is a type alias for a `HashMap` where the key is a `String` representing the name of a service,
    /// and the value is a `Service` struct representing the details of the service.
    ///
    /// This type is used to store and manipulate a collection of services in the program.
    pub type Services = HashMap<String, Service>;

    /// Loads the services data from a JSON file.
    ///
    /// This function opens the JSON file at the path "../../services/services.json", reads its contents into a string,
    /// and then deserializes the string into a `Services` object using the `serde_json::from_str` function.
    ///
    /// # Returns
    ///
    /// A `Services` object representing the services data. If the file cannot be opened, read, or parsed, the function will panic.
    ///
    /// # Panics
    ///
    /// This function will panic if the file cannot be opened, read, or parsed into a `Services` object.
    pub fn load_services() -> Services {
        // get the path to the ryujin-cli directory
        // it is used as the base path to the services.json file
        let dir_path = env::var("RYUJIN_CLI_PATH").expect("The RYUJIN_CLI_PATH env variable was not found. Please set it to the path of the ryujin-cli directory");
        
        let mut file = File::open(&format!("{}/services/services.json", dir_path)).expect(&format!("Unable to open file : {}/services/services.json", dir_path));
        let mut contents = String::new();
        file.read_to_string(&mut contents).expect(&format!("Unable to read file : {}/services/services.json", dir_path));
        let services: Services = serde_json::from_str(&contents).expect(&format!("Unable to parse JSON : {}/services/services.json", dir_path));
        services
    }

    /// Loads the user's selection from a JSON file.
    ///
    /// This function opens the `conf/conf.json` file and reads its contents. It then looks for the
    /// `selected_services` field in the JSON object and attempts to interpret it as an array.
    ///
    /// Each item in the array is expected to be a string. These strings are collected into a `Vec<String>`
    /// and returned. If the array is empty, `None` is returned instead.
    ///
    /// # Errors
    ///
    /// This function will return an error if the `conf/conf.json` file cannot be opened, if the file's
    /// contents cannot be parsed as JSON, or if the `selected_services` field is not an array.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// let selection = load_selection_from_json().unwrap();
    /// match selection {
    ///     Some(services) => println!("Selected services: {:?}", services),
    ///     None => println!("No services selected"),
    /// }
    /// ```
    pub fn load_selection_from_json() -> Result<Option<Vec<String>>, Box<dyn std::error::Error>> {
        // get the path to the ryujin-cli directory
        // it is used as the base path to the conf.json file
        let dir_path = env::var("RYUJIN_CLI_PATH").expect("The RYUJIN_CLI_PATH env variable was not found. Please set it to the path of the ryujin-cli directory");
        
        let file = File::open(format!("{dir_path}/conf/conf.json"))?;
        let reader = BufReader::new(file);
        let json: Value = serde_json::from_reader(reader)?;

        let selected_services = json["selected_services"].as_array()
        .ok_or_else(|| Error::new(std::io::ErrorKind::InvalidData, "selected_services is not an array."))?;

        let services : Vec<String> = selected_services.iter().map(|s| s.as_str().expect("Not a string").to_owned()).collect();
        
        if services.is_empty(){
            Ok(None)
        } else {
            Ok(Some(services))
        }
    }

    /// Saves the user's selection to a JSON file.
    ///
    /// This function opens the `conf/conf.json` file and reads its contents into a JSON object.
    /// It then replaces the `selected_services` field in the JSON object with the provided user selection.
    ///
    /// The updated JSON object is then written back to the `conf/conf.json` file.
    ///
    /// # Arguments
    ///
    /// * `user_selection` - A vector of strings representing the user's current selection of services.
    ///
    /// # Errors
    ///
    /// This function will return an error if the `conf/conf.json` file cannot be opened, if the file's
    /// contents cannot be parsed as JSON, or if the file cannot be written to.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// let selection = vec!["service1", "service2", "service3"];
    /// match save_selection_to_json(&selection) {
    ///     Ok(_) => println!("Selection saved successfully"),
    ///     Err(e) => println!("Error saving selection: {}", e),
    /// }
    /// ```
    pub fn save_selection_to_json(user_selection: &Vec<String>) -> Result<(), Box<dyn std::error::Error>>{
        // get the path to the ryujin-cli directory
        // it is used as the base path to the templates
        let dir_path = env::var("RYUJIN_CLI_PATH").expect("The RYUJIN_CLI_PATH env variable was not found. Please set it to the path of the ryujin-cli directory");
        
        let file = File::open(format!("{dir_path}/conf/conf.json"))?;
        let reader = BufReader::new(file);
        let mut json: Value = serde_json::from_reader(reader)?;

        let selection_json = json!(user_selection);

        if let Some(obj) = json.as_object_mut() {
            obj.insert("selected_services".to_string(), selection_json);
        }

        let file = OpenOptions::new().write(true).truncate(true).open(format!("{dir_path}/conf/conf.json"))?;
        let mut file = std::io::BufWriter::new(file);

        
        write!(file, "{}", json.to_string())?;

        Ok(())
    }

    /// Selects services based on user choice.
    ///
    /// This function takes a reference to a `Services` object and a vector of user choices. It iterates over the user choices,
    /// and for each choice, it tries to find a corresponding service in the `Services` object. If a service is found, it is
    /// added to a new `Services` object that will be returned. If a service is not found, the choice is added to a list of
    /// not found services.
    ///
    /// # Arguments
    ///
    /// * `services` - A reference to a `Services` object that contains all available services.
    /// * `user_choice` - A vector of `String` that represents the user's choices.
    ///
    /// # Returns
    ///
    /// * `Ok(Services)` - A `Services` object that contains only the services selected by the user.
    /// * `Err(Error)` - An `Error` object with `ErrorKind::InvalidInput` and a message that lists the services that were not found.
    ///
    /// # Errors
    ///
    /// This function will return an error if one or more of the user's choices do not correspond to a service in the `Services` object.
    ///
    /// # Example
    ///
    /// ```
    /// let services = load_services();
    /// let user_choice = vec!["service1".to_string(), "service2".to_string()];
    /// let selected_services = get_selected_services(&services, user_choice).unwrap();
    /// ```
    pub fn get_selected_services(services: &Services, user_choice: Vec<String>) -> Result<Services, Error> {
        let mut selected_services = Services::new();
        let mut not_found: Vec<String> = Vec::new();

        for choice in user_choice {
            match services.get(&choice) {
                Some(service) => { selected_services.insert(choice, service.clone()); },
                None => { not_found.push(choice); },
            }
        }

        if !not_found.is_empty() {
            let error_message = format!("The following services were not found: {:?}. Use the catalog command to get a list of available services.", not_found);
            Err(Error::new(ErrorKind::InvalidInput, error_message))
        } else {
            Ok(selected_services)
        }
    }


}




use clap::{command, Arg, Command};


use crate::generic::{load_services, load_selection_from_json, save_selection_to_json, Services};
/// Entry point of the Ryujin-CLI application.
///
/// This function is responsible for parsing command line arguments and executing the corresponding actions.
/// It supports several subcommands:
/// - `compose`: Starts the process of creating a docker-compose file. It takes optional `services` argument and a required `output-dir` argument.
/// - `catalog`: Displays the list of services to add to the docker-compose. It takes optional `long`, `tags`, and `name` arguments.
/// - `service`: Displays detailed information about a specific service. It takes a required `service_name` argument.
/// - `select`: Allows you to save a service selection and update it before using the compose command. It takes optional `new`, `add`, `delete`, `remove`, `print`, and `services` arguments.
///
/// Each subcommand has its own set of arguments and behaviors.
///
/// # Errors
/// This function will exit the process with a non-zero status code if an error occurs.

fn main(){
    let mut services: Services = load_services();

    let matches = command!()
        .subcommand(Command::new("compose")
            .about("Start the process of creating a docker-compose.")
            .arg(Arg::new("services")
                .short('s')
                .long("services")
                .required(false) // TEST
                .value_delimiter(',')
                .help(" List of services to add to the docker-compose.")
            )
            .arg(Arg::new("output-dir")
                .short('o')
                .long("output-dir")
                .required(true)
                .help("Path of the output directory where the docker-compose should be created.")
            )
        )
        .subcommand(Command::new("catalog")
            .about("Displays the list of services you can add to your docker-compose.")
            .arg(Arg::new("long")
                .short('l')
                .long("long")
                .required(false)
                .help("Display detailed information about the services.")
                .num_args(0)
            )
            .arg(Arg::new("tags")
                .short('t')
                .long("tags")
                .required(false)
                .value_delimiter(',')
                .help("Filter services by tags.")
            )
            .arg(Arg::new("name")
                .short('n')
                .long("name")
                .required(false)
                .help("Filter services by name.")
            )
        )
        .subcommand(Command::new("service")
            .about("Displays detailed information about a specific service.")
            .arg(Arg::new("service_name")
                .required(true)
                .help("The name of the service to display.")
            )
        )
        .subcommand(Command::new("select")
            .about("The select command allows you to save a service selection and update it before using the compose command.")
            .arg(Arg::new("new")
                .short('n')
                .long("new")
                .required(false)
                .help("Create a new selection.")
                .num_args(0)
                )
            .arg(Arg::new("add")
                .short('a')
                .long("add")
                .required(false)
                .help("Add some services to the current selection.")
                .requires("services")
                .num_args(0)
            )
            .arg(Arg::new("delete")
                .short('d')
                .long("delete")
                .required(false)
                .help("Delete the current selection.")
                .num_args(0)
                .conflicts_with_all(["new", "services", "add", "print", "remove"])
            )
            .arg(Arg::new("remove")
                .short('r')
                .long("remove")
                .required(false)
                .help("Remove some services from the current selection.")
                .requires("services")
                .num_args(0)
                .conflicts_with_all(["add", "new"])
            )
            .arg(Arg::new("print")
                .short('p')
                .long("print")
                .required(false)
                .help("Print the current selection.")
                .num_args(0)
            )
            .arg(Arg::new("services")
                .short('s')
                .long("services")
                .required(false)
                .value_delimiter(',')//TODO autres values delimiter 
                .help("The service you want")
            )
        )
          
        .get_matches();


    match matches.subcommand() {
        Some(("compose", compose_matches)) => {
            let mut choosen_services: Vec<String> = Vec::new();
            if let Some(services) = compose_matches.get_many::<String>("services"){
                for service in services {
                    choosen_services.push(service.to_string());
                }
            } else {
                match load_selection_from_json() {
                    Ok(Some(json_services)) => {
                        for services in json_services.iter(){
                            choosen_services.push(services.to_string());
                        }
                    }
                    Ok(None) => {
                        println!("The current selection is empty. Please add services to the selection or use the --services option.");
                        std::process::exit(1)  
                    }
                    Err(err) => {
                        println!("Loading JSON error: {}", err);
                        std::process::exit(1)
                    }
                    
                };
            }
            let mut output_dir = String::new();
            if let Some(dir) = compose_matches.get_one::<String>("output-dir"){
                output_dir = dir.to_string();
            }
            match compose::handle(&services, choosen_services, &output_dir) {
                Ok(_) => println!("Docker compose file generated successfully"),
                Err(e) => eprintln!("Error generating docker compose file: {}", e),
            }        
        }
        Some(("catalog", catalog_matches)) => {
            let mut detailed = false;
            if let Some(val) = catalog_matches.get_one::<bool>("long"){
                detailed = *val;
            }
            let mut tags = None;
            if let Some(tags_list) = catalog_matches.get_many::<String>("tags"){
                let tags_vec: Vec<String> = tags_list.into_iter().map(|s| s.to_string()).collect();
                tags = Some(tags_vec);
            }
            let mut name = None;
            if let Some(name_str) = catalog_matches.get_one::<String>("name"){
                name = Some(name_str.to_string());
            }
            catalog::handle(&mut services, detailed, tags, name).unwrap();
        }
        Some(("service", service_matches)) => {
            let service_name = service_matches.get_one::<String>("service_name").unwrap();
            match services.get(service_name) {
                Some(service) => service::print_service_page(service),
                None => eprintln!("Service not found"),
            }
        }
        Some(("select", select_matches)) => {
            let mut user_selection = match load_selection_from_json() {
                Ok(Some(user_selection)) => user_selection,
                Ok(None) => Vec::new(),
                Err(err) => {
                    println!("Loading JSON error: {}", err);
                    std::process::exit(1)
                }
            
                
            };

            match select::handle(&mut user_selection, select_matches, &services) {
                Ok(_) => {
                    match save_selection_to_json(&user_selection){
                        Ok(_) => {
                            println!("\n");
                    }
                        Err(e) => {
                            eprintln!("Selection saving error : {}",e);
                            std::process::exit(1)
                        }
                    }
                }
                Err(e) => eprintln!("Error updating selection: {}", e),
            }
            
            
            
        }

        _ => println!("No known subcommand was used"),
    }

   
}

