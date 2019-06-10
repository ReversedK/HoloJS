const HoloJs = require("./holojs.js");

const config = {
  instance_name:"test-instance",
  conductor_endpoint:'http://localhost:8888'  
}

 async function main() {   
   let post = new HoloJs("posts",config)
   await post.setup();  
       
   const post_addr = await post.add({"title":"yoyo"+new Date().getTime()});    
   console.log('post:',post_addr)

   const tag = new HoloJs("tag",config);
   await tag.setup();
   const tag_addr = await tag.add({"name":"beautiful "+new Date().getTime()});    
   console.log('tag:',tag_addr);

   // link  tag to  post
   let r = await tag.link_bidirectional(post_addr,tag_addr, "tags","tags")
   console.log("tagging:",r);
  /* let re = await post.find({"title":{"contains":"yo"}})
   console.log("find:",re)*/

   let t0 = await post.unlink(post_addr,tag_addr,"tags");
   console.log(t0);
   let t = await post.unlink(tag_addr,post_addr,"tags");
   console.log(t);
setTimeout(async()=>{
   let post_tags = await post.findLinkedItems(post_addr,"tags",{});
   console.log("tags for this post",post_tags);
  
   let tag_post = await tag.findLinkedItems(tag_addr,"tags",{});
   console.log("post for this tag",tag_post);


   let p=await post.update({"title":"yoyox the great was : "+new Date().getTime()},post_addr);
   console.log('update:',p)  
   let f = await post.delete(post_addr)  
   console.log('delete:',f) 
   let re = await post.find({"and":{"title":{"contains":"yo"}}})
   console.log("find:",re)
},500)
 }
 main();