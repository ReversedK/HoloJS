const { Config, Scenario } = require('@holochain/holochain-nodejs')
Scenario.setTape(require('tape'))
const dnaPath = 'dist/holochain-rust-todo.dna.json'
const dna = Config.dna(dnaPath, 'happs')
const agentAlice = Config.agent('alice')
const instanceAlice = Config.instance(agentAlice, dna)
const scenario = new Scenario([instanceAlice])
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

scenario.runTape('Can get a list with items', async (t, { alice }) => {
  const createResult = await alice.callSync('lists', 'create_list', { list: { name: 'test list' } })
  const listAddr = createResult.Ok
  const result1 = await alice.callSync('lists', 'add_item', { list_item: { entityType: "post", item: JSON.stringify({id:1,name:"post 1"}) }, list_addr: listAddr })
  const result2 = await alice.callSync('lists', 'add_item', { list_item: { entityType: "article",item: JSON.stringify({id:1,name:"post 3"})}, list_addr: listAddr })


  const getResult = await alice.callSync('lists', 'get_list', { list_addr: listAddr, link_tag : "article"
  ,search: JSON.stringify({id:1})
 })
  console.log(getResult.Ok.items[0])

  t.equal(getResult.Ok.items.length, 1, 'there should be 1 item in the post list')
})
