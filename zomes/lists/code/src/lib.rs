#![feature(try_from)]
#[macro_use]
extern crate hdk;
#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate holochain_core_types_derive;
use serde_json::value::Value;

//use serde_json::{Result, Value};
use hdk::{
    error::ZomeApiResult,
    holochain_core_types::{
        hash::HashString,
        error::HolochainError,
        dna::entry_types::Sharing,
        json::JsonString,
        cas::content::Address,
        entry::Entry,
    }
};


 
define_zome! {
    entries: [
        entry!(
            name: "list",
            description: "",
            sharing: Sharing::Public,
            validation_package: || hdk::ValidationPackageDefinition::Entry,
            validation: |validation_data: hdk::EntryValidationData<List>| {
                Ok(())
            },
            links: [
                to!(
                    "listItem",
                    link_type: "items",
                    validation_package: || hdk::ValidationPackageDefinition::Entry,
                    validation: |_validation_data: hdk::LinkValidationData| {
                        Ok(())
                    }
                )
            ]
        ),
        entry!(
            name: "listItem",
            description: "",
            sharing: Sharing::Public,
            validation_package: || hdk::ValidationPackageDefinition::Entry,
            validation: |validation_data: hdk::EntryValidationData<ListItem>| {
                Ok(())
            }
        )
    ]
 
    genesis: || {
        Ok(())
    }
 
	functions: [
        create_list: {
            inputs: |list: List|,
            outputs: |result: ZomeApiResult<Address>|,
            handler: handle_create_list
        }
        add_item: {
            inputs: |list_item: ListItem, list_addr: HashString|,
            outputs: |result: ZomeApiResult<Address>|,
            handler: handle_add_item
        }
        get_list: {
            inputs: |list_addr: HashString, link_tag: String,search: JsonString|,
            outputs: |result: ZomeApiResult<GetListResponse>|,
            handler: handle_get_list
        }
    ]
    traits: {
        hc_public [create_list, add_item, get_list]
    }
}


#[derive(Serialize, Deserialize, Debug, Clone, DefaultJson)]
struct List {
    name: String
}

#[derive(Serialize, Deserialize, Debug, Clone, DefaultJson)]
struct ListItem {
    entityType: String,
    item: JsonString
}

#[derive(Serialize, Deserialize, Debug, Clone, DefaultJson)]
struct SearchObject {
    entityType: String,
    item: JsonString
}

#[derive(Serialize, Deserialize, Debug, DefaultJson)]
struct GetListResponse {
    name: String,
    items: Vec<ListItem>
}




fn handle_create_list(list: List) -> ZomeApiResult<Address> {
    // define the entry
    let list_entry = Entry::App(
        "list".into(),
        list.into()
    );

    // commit the entry and return the address
	hdk::commit_entry(&list_entry)
}


fn handle_add_item(list_item: ListItem, list_addr: HashString) -> ZomeApiResult<Address> {
    let clone_item:ListItem = list_item.clone();
    // define the entry
    let list_item_entry = Entry::App(
        "listItem".into(),
        list_item.into()
    );
    
    
	let item_addr = hdk::commit_entry(&list_item_entry)?; // commit the list item
	hdk::link_entries(&list_addr, &item_addr, "items",&clone_item.entityType)?; // if successful, link to list address
	Ok(item_addr)
}


fn handle_get_list(list_addr: HashString,link_tag: String,search:JsonString) -> ZomeApiResult<GetListResponse> {

    // load the list entry. Early return error if it cannot load or is wrong type
    let list = hdk::utils::get_as_type::<List>(list_addr.clone())?;

    // try and load the list items, filter out errors and collect in a vector
    let list_items = hdk::get_links(&list_addr, Some("items".into()),Some(link_tag.into()))?.addresses()
        .iter()
        .map(|item_address| {
            
            hdk::utils::get_as_type::<ListItem>(item_address.to_owned())
        })
        
    /*    .map(|item:ListItem| {
            match searchSomething(search, item){
            _ =>false,

        }})*/
        .filter_map(Result::ok)
        .collect::<Vec<ListItem>>();

   
   
    // if this was successful then return the list items
    Ok(GetListResponse{
        name: list.name,
        items: list_items
    })

}
/*
fn searchSomething(search:SearchObject,item:ListItem)->ZomeApiResult<bool> {
println!("{}",&search.item);
//let s: SearchObject = SearchObject::try_from(&search.item.to_string())?;
//let e: SearchObject = GetSearchResponse::try_from(&item.item.to_string())?;

    // Access parts of the data by indexing with square brackets.
   // println!("Please call {} at the number {}", v["name"], v["phones"][0]);    
     //    if s["id"] != e["id"]{ Ok(GetSearchResponse{found:false}) } 
       // if search.item.name != item..item.name { false }
        
   
    Ok(true)
}
*/
