#![feature(try_from)]
#[macro_use]
extern crate hdk;
#[macro_use]
extern crate serde_derive;



#[macro_use]
extern crate holochain_core_types_derive;

use hdk::utils;
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


use serde_json;
use serde_json::value::Value;


 
define_zome! {
    entries: [
        entry!(
            name: "collection",
            description: "",
            sharing: Sharing::Public,
            validation_package: || hdk::ValidationPackageDefinition::Entry,
            validation: |validation_data: hdk::EntryValidationData<Collection>| {
                Ok(())
            },
            links: [
                to!(
                    "HoloJsEntry",
                    link_type: "items",
                    validation_package: || hdk::ValidationPackageDefinition::Entry,
                    validation: |_validation_data: hdk::LinkValidationData| {
                        Ok(())
                    }
                )
            ]
        ),
        entry!(
            name: "HoloJsEntry",
            description: "",
            sharing: Sharing::Public,
            validation_package: || hdk::ValidationPackageDefinition::Entry,
            validation: |validation_data: hdk::EntryValidationData<HoloJsEntry>| {
                Ok(())
            }, 
            links: [
                to!(
                    "HoloJsEntry",
                    link_type: "items",
                    validation_package: || hdk::ValidationPackageDefinition::Entry,
                    validation: |_validation_data: hdk::LinkValidationData| {
                        Ok(())
                    }
                )
            ]
        )
    ]
 
    genesis: || {
        Ok(())
    }
 
	functions: [
        create_list: {
            inputs: |collection: Collection|,
            outputs: |result: ZomeApiResult<Address>|,
            handler: handle_create_list
        }
        add_item: {
            inputs: |item: HoloJsEntry, base_addr: HashString|,
            outputs: |result: ZomeApiResult<Address>|,
            handler: handle_add_item
        }
        get_list: {
            inputs: |list_addr: HashString, link_tag: String,search: JsonString|,
            outputs: |result: ZomeApiResult<GetListResponse>|,
            handler: handle_get_list
        }
        get_linked_items: {
            inputs: |item_addr: HashString, link_tag: String,search: JsonString|,
            outputs: |result: ZomeApiResult<GetListResponse>|,
            handler: handle_get_linked_items
        }
        link_items: {
            inputs: |item_address: HashString,linkto_address: HashString, link_tag: String|,
            outputs: |result: ZomeApiResult<Address>|,
            handler: handle_link_items
        }
        unlink_items: {
            inputs: |target: HashString,base_item: HashString, link_tag: String|,
            outputs: |result: ZomeApiResult<bool>|,
            handler: handle_unlink_items
        }
        update_item: {
            inputs: |new_entry: HoloJsEntry,item_address: HashString|,
            outputs: |result: ZomeApiResult<Address>|,
            handler: handle_update_item
        }
        delete_item: {
            inputs: |item_address: HashString|,
            outputs: |result: ZomeApiResult<bool>|,
            handler: handle_delete_item
        }       
        link_bidir: {
            inputs: |item_a: HashString,item_b: HashString, link_tag_ab: String,link_tag_ba: String|,
            outputs: |result: ZomeApiResult<bool>|,
            handler: handle_link_bidir
        }
    ]
    traits: {
        hc_public [get_linked_items,create_list, add_item, get_list,link_bidir,delete_item,update_item,unlink_items,link_items]
    }
}
     

#[derive(Serialize, Deserialize, Debug, Clone, DefaultJson)]
struct Collection {
    name: String
}

#[derive(Serialize, Deserialize, Debug, Clone, DefaultJson,PartialEq)]
struct HoloJsEntry {
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
    items: Vec<HoloJsEntry>
}




/******************************************** */
/************     Handlers   ***************** */
/******************************************** */

fn handle_link_bidir(item_a: Address, item_b: Address,link_tag_ab:String,link_tag_ba:String) -> ZomeApiResult<bool> {
        hdk::utils::link_entries_bidir(&item_a, &item_b, "items","items", &link_tag_ab, &link_tag_ba)?;
        Ok(true)
}

fn handle_link_items(item_address:HashString,linkto_address:HashString,link_tag:String)->ZomeApiResult<Address>{
    let addr = hdk::api::link_entries( &item_address,&linkto_address, "items",&link_tag)?;
	Ok(addr)
}

fn handle_delete_item(item_address: Address) -> ZomeApiResult<bool> {
    hdk::remove_entry(&item_address)?;
    Ok(true)
}

fn handle_unlink_items(target:Address,base_item:Address,link_tag:String)-> ZomeApiResult<bool>{
  hdk::remove_link(&base_item,&target,"items",&link_tag)?;
  Ok(true)
}

fn  handle_update_item(new_entry: HoloJsEntry, address: Address) -> ZomeApiResult<Address> {
      let new_entry: HoloJsEntry = new_entry.into();
     hdk::update_entry(
        Entry::App("HoloJsEntry".into(),new_entry.into()),
        &address.clone()
    )
}
fn handle_create_list(collection: Collection) -> ZomeApiResult<Address> {
    // define the entry
    let list_entry = Entry::App(
        "collection".into(),
        collection.into()
    );

    // commit the entry and return the address
	hdk::commit_entry(&list_entry)
}


fn handle_add_item(item: HoloJsEntry, base_addr: HashString) -> ZomeApiResult<Address> {
    let clone_item:HoloJsEntry = item.clone();
    // define the entry
    let item = Entry::App(
        "HoloJsEntry".into(),
        item.into()
    );
    
	let item_addr = hdk::commit_entry(&item)?; // commit the collection item
	hdk::link_entries(&base_addr, &item_addr, "items",&clone_item.entityType)?; // if successful, link to collection address
	Ok(item_addr)
}


fn handle_get_list(list_addr: HashString,link_tag: String,search:JsonString) -> ZomeApiResult<GetListResponse> {
    // load the collection entry. Early return error if it cannot load or is wrong type
    let collection = hdk::utils::get_as_type::<Collection>(list_addr.clone())?;
   
    // try and load the collection items, filter out errors and collect in a vector
    let holojs_items = hdk::get_links(&list_addr, Some("items".into()),Some(link_tag.into()))?.addresses()
        .iter()
        .map(|item_address| {            
            hdk::utils::get_as_type::<HoloJsEntry>(item_address.to_owned())
        })
        .filter_map(Result::ok)    
        .collect::<Vec<HoloJsEntry>>();

    // filter the results
        let curated_list: Vec<HoloJsEntry> = 
        holojs_items.into_iter()
        .filter(|item| search_something(search.clone(), item))
        .collect(); 
    //  then return the collection items
    Ok(GetListResponse{
        name: "collection.name".to_string(),
        items: curated_list
    })
}

fn handle_get_linked_items(item_addr: HashString,link_tag: String,search:JsonString) -> ZomeApiResult<GetListResponse> {
    // load the collection entry. Early return error if it cannot load or is wrong type
    let collection = hdk::utils::get_as_type::<HoloJsEntry>(item_addr.clone())?;   
    // try and load the collection items, filter out errors and collect in a vector
    let holojs_items = hdk::get_links(&item_addr, Some("items".into()),Some(link_tag.into()))?.addresses()
        .iter()
        .map(|item_address| {            
            hdk::utils::get_as_type::<HoloJsEntry>(item_address.to_owned())
        })
        .filter_map(Result::ok)    
        .collect::<Vec<HoloJsEntry>>();

    // filter the results
        let curated_list: Vec<HoloJsEntry> = 
        holojs_items.into_iter()
        .filter(|item| search_something(search.clone(), item))
        .collect(); 
    //  then return the collection items
    Ok(GetListResponse{
        name: "collection.name".to_string(),
        items: curated_list
    })

}

/******************************************** */
/************     Fn         ***************** */
/******************************************** */




fn evaluateIsOperator(operator:String,searchval:Value,e:Value)->bool{
    
    if &operator=="is" {
    if searchval.is_u64() { hdk::debug("is U64----------------");e.as_u64()==searchval.as_u64() }
    else if searchval.is_i64() { hdk::debug("is I64----------------");e.as_i64()==searchval.as_i64() }
    else if searchval.is_f64() { hdk::debug("is F64----------------");e.as_f64()==searchval.as_f64() }
    else if searchval.is_string() { hdk::debug("is string----------------");e.as_str()==searchval.as_str() }
    else {false}
    }   else if &operator=="is_not" {
    if searchval.is_u64() { hdk::debug("is U64----------------");e.as_u64()!=searchval.as_u64() }
    else if searchval.is_i64() { hdk::debug("is I64----------------");e.as_i64()!=searchval.as_i64() }
    else if searchval.is_f64() { hdk::debug("is F64----------------");e.as_f64()!=searchval.as_f64() }
    else if searchval.is_string() { hdk::debug("is string----------------");e.as_str()!=searchval.as_str() }
    else {false}
    } else if &operator=="is_less_than"{
    if searchval.is_u64() { hdk::debug("is U64----------------");e.as_u64()<searchval.as_u64() }
    else if searchval.is_i64() { hdk::debug("is I64----------------");e.as_i64()<searchval.as_i64() }
    else if searchval.is_f64() { hdk::debug("is F64----------------");e.as_f64()<searchval.as_f64() }
    else {false}
    } else if &operator=="is_more_than"{
    if searchval.is_u64() { hdk::debug("is U64----------------");e.as_u64()>searchval.as_u64() }
    else if searchval.is_i64() { hdk::debug("is I64----------------");e.as_i64()>searchval.as_i64() }
    else if searchval.is_f64() { hdk::debug("is F64----------------");e.as_f64()>searchval.as_f64() }
    else {false}
    }else if &operator=="more_or_equal_than"{
    if searchval.is_u64() { hdk::debug("is U64----------------");e.as_u64()>=searchval.as_u64() }
    else if searchval.is_i64() { hdk::debug("is I64----------------");e.as_i64()>=searchval.as_i64() }
    else if searchval.is_f64() { hdk::debug("is F64----------------");e.as_f64()>=searchval.as_f64() }
    else {false}
    } else if &operator=="less_or_equal_than"{
    if searchval.is_u64() { hdk::debug("is U64----------------");e.as_u64()<=searchval.as_u64() }
    else if searchval.is_i64() { hdk::debug("is I64----------------");e.as_i64()<=searchval.as_i64() }
    else if searchval.is_f64() { hdk::debug("is F64----------------");e.as_f64()<=searchval.as_f64() }
    else {false}
    } else {false}

      /*  searchval.is_f64()=>Ok(e[&key].as_f64()==value2.as_f64()), 
        searchval.is_i64()=>Ok(e[&key].as_i64()==value2.as_i64()), 
        _ => ,Ok(e.as_str()==v.as_str())*/
    
}

//TODO : arrays de conditions
// TODO : search mode AND / OR 
// TODO  operateur regexp

fn search_something(_search:JsonString,_item:&HoloJsEntry)->bool {    
    let s:Value= serde_json::from_str(&_search.to_string()).unwrap();
    let e:Value= serde_json::from_str(&_item.item.to_string()).unwrap(); 
    let mut res: bool = true;
    let search_obj = s.as_object().unwrap();
    //let foo = obj.get("item").unwrap();
    for (key, search_object_json) in search_obj.iter() {     
        let search_object = search_object_json.as_object().unwrap();
        for (key2, searchvalue) in search_object.iter() {
        // not an array of condition, just one
        let estimation: Result<bool,String> = match key2.as_str() {        
        "contains" => Ok(e[&key].as_str().unwrap().contains(searchvalue.as_str().unwrap())),
        "does_not_contain" => Ok(!e[&key].as_str().unwrap().contains(searchvalue.as_str().unwrap())),       
        "is" | "is_more_than" | "is_less_than" | "is_not" | "more_or_equal_than" | "less_or_equal_than" => Ok(evaluateIsOperator(key2.to_string(),searchvalue.clone(), e[&key].clone())),  
        _ => Err("error".to_string())
            };
            if estimation.is_ok() {
                if estimation.clone().ok().unwrap()==false {  // this is an AND query : if anything is false, it's a no
                    hdk::debug(key.to_string());
                    res=estimation.ok().unwrap(); 
                }
            }
        }
    }
    res
}