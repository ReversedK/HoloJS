
const xhr = require('superagent');

class HoloJs {
    constructor(collection,config) {
      this.collection_name = collection.name;
      this.collection_addr = collection.addr;
      this.instance_name = config.instance_name;
      this.conductor_endpoint = config.conductor_endpoint;
    }
    async setup() {
      console.log('setting up collection '+this.collection_name);
      //this.collection_addr = await this.create_collection(this.collection_name);
      console.log('done setting up collection '+this.collection_name+': #',this.collection_addr);
        return true;
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
    //  console.log("*******************",this)
        let response;
        payload = this.preparePayload(zome,fn,payload);

        if(typeof callback == 'function')
            xhr.post(this.conductor_endpoint).set('Content-Type', 'application/json').set('accept', 'json')
            .send(payload)
            .end((err, res) => {
             callback(err,res.body);
        }); else {
            response = await xhr.post(this.conductor_endpoint).set('Content-Type', 'application/json').set('accept', 'json').send(payload);

            try {
              console.log(fn,payload,response.body.result);
            return JSON.parse(response.body.result).Ok
            } catch(e) { console.log(e); console.log(response.body);return e;}
        }
    }

    /************** ***************/
    /* CRUD methods & some more...*/
    /*************************** */
    async add(item2add) {
      console.log('adding item to collection '+this.collection_name+' #',this.collection_addr);
        item2add.entity_type = this.collection_name;
        item2add = {  entity_type: item2add.entity_type, item: JSON.stringify(item2add) }
        let payload = { item: item2add, base_addr: this.collection_addr };
        return await this.callHoloInstance('collections', 'add_item',payload);
    };

    async update(new_item,item_addr) {
        new_item.entity_type = this.collection_name;
        new_item = {  entity_type: this.collection_name, item: JSON.stringify(new_item) }
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
            collection_addr: this.collection_addr,
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
