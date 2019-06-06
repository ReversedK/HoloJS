const { Config, Scenario } = require('@holochain/holochain-nodejs')
Scenario.setTape(require('tape'))
const dnaPath = 'dist/orm.dna.json'
const dna = Config.dna(dnaPath, 'happs')
const agentAlice = Config.agent('alice')
const instanceAlice = Config.instance(agentAlice, dna)


const scenario = new Scenario([instanceAlice],{ debugLog:false })


/*
scenario.runTape('Can create a list', async (t, { alice }) => {
  const createResult = await alice.callSync('lists', 'create_list', { list: { name: 'test list' } })
  console.log(createResult)
  t.notEqual(createResult.Ok, undefined)
})
*/
/*scenario.runTape('Can add some items', async (t, { alice }) => {
  const createResult = await alice.callSync('lists', 'create_list', { list: { name: 'test list' } })
  const listAddr = createResult.Ok

  const result1 = await alice.callSync('lists', 'add_item', { list_item: { entityType: 'post', item: {id:1,name:"post 1"} }, list_addr: listAddr })
  const result2 = await alice.callSync('lists', 'add_item', { list_item: { entityType: 'post',item: {id:1,name:"post 3"}}, list_addr: listAddr })

  console.log(result1)
  console.log(result2)

  t.notEqual(result1.Ok, undefined)
  t.notEqual(result2.Ok, undefined)
})*/

scenario.runTape('Insert a post with tags, modify it and retrieve lists : by tag and whole list of posts', async (t, { alice }) => {
  const createResult = await alice.callSync('lists', 'create_list', { list: { name: 'test list' } })
  t.notEqual(createResult.Ok, undefined)
  const listAddr = createResult.Ok
  let item1 = {entityType: "article",id:1,name:"Holo ORM"};
  let item1mod = {entityType: "article",id:1,name:"Holo ORM modified!"};
  let tag1_i = {entityType: "tag",id:1,name:"my tag"};
  let tag2_i = {entityType: "tag",id:2,name:"my second tag"}; 

  let item2 = {entityType: "article",id:2,name:"Holo Philo"};
 
  let post1 = {  entityType: "article", item: JSON.stringify(item1) }
  let post1mod = {  entityType: "article", item: JSON.stringify(item1mod) }
  let tag1 = {  entityType: "tag", item: JSON.stringify(tag1_i) }
  let tag2 = {  entityType: "tag", item: JSON.stringify(tag2_i) }     
  let post2 = {  entityType: "article", item: JSON.stringify(item2) }
  

  let addr_post1 = await alice.callSync('lists', 'add_item', { list_item: post1, list_addr: listAddr })
  const addr_post2 = await alice.callSync('lists', 'add_item', { list_item: post2, list_addr: listAddr })
  const addr_tag1 = await alice.callSync('lists', 'add_item', { list_item: tag1, list_addr: listAddr })
  const addr_tag2 = await alice.callSync('lists', 'add_item', { list_item: tag2, list_addr: listAddr })
  t.notEqual(addr_post1.Ok, undefined)
  // post 1 linké avec tag 1 et 2
  const result1 = await alice.callSync('lists', 'link_bidir', { item_a: addr_post1.Ok, item_b: addr_tag1.Ok,link_tag_ab:"tag",link_tag_ba:"tag" })
  const result2 = await alice.callSync('lists', 'link_bidir', { item_a: addr_post1.Ok, item_b: addr_tag2.Ok,link_tag_ab:"tag",link_tag_ba:"tag" })
  
  // post 2 linké avec tag 1
  const result3 = await alice.callSync('lists', 'link_bidir', { item_a: addr_post2.Ok, item_b: addr_tag1.Ok,link_tag_ab:"tag",link_tag_ba:"tag" })
  t.equal(result1.Ok, true, 'Bidir link sould return true')
    
  // update post 1
  const update = await alice.callSync('lists', 'update_item', { new_entry: post1mod, item_address: addr_post1.Ok })
  t.notEqual(update.Ok, undefined)
  
  // searchpost for tag 1
  const postsForTag1 = await alice.callSync('lists', 'get_linked_items', { item_addr: addr_tag1.Ok, link_tag : "tag" ,search: "{}" });
  //console.log("postsForTag1:",postsForTag1.Ok.items)
  t.equal(postsForTag1.Ok.items.length, 2, 'there should be 2 items with tag1')
  // searchpost for tag 2
  const postsForTag2 = await alice.callSync('lists', 'get_linked_items', { item_addr: addr_tag2.Ok, link_tag : "tag" ,search: "{}" });
 // console.log("postsForTag2:",postsForTag2.Ok.items)
  t.equal(postsForTag2.Ok.items.length, 1, 'there should be 1 item with tag2')
// get all the posts 
  let allPosts = await alice.callSync('lists', 'get_list', { list_addr: listAddr, link_tag : "article" ,search: "{}"})
  if(allPosts.hasOwnProperty("Ok"))console.log("allPosts",allPosts.Ok.items)
  t.equal(allPosts.Ok.items.length, 2, 'there should be 2 items in the post list')

  // search post with criterias 
  let search_criterias = JSON.stringify({name:{"does_not_contain":"ORM"},id:{"more_or_equal_than":1}});
  const sPosts = await alice.callSync('lists', 'get_list', { list_addr: listAddr, link_tag : "article" ,search: search_criterias})
  console.log("allPosts",sPosts)
  t.equal(sPosts.Ok.items.length, 1, 'there should be 1 item in the post list after search')
})
  

 // t.equal(getResult.Ok.items.length, 1, 'there should be 1 item in the post list')
//})
