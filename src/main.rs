use calamine::{ Reader, open_workbook_auto };
// use std::collections::HashMap; 
use ahash::AHashMap;
use std::{thread, time::Duration, time::SystemTime};


fn main() {
    let start = SystemTime::now();
    
    // let path = "./Electric_Vehicle_Population_Data2.xlsx";
    let path = "./Electric_Vehicle_Population_Data3.xlsx";
    let mut workbook = open_workbook_auto(path).expect("Cannot open file");
    println!("Execution Read {:?} ", ( SystemTime::now().duration_since(start).unwrap() ) );

    // let mut data: HashMap<i32, Vec<String>> = HashMap::new();
    let mut data: AHashMap<i32, Vec<String>> = AHashMap::new();
    let mut i: i32 = 0;
    // let mut tmp :Vec<String> = Vec::new();
    if let Some(Ok(r)) = workbook.worksheet_range( "Sheet1" ) {
        for row in r.rows() {
            data.insert(i, Vec::new() );
            for x in 0..row.len() {
		data.get_mut(&i).expect("DATA NOT FOUND").push( row[x].to_string() );
                // tmp.push( row[x].to_string() );
            }
	    // tmp.clear();
            i+=1;
        }
    }
    println!("Count Data  {:?} ", data.len() );
    println!("Execution End  {:?} ", ( SystemTime::now().duration_since(start).unwrap().as_millis() ) );

    thread::sleep(Duration::from_millis(4000));
}
