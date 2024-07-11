
use crate::generic::{Services, Question, ReadmePartial, get_selected_services};
use std::io::{Error, Write};
use std::path::Path;
use std::fs::{File, create_dir_all, remove_file, read_to_string};
use tera::{Tera, Context};
use std::env;


/// Reads a line of input from the user.
///
/// This function reads a line of input from the standard input (usually the console). It trims the trailing newline
/// and returns the input as a `String`.
///
/// # Returns
///
/// * `Ok(String)` - A `String` that contains the user's input.
/// * `Err(Error)` - An `Error` object that indicates an I/O error occurred while reading the input.
///
/// # Errors
///
/// This function will return an error if an I/O error occurs while reading the input.
///
/// # Example
///
/// ```
/// let input = get_input().unwrap();
/// ```
fn get_input() -> Result<String, Error> {
    let mut input = String::new();
    match std::io::stdin().read_line(&mut input) {
        Ok(_) => Ok(input),
        Err(e) => Err(e),
    }
}


/// Asks the user questions related to each service.
///
/// This function iterates over each service in the provided `Services` object, and for each service, it iterates over
/// each question. It asks the user each question by calling the `ask_question()` function and passing the `get_input()`
/// function as the input provider.
///
/// # Arguments
///
/// * `services` - A mutable reference to a `Services` object that contains the services for which the user will be asked questions.
///
/// # Returns
///
/// * `Ok(())` - If all questions were asked successfully.
/// * `Err(Error)` - An `Error` object that indicates an I/O error occurred while reading the input.
///
/// # Errors
///
/// This function will return an error if an I/O error occurs while reading the input for any question.
///
/// # Example
///
/// ```
/// let mut services = load_services();
/// ask_services_questions(&mut services).unwrap();
/// ```
fn ask_services_questions(services: &mut Services) -> Result<(), Error>  {
    for service in services.values_mut(){
        for question in &mut service.questions {
            ask_question(question, get_input)?;
        }
    }
    Ok(())
}

/// Asks a question and stores the answer.
///
/// This function takes a mutable reference to a `Question` object and a function to get user input. It prints the question,
/// gets the user input, sanitizes it by removing non-alphanumeric characters (except for whitespace, underscores, periods, and slashes),
/// truncates it if it's longer than 50 characters, and stores it in the `answer` field of the `Question` object.
///
/// # Arguments
///
/// * `question` - A mutable reference to a `Question` object where the question is stored and the answer will be stored.
/// * `get_input` - A function that gets user input and returns a `Result<String, Error>`.
///
/// # Returns
///
/// * `Ok(())` - If the question was asked successfully and the answer was stored.
/// * `Err(Error)` - An `Error` object that indicates an I/O error occurred while reading the input.
///
/// # Errors
///
/// This function will return an error if an I/O error occurs while reading the input.
///
/// # Example
///
/// ```
/// let mut question = Question::new("What is your name?");
/// ask_question(&mut question, get_input).unwrap();
/// ```
fn ask_question<F: FnMut() -> Result<String, Error>>(question: &mut Question, mut get_input: F) -> Result<(), Error>  {
    let max_length = 50;

    println!("{}", question.question);
    let answer = match get_input() {
        Ok(input) => input,
        Err(e) => {
            eprintln!("Error reading input: {}", e);
            return Err(e.into());
        },
    };    
    let mut sanitized_answer = answer.trim().to_string().replace(|c: char| !c.is_alphanumeric() && !c.is_whitespace() && c != '_' && c != '.' && c!= '/', "");
    if sanitized_answer.len() > max_length {
        sanitized_answer.truncate(max_length);
    }
    question.answer = Some(sanitized_answer);
    
    Ok(())
}

/// Validates the output directory and creates it if necessary.
///
/// This function takes a string that represents the output directory and a function to get user input. It sanitizes the
/// directory string by replacing non-alphanumeric characters (except for underscores, periods, slashes, and hyphens) with underscores,
/// checks if the directory exists, and if it doesn't, asks the user if they want to create it.
///
/// If the directory exists, it checks if there is already a file named `docker-compose.yml` in the directory. If there
/// is, it asks the user if they want to overwrite it.
///
/// # Arguments
///
/// * `output_dir` - A string that represents the output directory.
/// * `get_input` - A function that returns a `Result<String, Error>`. This function is called to get the user's input.
///
/// # Returns
///
/// * `Ok(())` - If the output directory is valid and ready to be used.
/// * `Err(Error)` - An `Error` object that indicates an I/O error occurred while reading the input or creating the directory.
///
/// # Errors
///
/// This function will return an error if an I/O error occurs while reading the input or creating the directory.
///
/// # Example
///
/// ```
/// validate_output_dir("/path/to/output", get_input).unwrap();
/// ```
fn validate_output_dir<F: FnMut() -> Result<String, Error>>(output_dir: &str, mut get_input: F) -> Result<(), Error> {
    let sanitized_output_dir = output_dir.replace(|c: char| !c.is_alphanumeric() && c != '_' && c != '.' && c != '/' && c != '-', "_");
    let path = Path::new(&sanitized_output_dir);

    if !path.exists() {
        println!("The directory {} does not exist. Do you want to create it? (y/n)", sanitized_output_dir);
        let input = match get_input() {
            Ok(input) => input,
            Err(e) => {
                eprintln!("Error reading input: {}", e);
                return Err(e.into());
            },
        };
        if input.trim() == "y" {
            create_dir_all(path)?;
        } else {
            println!("Please enter a new directory:");
            let new_dir = match get_input() {
                Ok(input) => input,
                Err(e) => {
                    eprintln!("Error reading input: {}", e);
                    return Err(e.into());
                },
            };
            validate_output_dir(new_dir.trim(), get_input)?;
        }
    } else {
        let docker_compose_path = path.join("docker-compose.yml");
        if docker_compose_path.exists() {
            println!("There is already a file named docker-compose.yml in {}. Do you want to erase it? (y/n)", sanitized_output_dir);
            let input = match get_input() {
                Ok(input) => input,
                Err(e) => {
                    eprintln!("Error reading input: {}", e);
                    return Err(e.into());
                },
            };
            if input.trim() == "y" {
                remove_file(docker_compose_path)?;
            } else {
                println!("Terminating the program.");
                std::process::exit(1);
            }
        }
    }

    Ok(())
}

/// Generates a README file for the project.
///
/// This function takes a `Services` object that represents the selected services, a `String` that represents the output directory,
/// and a `bool` that indicates whether to overwrite existing files. It generates a README file that contains information about
/// the selected services and writes it to the output directory. If the `overwrite` parameter is `false` and a file named `README.md`
/// already exists in the output directory, the function will return an error.
///
/// # Arguments
///
/// * `services` - A `Services` object that contains the selected services.
/// * `output_dir` - A `String` that represents the output directory.
/// * `overwrite` - A `bool` that indicates whether to overwrite existing files.
///
/// # Returns
///
/// * `Ok(())` - If the README file was generated successfully.
/// * `Err(Error)` - An `Error` object that indicates an I/O error occurred while writing the file.
///
/// # Errors
///
/// This function will return an error if an I/O error occurs while writing the file or if `overwrite` is `false` and a file named `README.md`
/// already exists in the output directory.
///
/// # Example
///
/// ```
/// let services = load_services();
/// let output_dir = "/path/to/output";
/// readme_generator(&services, output_dir, true).unwrap();
/// ```
fn readme_generator(selected_services: Services, output_dir: &str) -> Result<(), Box<dyn std::error::Error>> {
    // get the path to the ryujin-cli directory
    // it is used as the base path to the templates
    let dir_path = env::var("RYUJIN_CLI_PATH").expect("The RYUJIN_CLI_PATH env variable was not found. Please set it to the path of the ryujin-cli directory");
    // Initialize a new Tera instance and tell it to look for templates in
    // `$dir_path/services/templates/readme`.
    let tera = Tera::new(&format!("{}/services/templates/readme/*", dir_path))?;

    // Create an empty `Context` instance to hold the variable for the template.
    let mut context = Context::new();

    // Prepare the context with the sub-readme of the services.
    let mut service_templates: Vec<ReadmePartial> = Vec::new();

    for (name, _) in &selected_services {
        let informations_path = format!("{dir_path}/services/templates/readme/partials/{name}-information.md");
        let configuration_path = format!("{dir_path}/services/templates/readme/partials/{name}-configuration.md");

        let informations_readme = read_to_string(&informations_path)
            .unwrap_or_else(|_| format!("Could not read {}", informations_path));
        let configuration_readme = read_to_string(&configuration_path)
            .unwrap_or_else(|_| format!("Could not read {}", configuration_path));

        service_templates.push(ReadmePartial {
            service: name.clone(),
            informations_readme,
            configuration_readme,
        });
    }
    
    context.insert("services", &service_templates);
    
    // Render `readme_compose` template with context.
    match tera.render("readme-template-readme.md", &context) {
        Ok(rendered) => {
            // Write the rendered content to the file
            let mut file = File::create(format!("{}/readme-compose.md", output_dir))?;
            file.write_all(rendered.as_bytes())?;
        },
        Err(e) => {
            eprintln!("Error rendering template: {:?}", e);
            return Err(Box::new(e));
        },
    }

    Ok(())
}

/// Generates a `docker-compose.yml` file based on the selected services and their answers to questions.
///
/// This function uses the Tera templating engine to render a `docker-compose.yml` file. It first creates a context
/// and adds the names of the selected services to it. Then, it loops over the selected services and their questions,
/// and adds the answer of each question to the context under the key of the question's variable. The context is then
/// used to render the `template-docker-compose.yml` template, which includes service-specific templates based on the
/// names of the selected services. The rendered template is written to a `docker-compose.yml` file in the specified
/// output directory.
///
/// # Arguments
///
/// * `selected_services` - A `Services` instance representing the services selected by the user. Each service has a
///   list of questions, and each question has an answer that is used to populate the service's template.
/// * `output_dir` - A string slice representing the directory where the `docker-compose.yml` file will be written.
///
/// # Returns
///
/// This function returns a `Result`. If the function succeeds, it returns `Ok(())`. If the function fails, it returns
/// `Err` with the error that occurred. Potential errors include Tera being unable to find, parse, or render a template,
/// or the program being unable to write to the `docker-compose.yml` file.
///
/// # Example
///
/// ```rust
/// let selected_services = get_selected_services();
/// let output_dir = "./output";
/// docker_compose_generator(selected_services, output_dir).unwrap();
/// ```
fn docker_compose_generator(selected_services: Services, output_dir: &str) -> Result<(), Box<dyn std::error::Error>> {
    // get the path to the ryujin-cli directory
    // it is used as the base path to the templates
    let dir_path = env::var("RYUJIN_CLI_PATH").expect("The RYUJIN_CLI_PATH env variable was not found. Please set it to the path of the ryujin-cli directory");
    
    // Initialize a new Tera instance and tell it to look for templates in the `$dir_path/services/templates` directory.
    let tera = Tera::new(&format!("{}/services/templates/compose/*", dir_path))?;

    // Create an empty `Context` instance to hold the variables for the template.
    let mut context = Context::new();

    // Add the selected services to the context.
    let service_names: Vec<String> = selected_services.keys().cloned().collect();
    context.insert("services", &service_names);

    // Add the answers to the questions to the context.
    for service in selected_services.values() {
        for question in &service.questions {
            if let Some(answer) = &question.answer {
                context.insert(&question.variable, answer);
            }
        }
    }

    // Render the `template-docker-compose.yml` template with the context.
    let docker_compose_content = tera.render("template-docker-compose.yml", &context)?;
    
// Write the `docker_compose.yml` content to a file in the `output_dir`.
    let mut file = File::create(format!("{}/docker-compose.yml", output_dir))?;
    file.write_all(docker_compose_content.as_bytes())?;

    Ok(())
}

/// Composes the Docker services based on user's choices.
///
/// This function takes a reference to the available services, a vector of user's chosen services, and an output directory.
/// It validates the output directory, selects the chosen services, asks the user questions related to each chosen service,
/// and generates the Docker compose file in the output directory.
///
/// # Arguments
///
/// * `available_services` - A reference to a `Services` object that contains all available services.
/// * `chosen_services` - A vector of `String` that represents the user's chosen services.
/// * `output_dir` - A string that represents the output directory.
///
/// # Returns
///
/// * `Ok(())` - If the Docker compose file was successfully generated.
/// * `Err(Error)` - An `Error` object that indicates an error occurred while validating the output directory, selecting the services, asking the questions, or generating the Docker compose file.
///
/// # Errors
///
/// This function will return an error if an error occurs while validating the output directory, selecting the services, asking the questions, or generating the Docker compose file.
///
/// # Example
///
/// ```
/// let services = load_services();
/// let user_choice = vec!["apache".to_string(), "mongodb".to_string()];
/// compose(&services, user_choice, "./output").unwrap();
/// ```
pub fn handle(available_services: &Services, choosen_services: Vec<String>, output_dir: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Validate the output directory
    validate_output_dir(output_dir, get_input)?;

    // Get the selected services
    let mut selected_services = get_selected_services(available_services, choosen_services)?;

    // Ask questions for the selected services
    ask_services_questions(&mut selected_services)?;

    // Generate the docker compose file
    docker_compose_generator(selected_services.clone(), output_dir)?;

    // Generate the readme for the docker compose 
    readme_generator(selected_services, output_dir)?;
    
    Ok(())
}

