#![allow(warnings)]
use std::i32;



struct Allocator{
    max_size: i32 ,
    heap: Vec<u8>,
    total_memory_allocated : i32,
    free_lists: Vec<Vec<usize>>, 
    bin_sizes : Vec<i32> 
    

    
}

impl Allocator{
    

    fn init_heap(&mut self ){
        self.max_size = 8192;
        self.heap = Vec::new();
        self.total_memory_allocated = 0; 
        self.free_lists = Vec::new();
        for i in 0..8192{
            self.heap.push(0);
        }
        
    } 
    fn init_free_lists(&mut self){
        let mut size_bytes = 16; 

        for i in 0..128{
            self.bin_sizes.push(size_bytes);
            size_bytes+=8; 
            self.free_lists.push(Vec::new());
        }

        self.free_lists[127].push(8); 
        
    }

    fn len_header_init( &mut self , header_pointer :&i32 ){
        let mut ptr = header_pointer.clone() + 4;
        for i in 0..4 {
            self.heap[(ptr + i ) as usize] = 1;
        }

    } 

    fn plen_header_init(&mut self , header_pointer: &i32  , size:i32){
        let mut ptr = header_pointer + size;
        for i in 0..4{
            self.heap[(ptr + i ) as usize] = 10;
        }

    }
    fn status_header_init(&mut self , header_pointer: &i32){
        let mut ptr = header_pointer; 
        for i in 0..4{
            self.heap[(ptr + i ) as usize] = 0;
        }
    }
    fn find_bin( &self , size:i32) -> i32{
        for i in &self.bin_sizes{
            if(size <=  *i ){
                return *i;
            }
            
        }
        if(size > 1024 && size < 8192){
            return  127;
        }
        else{
            return -1; 
        }    
        
    }
    fn find_bestfit_bin(&self , size:i32) -> i32{
        for i in &self.bin_sizes{
            if(size == *i ){
                return *i; 
            }
        }
        if(size > 0 && size < 8192){
            return 127; 
        }
        else{
            return -1 ;
        }
      
    }
    fn split_block(&self , ptr:usize , splitsize:i32 ){
        

    }
    fn update_prev_ptr(){

    }
    fn insert_data(){

    }
    fn insert_data_extra_memory(){

    }
    fn find_free_block(){


    }
    fn coalesce(){


    }
    fn free(){


    }

    fn sbrk(){


    }
    fn allocate_sting (){


    }
    fn get_string(){

    }
    fn tests(){
        
    }

    fn malloc(){


    }





   







}





fn main() {
   let mut  allocator = Allocator{
        max_size: 0,
        heap: Vec::new(),
        total_memory_allocated : 0,
        free_lists: Vec::new(),
        bin_sizes : Vec::new() 
    };
    allocator.init_heap();
    let ptr = 4;
    allocator.len_header_init(&ptr);
    allocator.plen_header_init(&ptr , 32 );
    println!("{:?}",allocator.heap);
    



}
