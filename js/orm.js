
class HoloJs {
    constructor(instance) {      
     // this.HoloAPI = instance;    
    }  

    async callHoloInstance(zome,fn,payload) {}




    async add(item2add,base_addr) {
        let payload = { list_item: item2add, list_addr: base_addr };
        await this.callHoloInstance('lists', 'add_item',payload);  
    };  
    async create_list(list_name) {
        let payload = { list: { name: list_name } };
        await this.callHoloInstance('lists', 'create_list',payload);  
    };    
  }

  class Post extends HoloJs {
    constructor() { 
        super();  
      }
  }

  const r = new Post(instance).create_list();
  console.log(r);