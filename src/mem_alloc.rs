macro_rules! assign_pool_size {
    ($val:expr) => {
        if $val % 2 != 0 {
            return None;
        }
    };
}

macro_rules! calc_block_size{

	 ($val:expr) => {
	  	
		if val <= 0{
		    return None;	
		}
		
		while (val & (val - 1)) == 0{
			val =val+1; 
		}
	}
		

}



pub fn allocate(alloc_size u32){
	
	if(alloc_size

}
