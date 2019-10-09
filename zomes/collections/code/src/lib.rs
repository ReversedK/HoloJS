#[macro_use]
extern crate hdk;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[macro_use]
extern crate holochain_json_derive;



use hdk::{
    error::ZomeApiResult,
    holochain_core_types::{
        link::LinkMatch,
        dna::entry_types::Sharing,
        entry::Entry,

    }
};

use hdk::holochain_persistence_api::{
    cas::content::Address,
};


use serde_json::value::Value;

use hdk::holochain_json_api::{
    error::JsonError,
    json::JsonString,
};

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
                    link_type: "related_items",
                    validation_package: || hdk::ValidationPackageDefinition::Entry,
                    validation: |_validation_data: hdk::LinkValidationData| {
                        Ok(())
                    }
                )
            ]
        )
    ]

    init: || { Ok(()) }

    validate_agent: |validation_data : EntryValidationData::<AgentId>| {
        Ok(())
    }

	functions: [
        create_collection: {
            inputs: |collection: Collection|,
            outputs: |result: ZomeApiResult<Address>|,
            handler: handle_create_collection
        }
        add_item: {
            inputs: |item: HoloJsEntry, base_addr: Address|,
            outputs: |result: ZomeApiResult<Address>|,
            handler: handle_add_item
        }
        get_list: {
            inputs: |collection_addr: Address, link_tag: String,search: JsonString|,
            outputs: |result: ZomeApiResult<GetCollectionResponse>|,
            handler: handle_get_list
        }
        get_linked_items: {
            inputs: |item_addr: Address, link_tag: String,search: JsonString|,
            outputs: |result: ZomeApiResult<GetCollectionResponse>|,
            handler: handle_get_linked_items
        }
        link_items: {
            inputs: |item_address: Address,linkto_address: Address, link_tag: String|,
            outputs: |result: ZomeApiResult<Address>|,
            handler: handle_link_items
        }
        unlink_items: {
            inputs: |target: Address,base_item: Address, link_tag: String|,
            outputs: |result: ZomeApiResult<bool>|,
            handler: handle_unlink_items
        }
        update_item: {
            inputs: |new_entry: HoloJsEntry,item_address: Address|,
            outputs: |result: ZomeApiResult<Address>|,
            handler: handle_update_item
        }
        delete_item: {
            inputs: |item_address: Address|,
            outputs: |result: ZomeApiResult<bool>|,
            handler: handle_delete_item
        }
        link_bidir: {
            inputs: |item_a: Address,item_b: Address, link_tag_ab: String,link_tag_ba: String|,
            outputs: |result: ZomeApiResult<bool>|,
            handler: handle_link_bidir
        }
    ]
    traits: {
        hc_public [get_linked_items,create_collection, add_item, get_list,link_bidir,delete_item,update_item,unlink_items,link_items]
    }
}


/******************************************* */
/******** STRUCTURE DECLARATIONS     ******* */
/******************************************* */



#[derive(Serialize, Deserialize, Debug, Clone, DefaultJson)]
struct Collection {
    name: String
}

#[derive(Serialize, Deserialize, Debug, Clone, DefaultJson,PartialEq)]
struct GetHoloJsEntry {
    entity_type: String,
    item: JsonString,
    addr : Address
}

#[derive(Serialize, Deserialize, Debug, Clone, DefaultJson,PartialEq)]
struct HoloJsEntry {
    entity_type: String,
    item: JsonString
}

#[derive(Serialize, Deserialize, Debug, Clone, DefaultJson)]
struct SearchObject {
    entity_type: String,
    item: JsonString
}

#[derive(Serialize, Deserialize, Debug, DefaultJson)]
struct GetCollectionResponse {
    name: String,
    items: Vec<GetHoloJsEntry>
}

/******************************************** */
/************     Handlers   ***************** */
/******************************************** */

fn handle_link_bidir(item_a: Address, item_b: Address,link_tag_ab:String,link_tag_ba:String) -> ZomeApiResult<bool> {
        hdk::utils::link_entries_bidir(&item_a, &item_b, "related_items","related_items", &link_tag_ab, &link_tag_ba)?;
        Ok(true)
}

fn handle_link_items(item_address:Address,linkto_address:Address,link_tag:String)->ZomeApiResult<Address>{
    let addr = hdk::api::link_entries( &item_address,&linkto_address, "related_items",&link_tag)?;
	Ok(addr)
}

fn handle_delete_item(item_address: Address) -> ZomeApiResult<bool> {
    hdk::remove_entry(&item_address)?;
    Ok(true)
}

fn handle_unlink_items(target:Address,base_item:Address,link_tag:String)-> ZomeApiResult<bool>{
  hdk::remove_link(&base_item,&target,"related_items",&link_tag)?;
  Ok(true)
}

fn  handle_update_item(new_entry: HoloJsEntry, address: Address) -> ZomeApiResult<Address> {
      let new_entry: HoloJsEntry = new_entry.into();
     hdk::update_entry(
        Entry::App("HoloJsEntry".into(),new_entry.into()),
        &address.clone()
    )?;
    Ok(address)
}

fn handle_create_collection(collection: Collection) -> ZomeApiResult<Address> {
    // define the entry
    let list_entry = Entry::App(
        "collection".into(),
        collection.into()
    );
    // commit the entry and return the address
	hdk::commit_entry(&list_entry)
}

fn handle_add_item(item: HoloJsEntry, base_addr: Address) -> ZomeApiResult<Address> {
    let clone_item:HoloJsEntry = item.clone();
    // define the entry
    let item = Entry::App(
        "HoloJsEntry".into(),
        item.into()
    );

	let item_addr = hdk::commit_entry(&item)?; // commit the collection item
	hdk::link_entries(&base_addr, &item_addr, "items",&clone_item.entity_type)?; // if successful, link to collection address
	Ok(item_addr)
}


fn handle_get_list(collection_addr: Address,link_tag: String,search:JsonString) -> ZomeApiResult<GetCollectionResponse> {
    // load the collection entry. Early return error if it cannot load or is wrong type
     hdk::utils::get_as_type::<Collection>(collection_addr.clone())?;

    // try and load the collection items, filter out errors and collect in a vector
    let holojs_items : Vec<GetHoloJsEntry>  = hdk::get_links(&collection_addr, LinkMatch::Exactly("items"),LinkMatch::Exactly(&link_tag))?.addresses()
        .iter()
        .map(|item_address|-> ZomeApiResult<GetHoloJsEntry>  {
           let the_entry = hdk::utils::get_as_type::<HoloJsEntry>(item_address.to_owned())?;
           Ok(GetHoloJsEntry { addr : item_address.clone(), item: the_entry.item,entity_type:the_entry.entity_type})
        })
        .filter_map(Result::ok)
        .collect::<Vec<GetHoloJsEntry>>();

    // filter the results
        let curated_list: Vec<GetHoloJsEntry> =
        holojs_items.into_iter()
        .filter(|item| search_something(search.clone(), &item.clone()))
        .collect::<Vec<GetHoloJsEntry>>();
    //  then return the collection items
    Ok(GetCollectionResponse{
        name: link_tag.clone(),
        items: curated_list
    })
}

fn handle_get_linked_items(item_addr: Address,link_tag: String,search:JsonString) -> ZomeApiResult<GetCollectionResponse> {
    // load the  entry. Early return error if it cannot load or is wrong type
    hdk::utils::get_as_type::<HoloJsEntry>(item_addr.clone())?;

    // try and load the  items linked to base_item, filter out errors and collect in a vector
    let holojs_items = hdk::get_links(&item_addr, LinkMatch::Exactly("related_items"),LinkMatch::Exactly(&link_tag))?.addresses()
        .iter()
        .map(|item_address|-> ZomeApiResult<GetHoloJsEntry>  {
           let the_entry = hdk::utils::get_as_type::<HoloJsEntry>(item_address.to_owned())?;
           Ok(GetHoloJsEntry { addr : item_address.to_owned(), item: the_entry.item,entity_type:the_entry.entity_type})
        })
        .filter_map(Result::ok)
        .collect::<Vec<GetHoloJsEntry>>();

    // filter the results
        let curated_list: Vec<GetHoloJsEntry> =
        holojs_items.into_iter()
        .filter(|item| search_something(search.clone(), item))
        .collect();
    //  then return the  items encapsulated in a GetCollectionResponse struct
    Ok(GetCollectionResponse{
        name: link_tag.clone(),
        items: curated_list
    })
}


/******************************************** */
/************     Fn         ***************** */
/******************************************** */




fn evaluate_is_operator(operator:String,searchval:Value,e:Value)->bool{

    if &operator=="is" {
    if searchval.is_u64() { e.as_u64()==searchval.as_u64() }
    else if searchval.is_i64() { e.as_i64()==searchval.as_i64() }
    else if searchval.is_f64() { e.as_f64()==searchval.as_f64() }
    else if searchval.is_string() { e.as_str()==searchval.as_str() }
    else {false}
    }   else if &operator=="is_not" {
    if searchval.is_u64() { e.as_u64()!=searchval.as_u64() }
    else if searchval.is_i64() { e.as_i64()!=searchval.as_i64() }
    else if searchval.is_f64() { e.as_f64()!=searchval.as_f64() }
    else if searchval.is_string() { e.as_str()!=searchval.as_str() }
    else {false}
    } else if &operator=="is_less_than"{
    if searchval.is_u64() { e.as_u64()<searchval.as_u64() }
    else if searchval.is_i64() { e.as_i64()<searchval.as_i64() }
    else if searchval.is_f64() { e.as_f64()<searchval.as_f64() }
    else {false}
    } else if &operator=="is_more_than"{
    if searchval.is_u64() { e.as_u64()>searchval.as_u64() }
    else if searchval.is_i64() { e.as_i64()>searchval.as_i64() }
    else if searchval.is_f64() { e.as_f64()>searchval.as_f64() }
    else {false}
    }else if &operator=="is_more_or_equal_than"{
    if searchval.is_u64() { e.as_u64()>=searchval.as_u64() }
    else if searchval.is_i64() { e.as_i64()>=searchval.as_i64() }
    else if searchval.is_f64() { e.as_f64()>=searchval.as_f64() }
    else {false}
    } else if &operator=="is_less_or_equal_than"{
    if searchval.is_u64() { e.as_u64()<=searchval.as_u64() }
    else if searchval.is_i64() { e.as_i64()<=searchval.as_i64() }
    else if searchval.is_f64() { e.as_f64()<=searchval.as_f64() }
    else {false}
    } else {false}

      /*  searchval.is_f64()=>Ok(e[&key].as_f64()==value2.as_f64()),
        searchval.is_i64()=>Ok(e[&key].as_i64()==value2.as_i64()),
        _ => ,Ok(e.as_str()==v.as_str())*/

}

//TODO : arrays de conditions
// TODO : search mode AND / OR
// TODO  operateur regexp

fn search_something(_search:JsonString,_item:&GetHoloJsEntry)->bool {
    let s:Value= serde_json::from_str(&_search.to_string()).unwrap();
    let e:Value= serde_json::from_str(&_item.item.to_string()).unwrap();
    let mut res: bool = true;
    let search_obj = s.as_object().unwrap();
    //let foo = obj.get("item").unwrap();
    for (operator, search_object_json) in search_obj.iter() {
        if operator=="or" { res = false; } // if operator is or we start at false
        let search_object = search_object_json.as_object().unwrap();
         for (field, search_object_json_1) in search_object.iter() {
            let search_object2 = search_object_json_1.as_object().unwrap();
            for (key2, search_object_json_3) in search_object2.iter() {
                let condition_array = search_object_json_3.as_array().unwrap();
                for searchvalue in condition_array.iter() {
            //  must always be an array of conditions
            let estimation: Result<bool,String> = match key2.as_str() {
            "contains" => Ok(e[&field].as_str().unwrap().contains(searchvalue.as_str().unwrap())),
            "does_not_contain" => Ok(!e[&field].as_str().unwrap().contains(searchvalue.as_str().unwrap())),
            "is" | "is_more_than" | "is_less_than" | "is_not" | "more_or_equal_than" | "less_or_equal_than" => Ok(evaluate_is_operator(key2.to_string(),searchvalue.clone(), e[&field].clone())),
            _ => Err("error".to_string())
                };
                if estimation.is_ok() {
                    if operator=="and" && estimation.clone().ok().unwrap()==false {  // this is an AND query : if anything is false, it's a no
                        res=estimation.ok().unwrap();
                    } else if operator=="or" && estimation.clone().ok().unwrap()==true {  // this is an OR query : if anything is true, it's a go
                        res=estimation.ok().unwrap();
                    }
                }
            }
            }
         }
    }
    res
}
