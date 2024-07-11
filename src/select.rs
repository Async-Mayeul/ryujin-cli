use crate::generic::Services;
use std::io::{Error, ErrorKind};
use std::collections::HashSet;
use clap::ArgMatches;


/// Checks if the provided services exist in the catalog.
///
/// This function takes a vector of service names and a `Services` object that represents the catalog of available services.
/// It checks if each service in the vector exists in the catalog. If a service does not exist, it returns an error.
///
/// # Arguments
///
/// * `service` - A vector of service names to check.
/// * `catalog` - A `Services` object that represents the catalog of available services.
///
/// # Errors
///
/// This function will return an error if a service in the vector does not exist in the catalog.
///
/// # Example
///
/// ```
/// let services = vec!["service1".to_string(), "service2".to_string()];
/// let catalog = load_services();
/// select_check_catalog(&services, &catalog);
/// ```
fn select_check_catalog(service: &Vec<String>, catalog : &Services) -> Result<(), Error>{
    
    let catalog_keys: HashSet<String> = catalog.keys().map(|k| k.clone()).collect();

    for item in service.iter() {
        if !catalog_keys.contains(item) { 
            return Err(Error::new(ErrorKind::InvalidData, format!("Requested service {} is not in the available services.", item)));
        }
    }

    Ok(())
}

/// Erases the current user selection.
///
/// This function checks if the user selection is empty. If it is, it returns an error.
/// If the user selection is not empty, it clears the selection.
///
/// # Arguments
///
/// * `user_selection` - A mutable reference to a vector of strings representing the user's current selection of services.
///
/// # Errors
///
/// This function will return an error if the user selection is empty.
///
/// # Examples
///
/// ```no_run
/// let mut selection = vec!["service1", "service2", "service3"];
/// match erase_current_selection(&mut selection) {
///     Ok(_) => println!("Selection erased successfully"),
///     Err(e) => println!("Error erasing selection: {}", e),
/// }
/// ```
fn erase_current_selection(user_selection: &mut Vec<String>) -> Result<(), Error>{
    
    match user_selection.is_empty(){
        true => return Err(Error::new(ErrorKind::InvalidInput, "Selection is empty.")),
        false => user_selection.clear(),
    }

    Ok(())
} 

/// Adds new services to the current user selection.
///
/// This function iterates over the `new_services` vector. For each item, it checks if the item is already
/// in the `user_selection` vector. If it's not, the item is added to `user_selection` and a message is printed
/// indicating that the item has been added. If the item is already in `user_selection`, a message is printed
/// indicating that the item is already in the list.
///
/// # Arguments
///
/// * `user_selection` - A mutable reference to a vector of strings representing the current user selection.
/// * `new_services` - A reference to a vector of strings representing the new services to be added.
///
/// # Returns
///
/// * `Result<(), Error>` - Returns `Ok(())` if the operation is successful, otherwise returns an `Error`.
///
/// # Examples
///
/// ```rust
/// let mut user_selection = vec!["service1".to_string(), "service2".to_string()];
/// let new_services = vec!["service3".to_string(), "service4".to_string()];
/// add_to_current_selection(&mut user_selection, &new_services);
/// ```
fn add_to_current_selection(user_selection: &mut Vec<String>, new_services: &Vec<String>) -> Result<(), Error> {

    for item in new_services.iter(){
        if !user_selection.contains(item){
            user_selection.push(item.to_string());
            println!("The item {} has been added to the selection.", item)
        } else {
            println!("The item {} is already in the current selection !", item)
        }
    }
    Ok(())
}

/// Removes specified services from the current user selection.
///
/// This function checks if the user selection is empty. If it is, it returns an error.
/// If the user selection is not empty, it iterates over the services to remove and removes them from the user selection.
///
/// # Arguments
///
/// * `user_selection` - A mutable reference to a vector of strings representing the user's current selection of services.
/// * `services_to_remove` - A reference to a vector of strings representing the services to be removed from the user's current selection.
///
/// # Errors
///
/// This function will return an error if the user selection is empty.
///
/// # Examples
///
/// ```no_run
/// let mut selection = vec!["service1", "service2", "service3"];
/// let services_to_remove = vec!["service2"];
/// match remove_from_current_selection(&mut selection, &services_to_remove) {
///     Ok(_) => println!("Services removed successfully"),
///     Err(e) => println!("Error removing services: {}", e),
/// }
/// ```
fn remove_from_current_selection(user_selection: &mut Vec<String>, services_to_remove: &Vec<String>) -> Result<(), Error> {
        
    if user_selection.is_empty(){
        return Err(Error::new(ErrorKind::InvalidInput, "Selection is empty."))
    }

    for item in services_to_remove.iter(){
        match user_selection.contains(item){
            true => {
                user_selection.retain(|x| x != item);
                println!("The item {} has been removed.", item)
            },
            false => println!("The item {} is not in the current selection.",item)
        }
    }

    Ok(())
}

/// Prints the current user selection of services.
///
/// This function checks if the user selection is empty. If it is, it returns an error.
/// If the user selection is not empty, it iterates over the selection and prints each service.
///
/// # Arguments
///
/// * `user_selection` - A reference to a vector of strings representing the user's current selection of services.
///
/// # Errors
///
/// This function will return an error if the user selection is empty.
///
/// # Examples
///
/// ```no_run
/// let selection = vec!["service1", "service2", "service3"];
/// match print_current_selection(&selection) {
///     Ok(_) => println!("Selection printed successfully"),
///     Err(e) => println!("Error printing selection: {}", e),
/// }
/// ```
fn print_current_selection(user_selection: &Vec<String>) -> Result<(), Error> {
    
    match user_selection.is_empty(){
        true => return Err(Error::new(ErrorKind::InvalidInput, "Selection is empty.")),
        false => {
            println!("Current selection:");
            for item in user_selection.iter(){
                println!("- {}\n", item)
            }
        }
    }
    
    Ok(())
}

pub fn handle(mut user_selection: &mut Vec<String>, select_matches: &ArgMatches, catalog : &Services) -> Result<(), Error> {
    
    if let Some(true) = select_matches.get_one::<bool>("new") { 
        if let Some(servicess) = select_matches.get_many::<String>("services"){ 
             let new_services: Vec<String> = servicess.map(|s| s.to_string().to_lowercase()).collect();
             if let Err(e) = select_check_catalog(&new_services, &catalog){
                //  println!("Error : {} \n", e);
                //  std::process::exit(1)
                return Err(e)
             }
             
             user_selection.clear();
             println!("New selection created!");

             if let Err(e) = add_to_current_selection(&mut user_selection, &new_services){
                // println!("Error : {}", e)
                return Err(e)
             }
              // peut être remplacer le match par add_to_current_selection(&mut user_selection, &services)
             //     //.unwrap_or_else(|err| println!("Une erreur est survenue: {}", err));
         
         } else {
             match erase_current_selection(&mut user_selection){
                 Ok(_) => println!("New selection created!"),
                 //Err(_) => println!("Selection is already empty.") 
                 Err(e) => return Err(e)
             }
         }
         
    } 

     if let Some(true) = select_matches.get_one::<bool>("add"){
         if let Some(servicess) = select_matches.get_many::<String>("services"){
             let new_services: Vec<String> = servicess.map(|s| s.to_string().to_lowercase()).collect();//gérer le cas ou l'élément est deja dans la liste
             if let Err(e) = select_check_catalog(&new_services, &catalog){
                //  println!("Error : {} \n ", e);
                //  std::process::exit(1)
                return Err(e)
             }
             
             if let Err(e) = add_to_current_selection(&mut user_selection, &new_services){
                //  println!("Error : {}", e)
                return Err(e)
             }

         }
     
     
     }

     if let Some(true) = select_matches.get_one::<bool>("delete"){
         match erase_current_selection(&mut user_selection){
             Ok(_) => println!("Selection deleted!"),
             Err(e) => {
                //  println!("Error : {}. Selection was not updated.", e);
                //  std::process::exit(1);
                return Err(e)
             }
         }
         
         
     }

     if let Some(true) = select_matches.get_one::<bool>("remove"){
         if let Some(services) = select_matches.get_many::<String>("services"){
             let services: Vec<String> = services.map(|s| s.to_string().to_lowercase()).collect();
             
             if let Err(e) = remove_from_current_selection(&mut user_selection, &services){
                //  println!("Error : {}", e);
                //  std::process::exit(1)
                return Err(e)
             }
         }
     }

     if let Some(true) = select_matches.get_one::<bool>("print"){ 
         if let Err(e) = print_current_selection(&user_selection) {
            //  println!("{}", e);
            //  std::process::exit(1)
            return Err(e)
         }
     
     // prendre en compte la casse car actuelement ça ne fonctionne ap 
     //revoir certains retour d'erreur dans mes fonctions et changer des print pour Err(e) => println!("Error : {}" : e)
     //revoir si certaines function doit renvoyer forcement qqch car voir si besoin return error 
     //revoir si quand je déclarer que ma fonction return une erreur, regarder dans la fonction si je return bien une erreur
     }

    Ok(())
}