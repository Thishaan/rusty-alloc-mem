macro_rules! assign_pool_size {
    ($val:expr) => {
        if $val % 2 != 0 {
            return None;
        }
    };
}

macro_rules! calc_block_size{
	
	 ($val:expr) => {
		
		block_size = val
  	
		if val <= 0{
		    return None;	
		}
		while (val & (val - 1)) == 0{
			block_size = block_size + 1; 
		}
	}
		

}


struct heap_space{

	unsafe{



	}		
	

}


pub fn allocate(alloc_size: u32) -> i32{
	
	let block_size = calc_block_size

	if block_size== None{
		return -1
	}


	

	return block_size;

}
