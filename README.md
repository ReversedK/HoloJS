

# HoloJS

## **What is it ?**

HoloJS is NOT a HDK for javascript it is merely a wrapper around some of the Rust HDK functions, particularly the CRUD functions. Holochain has many features, HoloJS only focuses on the data layer aspect.

HoloJs can  be considered as a data connector allowing JS app to use Holochain as a data persistence layer.

HoloJS is composed of two pieces of software :
-   A NodeJS module    
-   A Rust hApp     

The hApp acts as an API for some of the functions of the Rust HDK. The NodeJs library streamlines the consumption of the API and makes it easy to integrate  Holochain to any JS project.

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
  

## **Documentation (soon)**

HoloJS (instance_name,instance_endpoint)

create_collection

add

delete

update

link

link_bidir

unlink

find

  
  

## Code example

    Let post = new HoloJs(“posts”,config);    
    Let tag = new HoloJs(“tag”,config);    
    Await posts.setup() // creates the posts collection anchor    
    Await tags.setup() // creates the tags collection anchor    
    
    # the entry is inserted and automatically linked to the “posts” collection
    Const my1stPost = await post.add( { title : “My first post”, body : “Holochain”, timestamp : 155339196544 });    
    Const my2ndPost = await post.add( { title : “My second post”, body : “Holochain”, timestamp : 155339196545 });
    
    # let’s tag the first post as “beautiful”  
	# first create the tag    
    Const beautiful = await tag.add({name:”beautiful”}); 
    # then link it bidir so that we can both find the tags for this post and the posts for this tag   
    let  r = await  tag.link_bidirectional(my1stPost,beautiful, "tags","tags")    
    
     # Find the tags for a post   
    let  post_tags = await  post.findLinkedItems(my1stPost,"tags",{});    
    
    # find the posts for a tag
    let  tag_posts = await  tag.findLinkedItems(beautiful,"tags",{});
    
    # find the posts for a tag with parameters    
    let  tag_post = await  tag.findLinkedItems(beautiful,"tags",{    
    “and”:{“title”:{“contains”:”first”}, “timestamp”:“is_less_than”:155339196545}} });

 
## The future :)  

Implement type validation inside the nodeJS lib (typescript?)
React / angular ?
add possibility to create private data (duplicate existing with different Sharing property?)

Your comments and pull requests are very welcome :)

