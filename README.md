  
  

# HoloJS 
tested with Holochain version 0.0.20-alpha3

WARNING : for some reason the new tests using Diorama are failing. When using the Holochain conductor everything works as it's supposed to.  
  

## **What is it ?**

  

HoloJS is NOT a HDK for javascript it is merely a wrapper around some of the Rust HDK functions, particularly the CRUD functions. Holochain has many features, HoloJS only focuses on the data layer aspect.

  

HoloJs can be considered as a data connector allowing JS app to use Holochain as a data persistence layer.

  

HoloJS is composed of two pieces of software :

- A NodeJS module

- A Rust hApp

  

The hApp acts as an API for some of the functions of the Rust HDK. The NodeJs library streamlines the consumption of the API and makes it easy to integrate Holochain to any JS project.

  

## **Terminology**

  

HoloJS ‘s terminology is inspired by NoSQL with Entries and Collections as basic structures.

  

**Collections**

Collections can be pictured as arrays of entries, like a “table” could be seen as an array of records in SQL.

Technically a collection is an entry that will be used as an anchor to link to other entries.

  

**Entries**

HoloJSentries acts as storage containers for a JSON payload describing the object’s state. The NodeJs module makes it easy to create, read, update, delete or, link entries.

  

**Logic**

Logic happens outside the Rust app.providing more flexibility and ease to the devs

  

## **What can I do with HoloJS?**

  

You can create, read, update, delete and link entries. A limited find function is also included.

  

## **Installation**

  

Clone this repository

open a terminal, cd to the directory where you cloned this project and type :

hc run -i http

You can then use the node module, cd to the "js" directory. run :

node tests.js

to run the tests

  

## **Documentation**

  

**HoloJs**(collection_name,config) : instantiates a HoloJs object

    const config = {
    instance_name:"test-instance",
    conductor_endpoint:'http://localhost:8888'
    }
    let post = new HoloJs(“posts”,config);

**add**(item:JSON) : inserts an entry into the collection
*accepts : **item** :any valid JSON object
returns : String representing the entry address*

    const  post_addr  =  await  post.add({"title":"my first post"});

**update** (item:JSON,item_addr : string) : updates an entry
 *accepts : **item** :  any valid JSON object ,  **item_addr** : address of the entry to update
returns : String representing the entry address*

    let p=await post.update({"title":"yoyox the great was : "+new Date().getTime()},post_addr);    

**delete** (item_addr : string) : deletes an entry
 *accepts :  **item_addr** : address of the entry to delete
returns : boolean* 

**link**(item_a,item_b,linktag) : creates a link from item_a to item_b with the specified linktag.
 *accepts :  
 **item_a** : address of an entry, string
**item_b** : address of an entry, string
**linktag** : linktag that will caracterize the association , string*
  

**link_bidirectional**(item_a,item_b,linktag_a_b,linktag_b_a) : creates a link from item_a to item_b and reciprocally from item_b to item_a. Each association can have its own linktag.
 *accepts :  
 **item_a** : address of an entry, string
 **item_b** : address of an entry, string
  **linktag_a_b** : linktag that will caracterize the association , string
 **linktag_b_a** :  linktag that will caracterize the association , string
returns : boolean* 

    await  tag.link_bidirectional(post_addr,tag_addr, "tags","tags")
      
**unlink**(item_a,item_b,linktag) : removes a link from item_a to item_b for the specified linktag. 
 *accepts :  
 **item_a** : address of an entry, string
**item_b** : address of an entry, string
**linktag** : linktag that will caracterize the association , string*

**find**(search_params) : finds the entries corresponding to the search parameters in the current collection 
 *accepts :  
 **search_params** : JSON object*
 
 The search params object must be structured as follows :
{ **query mode (and/or)** : { **field_name**  : { **operator** : **array of values** }}
for example : `{"and":{"title":{"contains":["first","step"]},id:{"less_than":[10]}}`
would return the entries with a  title  containing the strings "first" and "step" with an id inferior to 10. Note that even if the condition has a unique value you have to encapsulate it in an array


| Operation | Operator |
|--|--|--|
|substring search  | Contains, does_not_contain |
|comparisons  | is_less_than,is_more_than,is_more_or_equal_than,is_less_or_equal_than | 
|exact comparison  | is, is_not | 


returns : array of entries* 

**findLinkedItems**(base_addr,linktag,search_params) : finds entries linked to the specified base entry according to a linktag and search parameters
 *accepts :  
 **search_params** : JSON object
 returns : array of entries* 



    


  

  

## Code example

  

    let post = new HoloJs(“posts”,config);    
    let tag = new HoloJs(“tag”,config);    
    await posts.setup() // creates the posts collection anchor    
    await tags.setup() // creates the tags collection anchor    
    
    // the entry is inserted and automatically linked to the “posts” collection    
    const my1stPost = await post.add( { title : “My first post”, body : “Holochain”, timestamp : 155339196544 });    
    const my2ndPost = await post.add( { title : “My second post”, body : “Holochain”, timestamp : 155339196545 });
    
    // let’s tag the first post as “beautiful”    
    // first create the tag    
    Const beautiful = await tag.add({name:”beautiful”});
        
    // then link it bidir so that we can both find the tags for this post and the posts for this tag    
    let r = await tag.link_bidirectional(my1stPost,beautiful, "tags","tags")
    
    // Find the tags for a post    
    let post_tags = await post.findLinkedItems(my1stPost,"tags",{});
    
    // find the posts for a tag    
    let tag_posts = await tag.findLinkedItems(beautiful,"tags",{});
    
    // find the posts for a tag with parameters    
    let tag_post = await tag.findLinkedItems(beautiful,"tags",{    
    “and”:{“title”:{“contains”:[”first”]}, “timestamp”:“is_less_than”:[155339196545]}} });
  

## The future ? 

Implement type validation inside the nodeJS lib (typescript?)

React / angular ?

add possibility to create private data (duplicate existing with different Sharing property?)

  

Your comments and pull requests are very welcome :)
