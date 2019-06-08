
const superagent = require('superagent');



class HoloJs {
    constructor(instance_name) {      
      this.instance_name = instance_name;    
    }  

    preparePayload(zome,fn,payload_obj) {
       return {
            "jsonrpc": "2.0",
            "id": "0",
            "method": "call",
            "params": {
                "instance_id": this.instance_name,
                "zome": zome,
                "function": fn,
                "args": payload_obj
            }
        }
    }

    async callHoloInstance(zome,fn,payload,callback=false) {
        let response;
        payload = this.preparePayload(zome,fn,payload);
        let xhr = superagent;
        
        if(typeof callback == 'function') 
            xhr.post('http://localhost:8888').set('Content-Type', 'application/json').set('accept', 'json')
            .send(payload) 
            .end((err, res) => {
             callback(err,res.body);
        }); else {
            response = await xhr.post('http://localhost:8888').set('Content-Type', 'application/json').set('accept', 'json').send(payload);
            try {
            return JSON.parse(response.body.result).Ok
            } catch(e) { console.log(e); return e;}
        }
    }

    /************** */
    /* ORM methods */
    /************** */
    async add(item2add) { 
        item2add.entityType = this.collection_name;       
        item2add = {  entityType: item2add.entityType, item: JSON.stringify(item2add) }      
        let payload = { item: item2add, base_addr: this.collection_addr };
        return await this.callHoloInstance('collections', 'add_item',payload);  
    };
    async update(new_item,item_addr) { 
        new_item.entityType = this.collection_name;         
        new_item = {  entityType: this.collection_name, item: JSON.stringify(new_item) }      
        let payload = { new_entry: new_item, item_address: item_addr };
        console.log(payload)
        return await this.callHoloInstance('collections', 'update_item',payload);  
    };
    async create_collection(name) {
        let payload = { "collection": { "name": name } };       
        return await this.callHoloInstance('collections', 'create_collection',payload);  
    };      
    async find(search={}) {
        let payload = { 
            list_addr: this.collection_addr,
            link_tag : this.collection_name,
            search : JSON.stringify(search)
          };
        return await this.callHoloInstance('collections', 'get_list',payload);  
    };

    async findLinkedItems(base_addr,link_tag,search={}) {
        let payload = { 
            list_addr: base_addr,
            link_tag : link_tag,
            search : JSON.stringify(search)
          };
        return await this.callHoloInstance('collections', 'get_linked_items',payload);  
    };

  }

  class Post extends HoloJs {
     constructor(instance_name) { 
        super(instance_name);  
        this.collection_name = "posts";
    }

    async setup() {
        this.collection_addr = await this.create_collection(this.collection_name);       
    }
  }

  async function main() {
  const post =  new Post("test-instance")
  await post.setup();
  
    
  const post_addr = await post.add({"title":"yoyo"});
console.log('post:',post_addr)
 let p=await post.update({"title":"yoyo the great"},post_addr);
 console.log('update:',p)
 
let re = await post.find({"title":{"contains":"yo"}})
  console.log("find:",re)

  }

  main();