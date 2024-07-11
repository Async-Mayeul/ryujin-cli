use crate::generic::Services;
use std::io::{Error, ErrorKind};


/// Prints the catalog of services.
///
/// This function takes a `Services` object that represents the available services and prints a list of the service names.
/// The services are sorted in alphabetical order before being printed.
///
/// # Arguments
///
/// * `services` - A `Services` object that contains the available services.
///
/// # Example
///
/// ```
/// let services = load_services();
/// print_catalog(&services);
/// ```
fn print_catalog(services: Services){
    println!("Available services to add to your docker-compose:\n");
    // Print the name of each service with 3 columns of services per row
    let mut count = 0; // to track the number of services printed in a row
    for service in services.keys(){
        print!("{:<20}", service);// print the service name with a width of 20 characters
        count += 1;
        if count == 3 { // if 3 services have been printed, start a new line
            println!();
            count = 0;
        }
    }

}

/// Prints a detailed catalog of services.
///
/// This function takes a `Services` object that represents the available services and prints a detailed list of the services.
/// For each service, it prints the service name, description, and version. The services are sorted in alphabetical order before being printed.
///
/// # Arguments
///
/// * `services` - A `Services` object that contains the available services.
///
/// # Example
///
/// ```
/// let services = load_services();
/// print_detailed_catalog(&services);
/// ```
fn print_detailed_catalog(services: Services){
    println!("Available services to add to your docker-compose:\n");
    // One service per line, with 4 columns of information per service ( name, description, current version, tags)

    // Print the header
    println!("{:<20} {:<70} {:<20} {:<20}\n", "Name", "Description", "Current Version", "Tags");

    // Print the services
    for service in services.values(){
        println!("{:<20} {:<70} {:<20} {:<20}", service.name, service.description, service.current_version, service.tags.join(", "));
    }
}

/// Filters the catalog of services by tags.
///
/// This function takes a mutable reference to a `Services` object and a vector of tags. It removes any services from the `Services` object that do not have any of the specified tags.
///
/// # Arguments
///
/// * `services` - A mutable reference to a `Services` object that contains the available services.
/// * `tags` - A vector of strings representing the tags to filter by.
///
/// # Errors
///
/// This function will return an error if no services are found with the specified tags.
///
/// # Example
///
/// ```
/// let mut services = load_services();
/// let tags = vec!["database", "web"];
/// match filter_catalog_by_tags(&mut services, tags) {
///     Ok(_) => println!("Catalog filtered successfully"),
///     Err(e) => println!("Error filtering catalog: {}", e),
/// }
/// ```
fn filter_catalog_by_tags(services: &mut Services, tags: Vec<String>) -> Result<(), Error> {
    // remove services from services that do not have any of the tags
    let mut services_to_remove = Vec::new();
    for service in services.values(){
        let mut found = false;
        for tag in &tags {
            if service.tags.contains(tag) {
                found = true;
                break;
            }
        }
        if !found {
            services_to_remove.push(service.name.clone());
        }
    }

    for service_name in services_to_remove {
        services.remove(&service_name);
    }

    // if no services are left, return an error
    if services.is_empty() {
        return Err(Error::new(ErrorKind::InvalidInput, "No services found with the specified tags."));
    }

    Ok(())
}

/// Filters the catalog of services by name.
///
/// This function takes a mutable reference to a `Services` object and a name string. It removes any services from the `Services` object that do not contain the name string in their name.
///
/// # Arguments
///
/// * `services` - A mutable reference to a `Services` object that contains the available services.
/// * `name` - A string representing the name to filter by.
///
/// # Errors
///
/// This function will return an error if no services are found containing the specified name.
///
/// # Example
///
/// ```
/// let mut services = load_services();
/// let name = "database";
/// match filter_catalog_by_name(&mut services, name) {
///     Ok(_) => println!("Catalog filtered successfully"),
///     Err(e) => println!("Error filtering catalog: {}", e),
/// }
/// ```
fn filter_catalog_by_name(services: &mut Services, name: String) -> Result<(), Error> {
    // remove services from services that do not contain the name string in their name
    let mut services_to_remove = Vec::new();
    for service in services.values(){
        if !service.name.contains(&name) {
            services_to_remove.push(service.name.clone());
        }
    }

    for service_name in services_to_remove {
        services.remove(&service_name);
    }

    // if no services are left, return an error
    if services.is_empty() {
        return Err(Error::new(ErrorKind::InvalidInput, "No services found containing the specified name."));
    }

    Ok(())
}

/// Filters and prints the catalog of services.
///
/// This function takes a mutable reference to a `Services` object, a boolean indicating whether to print detailed information,
/// an optional vector of tags, and an optional name. It filters the services based on the tags and name, if provided, and then
/// prints the catalog. If `detailed` is true, it prints a detailed catalog; otherwise, it prints a simple catalog.
///
/// # Arguments
///
/// * `services` - A mutable reference to a `Services` object that contains the available services.
/// * `detailed` - A boolean indicating whether to print a detailed catalog.
/// * `tags` - An optional vector of tags to filter the services.
/// * `name` - An optional name to filter the services.
///
/// # Errors
///
/// This function will return an error if the filtering fails.
///
/// # Example
///
/// ```
/// let mut services = load_services();
/// let detailed = true;
/// let tags = Some(vec!["tag1".to_string(), "tag2".to_string()]);
/// let name = Some("service1".to_string());
/// catalog(&mut services, detailed, tags, name);
/// ```
pub fn handle(services : &mut Services, detailed: bool, tags: Option<Vec<String>>, name: Option<String>) -> Result<(), Error> {
    // filter the services based on the tags
    if let Some(tags) = tags {
        filter_catalog_by_tags(services, tags)?;
    }

    // filter the services based on the name
    if let Some(name) = name {
        filter_catalog_by_name(services, name)?;
    }

    // print the catalog
    if detailed {
        print_detailed_catalog(services.clone());
        Ok(())
    } else {
        print_catalog(services.clone());
        Ok(())
    }
    
}




