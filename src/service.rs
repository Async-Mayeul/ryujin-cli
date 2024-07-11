use crate::generic::Service;

/// Prints the details of a specific service.
///
/// This function takes a reference to a `Service` object and prints its details, including the name, description, current version,
/// modification status, last update, developers, links, and tags. The description is formatted to a width of 80 characters.
///
/// # Arguments
///
/// * `service` - A reference to a `Service` object whose details are to be printed.
///
/// # Example
///
/// ```
/// let service = load_service("service1");
/// print_service_page(&service);
/// ```
pub fn print_service_page(service: &Service) {
    println!("————————————————————————————————————————");
    println!("Details about {}", service.name);
    println!("————————————————————————————————————————\n");
    println!("Name: {}", service.name);
    println!("Description:");
    let description = format!("{: <80}", service.description);
    for line in description.as_str().chars().collect::<Vec<char>>().chunks(80) {
        let line: String = line.into_iter().collect();
        println!("    {}", line);
    }
    println!("\nCurrent Version: {}", service.current_version);
    println!("Modified: {}", if service.is_modified { "Yes" } else { "No" });
    println!("\nLast Update: {}", service.last_update);
    println!("Developers: {}", service.developers);
    println!("\nLinks:");
    for (key, value) in &service.links {
        println!("- {}: {}", key, value);
    }
    println!("\nTags: {}", service.tags.join(", "));
    println!("————————————————————————————————————————");
}

