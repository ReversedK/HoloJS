const HoloJs = require("./orm.js");


class Post extends HoloJs {
    constructor(instance_name) { 
       super(instance_name);  
       this.collection_name = "posts";
   }
   async setup() {
       this.collection_addr = await this.create_collection(this.collection_name);       
   }
 }

 class Tag extends HoloJs {
   constructor(instance_name) { 
      super(instance_name);  
      this.collection_name = "tags";
  }
  
  async setup() {
      this.collection_addr = await this.create_collection(this.collection_name);       
  }
}

 async function main() {
   const post =  new Post("test-instance")
   await post.setup();  
       
   const post_addr = await post.add({"title":"yoyo"+new Date().getTime()});    
   console.log('post:',post_addr)

   const tag = new Tag("test-instance");
   await tag.setup();
   const tag_addr = await tag.add({"name":"beautiful "+new Date().getTime()});    
   console.log('tag:',tag_addr);

   // link  tag to  post
   let r = await tag.link_bidirectional(post_addr,tag_addr, "tags","tags")
   console.log("tagging:",r);
  /* let re = await post.find({"title":{"contains":"yo"}})
   console.log("find:",re)*/

   await post.unlink(post_addr,tag_addr,"tags");
   let t = await post.unlink(tag_addr,post_addr,"tags");
   console.log(t);

   let post_tags = await post.findLinkedItems(post_addr,"tags",{});
   console.log("tags for this post",post_tags);
  
   let tag_post = await tag.findLinkedItems(tag_addr,"tags",{});
   console.log("post for this tag",tag_post);
/*
   let p=await post.update({"title":"yoyox the great was : "+new Date().getTime()},post_addr);
   console.log('update:',p)  
   let f = await post.delete(post_addr)  
   console.log('delete:',f) 
   let re = await post.find({"title":{"contains":"yo"}})
   console.log("find:",re)*/
 }
 main();