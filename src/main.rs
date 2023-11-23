use calamine::{ Reader, open_workbook_auto };
// use std::collections::HashMap; 
// use ahash::AHashMap;
use std::{thread, time::Duration, time::SystemTime};
use memory_stats::memory_stats;

pub struct ZU32(pub u32);

impl ZU32 {

	fn init(){
	    let start = SystemTime::now();
	    
	    // let path = "./Electric_Vehicle_Population_Data2.xlsx";
	    let path = "/home/brian/Downloads/Electric_Vehicle_Population_Data3.xlsx";
	    let mut workbook = open_workbook_auto(path).expect("Cannot open file");
	    println!("Execution Read {:?} ", ( SystemTime::now().duration_since(start).unwrap() ) );

	    // let mut data: HashMap<i32, Vec<String>> = HashMap::new();
	    let mut data: Vec< Vec<String> > = Vec::new() ;
	    let mut i: usize = 0;
	    
		if let Some(usage) = memory_stats() {
		    println!("Current physical memory usage: {}", usage.physical_mem);
		    println!("Current virtual  memory usage: {}", usage.virtual_mem);
		} else {
		    println!("Couldn't get the current memory usage :(");
		}
	    // let mut tmp :Vec<String> = Vec::new();
	    let test = workbook.worksheet_range( "Sheet1" );
	    // if let Some(Ok(r)) = workbook.worksheet_range( "Sheet1" ) {
	    //    for row in r.rows() {
	    //        data.push( Vec::new() );
	    //        for x in 0..row.len() {
	//		data.get_mut(i).expect("REASON").push( row[x].to_string() );
	  //          }
	    //        i+=1;
	    //    }
	    // }
	    if let Some(usage) = memory_stats() {
		    println!("Current physical memory usage: {}", usage.physical_mem);
		    println!("Current virtual  memory usage: {}", usage.virtual_mem);
		} else {
		    println!("Couldn't get the current memory usage :(");
		}
		
	    // println!("Count Data  {:?} ", data.len() );
	    // println!("Execution End  {:?} ", ( SystemTime::now().duration_since(start).unwrap().as_millis() ) );

	    // thread::sleep(Duration::from_millis(4000));
		
		
	    // unsafe {
	//	std::mem::drop(test);
	//	std::mem::drop(workbook);
	 //   }
		if let Some(usage) = memory_stats() {
		    println!("Current physical memory usage: {}", usage.physical_mem);
		    println!("Current virtual  memory usage: {}", usage.virtual_mem);
		} else {
		    println!("Couldn't get the current memory usage :(");
		}
	    println!("Execution Dealocate memory" );

	    thread::sleep(Duration::from_millis(10000));
	    println!("Execution exit" );
	}
}

impl Drop for ZU32 {
    fn drop(&mut self) {
        println!("zeroing memory");
        unsafe{ ::std::ptr::write_volatile(&mut self.0, 0) };
    }
}

pub fn test( ) {
	let mut datatmp: Vec<String>= Vec::new();
        
    // let er = ExcelRead {} ;
    // er.init( path );
    // readexcel::excel_read::init(path);
    
  
    let start = SystemTime::now();
		
    // let mut workbook = open_workbook_auto(path).expect("Cannot open file");
    println!("Execution Read {:?} ", ( SystemTime::now().duration_since(start).unwrap() ) );

    // let mut data: Vec<Vec<String>> = Vec::new();
    // let mut i: usize = 0;
    //  let mut tmp: String = String::new();
    // let excel = workbook.worksheet_range( "Sheet1" ).expect("Cannot open file");
    println!("The useful size of `v` is {}", std::mem::size_of_val(&*datatmp));

    // unsafe {
    	  let mut n = 0;
    	  while n < 2000000 {
    		datatmp.push( "askjlafaskfljasfklasjfklajsfklasjfalksfjassssssssssssssssssssssssssssssssflaksjfkalsfjaksjfalksfjalsfjlaskfjaslkfjlaskfjlaskjf".to_string() );
    		n += 1;
    	  } 
	    // if let Ok(ref r) = excel {
	    //     for row in r.rows() {
	             // data.push( Vec::new() );
	    //         for x in 0..row.len() {
		 			// data.get_mut(i).expect("REASON").push( row[x].to_string() );
		 			// DATA.get_mut(i).expect("REASON").push( row[x].to_string() );
		// 			tmp.push_str( &row[x].to_string() );
	    //         }
	    //         DATA.push( tmp );
	    //         tmp = String::new();
	             // i+=1;
	    //     }
	    // }
	    
	    println!("The useful size of `v` is {}", std::mem::size_of_val(&*datatmp));
	    println!("Count Data  {:?} ", datatmp.len() );
	    println!("Execution End  {:?} ", ( SystemTime::now().duration_since(start).unwrap().as_millis() ) ); 
    // }
    // datatmp.clear();
    // datatmp = Vec::new();
	// unsafe {
	//	std::mem::drop( workbook );
	//	std::mem::drop( excel );
	// 	std::mem::drop( tmp );
		std::mem::drop( datatmp );
	// }
	
	// process::exit(0x0100);
	
 	// let data = models::ExcelModel {
    //    rows_total: 0,
    //    columns_total: 0,
    // };
    // println!("pair contains {:?} and {:?}", data.rows_total, data.columns_total );
	
	// println!("pair class {:?}", class );
	// println!("The useful size of `v` is {}", std::mem::size_of_val(&*datatmp));
}

fn main() {
	let mut i = 0;
	while i < 1 {
		// test();
		ZU32::init();
		if let Some(usage) = memory_stats() {
		    println!("Current physical memory usage: {}", usage.physical_mem);
		    println!("Current virtual  memory usage: {}", usage.virtual_mem);
		} else {
		    println!("Couldn't get the current memory usage :(");
		}
		i+=1;
	}
	
//	 println!("===============================================");
    // ZU32::init();
//	 println!("===============================================");
// ZU32::init();
//	 println!("===============================================");
//ZU32::init();
//	 println!("===============================================");
//ZU32::init();
//	 println!("===============================================");
//ZU32::init();
     
}
