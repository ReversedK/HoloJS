module.exports = scenario => {
scenario('Insert a post with tags, modify it and retrieve collections : by tag and whole collection of posts', async (s, t, {alice}) => {

  const createResult = await alice.call('collections', 'create_collection', {
    collection: {
      name: 'test collection'
    }
  })

  t.notEqual(createResult.Ok, undefined,"can create a collection")

  const listAddr = createResult.Ok
  let item1 = {
    entityType: "article",
    id: 1,
    name: "Holo ORM"
  };
  let item1mod = {
    entityType: "article",
    id: 1,
    name: "Holo ORM modified!"
  };
  let tag1_i = {
    entityType: "tag",
    id: 1,
    name: "my tag"
  };
  let tag2_i = {
    entityType: "tag",
    id: 2,
    name: "my second tag"
  };

  let item2 = {
    entityType: "article",
    id: 2,
    name: "Holo Philo"
  };

  let post1 = {
    entityType: "article",
    item: JSON.stringify(item1)
  }
  let post1mod = {
    entityType: "article",
    item: JSON.stringify(item1mod)
  }
  let tag1 = {
    entityType: "tag",
    item: JSON.stringify(tag1_i)
  }
  let tag2 = {
    entityType: "tag",
    item: JSON.stringify(tag2_i)
  }
  let post2 = {
    entityType: "article",
    item: JSON.stringify(item2)
  }


  let addr_post1 = await alice.call('collections', 'add_item', {
    item: post1,
    base_addr: listAddr
  })
  const addr_post2 = await alice.call('collections', 'add_item', {
    item: post2,
    base_addr: listAddr
  })
  const addr_tag1 = await alice.call('collections', 'add_item', {
    item: tag1,
    base_addr: listAddr
  })
  const addr_tag2 = await alice.call('collections', 'add_item', {
    item: tag2,
    base_addr: listAddr
  })
  console.log(addr_post1);
  t.notEqual(addr_post1.Ok, undefined,"can create an entry");

  // post 1 linké avec tag 1 et 2
  const result1 = await alice.call('collections', 'link_bidir', {
    item_a: addr_post1.Ok,
    item_b: addr_tag1.Ok,
    link_tag_ab: "tag",
    link_tag_ba: "tag"
  })
  const result2 = await alice.call('collections', 'link_bidir', {
    item_a: addr_post1.Ok,
    item_b: addr_tag2.Ok,
    link_tag_ab: "tag",
    link_tag_ba: "tag"
  })

  // post 2 linké avec tag 1
  const result3 = await alice.call('collections', 'link_bidir', {
    item_a: addr_post2.Ok,
    item_b: addr_tag1.Ok,
    link_tag_ab: "tag",
    link_tag_ba: "tag"
  })
  t.equal(result1.Ok, true, 'Bidir link sould return true')

  // update post 1  
  const update = await alice.call('collections', 'update_item', {
    new_entry: post1mod,
    item_address: addr_post1.Ok
  })
  t.notEqual(update.Ok, undefined)

  // searchpost for tag 1
  const postsForTag1 = await alice.call('collections', 'get_linked_items', {
    item_addr: addr_tag1.Ok,
    link_tag: "tag",
    search: "{}"
  });
  //console.log("postsForTag1:",postsForTag1.Ok.items)
  t.equal(postsForTag1.Ok.items.length, 2, 'there should be 2 items with tag1')

  // unlink post1 and tag 1

  const unlink = await alice.call('collections', 'unlink_items', {
    target: addr_tag1.Ok,
    link_tag: "tag",
    base_item: addr_post1.Ok
  });
  t.notEqual(unlink.Ok, undefined)
  const unlink2 = await alice.call('collections', 'unlink_items', {
    base_item: addr_tag1.Ok,
    link_tag: "tag",
    target: addr_post1.Ok
  });
  t.notEqual(unlink2.Ok, undefined)


  const npostsForTag1 = await alice.call('collections', 'get_linked_items', {
    item_addr: addr_tag1.Ok,
    link_tag: "tag",
    search: "{}"
  });
  //console.log("postsForTag1:",postsForTag1.Ok.items)
  t.equal(npostsForTag1.Ok.items.length, 1, 'there should be 1 items with tag1')




  // searchpost for tag 2
  const postsForTag2 = await alice.call('collections', 'get_linked_items', {
    item_addr: addr_tag2.Ok,
    link_tag: "tag",
    search: "{}"
  });
  console.log("postsForTag2:", postsForTag2.Ok)
  t.equal(postsForTag2.Ok.items.length, 1, 'there should be 1 item with tag2')
  // get all the posts 
  let allPosts = await alice.call('collections', 'get_list', {
    collection_addr: listAddr,
    link_tag: "article",
    search: "{}"
  })
  console.log("allPosts", allPosts.Ok.items)
  t.equal(allPosts.Ok.items.length, 2, 'there should be 2 items in the post collection')

  // search post with criterias 
  let search_criterias = JSON.stringify({
    "or": {
      "name": {
        "does_not_contain": "Holo"
      },
      "id": {
        "is_more_or_equal_than": 2
      }
    }
  });
  const sPosts = await alice.call('collections', 'get_list', {
    collection_addr: listAddr,
    link_tag: "article",
    search: search_criterias
  })
  console.log("allPosts criterias", sPosts.Ok.items)
  t.equal(sPosts.Ok.items.length, 1, 'there should be 1 item in the post collection after search')
});
}