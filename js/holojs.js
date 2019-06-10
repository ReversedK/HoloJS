
const superagent = require('superagent');

class HoloJs {
    constructor(instance_name,conductor_endpoint='http://localhost:8888') {      
      this.instance_name = instance_name; 
      this.conductor_endpoint = conductor_endpoint;   
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
            xhr.post(this.conductor_endpoint).set('Content-Type', 'application/json').set('accept', 'json')
            .send(payload) 
            .end((err, res) => {
             callback(err,res.body);
        }); else {
            response = await xhr.post(this.conductor_endpoint).set('Content-Type', 'application/json').set('accept', 'json').send(payload);
            try {                
            return JSON.parse(response.body.result).Ok
            } catch(e) { console.log(e); console.log(response.body);return e;}
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
        return await this.callHoloInstance('collections', 'update_item',payload);  
    };

    async delete(item_addr) {       
        let payload = {item_address: item_addr };
        return await this.callHoloInstance('collections', 'delete_item',payload);  
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
            item_addr: base_addr,
            link_tag : link_tag,
            search : JSON.stringify(search)
          };
        return await this.callHoloInstance('collections', 'get_linked_items',payload);  
    };

    async link(item_address,linkto_address, link_tag){
        let payload = { 
            item_address: item_address,
            link_tag : link_tag,
            linkto_address : linkto_address
          };
        return await this.callHoloInstance('collections', 'link_items',payload);  
  }

  async unlink(base_addr,target, link_tag){
    let payload = { 
        base_item: base_addr,
        link_tag : link_tag,
        target : target
      };
    return await this.callHoloInstance('collections', 'unlink_items',payload);  
}
  async link_bidirectional(item_address,linkto_address, link_tag_ab,link_tag_ba){
    let payload = { 
        item_a: item_address,
        link_tag_ab : link_tag_ab,
        link_tag_ba : link_tag_ba,
        item_b : linkto_address
      };
    return await this.callHoloInstance('collections', 'link_bidir',payload);  
}
}

module.exports = HoloJs;