#![allow(warnings)]
use std::{i32, usize};



struct Allocator{
    max_size: i32 ,
    heap: Vec<i32>,
    total_memory_allocated : i32,
    free_lists: Vec<Vec<usize>>, 
    bin_sizes : Vec<i32> 
    

    
}

impl Allocator{
    //  X status X prevSize ..... size ... X size x status 

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
        self.free_lists[127].push(2); 
        self.header_init(&0 , 8188); 
        
    }

    fn header_init(&mut self , header_pointer :&usize , size:i32){
        self.len_header_init(header_pointer, size);
        self.plen_header_init(header_pointer); 
    }
    fn len_header_init( &mut self , header_pointer :&usize , size: i32  ){
        let mut ptr =  *header_pointer ;
        self.heap[(ptr + 2  + size as usize ) ] = size; //
        self.heap[(ptr + 3   + size as usize ) ] = 1 ; //
        self.heap[ptr + 1 ] = size; // size meta data 

    } 
    fn plen_header_init(&mut self , header_pointer: &usize  ){
        let size: i32 = self.heap[*header_pointer + 1]; 
        self.heap[*header_pointer ] = 1; // status 
        if(*header_pointer >  0){
            print!("{}" , self.heap[*header_pointer -1 ]); 
            if(self.heap[*header_pointer - 1]   == 1){
                print!("gg");
                self.heap[*header_pointer + 2 ] = self.heap[*header_pointer - 2 ]; // sets prevSize meta data 
            }
        }
    }
    fn find_free_block(&self , size :i32 , bin :&Vec<usize> ) -> usize {
        for i in bin{
            if(self.heap[*i] >= size){
                return bin[*i]
            }
        }
        return 129; 

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
    fn insert_data(&self , size:i32  , ptr_free_block:&usize  ){
        let n = ptr_free_block ; 

    }
    fn insert_data_extra_memory(){

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
    fn allocate_memory(&self , size:i32 ,  binIndex :i32){
        let mut  ptr_free_block :usize; 
        let mut binIndex; 
        let mut ptrAllocatedBlock  :usize; 
        for i in &self.free_lists{
            if(i.is_empty() != true ){
                if(self.find_free_block(size, i )!= 128 ){
                    ptr_free_block = self.find_free_block(size, i);
                    binIndex = i ; 
                }
            }


        }
        


            

            
        



    }
    fn malloc(&self , size: i32) -> usize {
        let binIndex = self.find_bin(size); 
        match binIndex {
            -1 => print!("called sbrk"), 
            _=> print!("normal case calls malloc ")
            
        }
        




        0
        


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
    allocator.init_free_lists();
    let ptr = 0 ;
    let ptr1 =  14; 
    allocator.header_init(&ptr , 10);
    allocator.header_init(&ptr1 , 14);

    println!("{:?}",allocator.heap);
    



}
