use calamine::{ Reader, open_workbook_auto };
use std::time::SystemTime;
use std::collections::HashMap; 

fn main() {
    let start = SystemTime::now();
    
    let path = "/home/brian/Downloads/Electric_Vehicle_Population_Data3.xlsx";
    let mut workbook = open_workbook_auto(path).expect("Cannot open file");
    println!("Execution Read {:?} ", ( SystemTime::now().duration_since(start).unwrap() ) );

    let mut data: HashMap<i32, Vec<String>> = HashMap::new();
    let mut i: i32 = 0;
    if let Some(Ok(r)) = workbook.worksheet_range( "Sheet1" ) {
        for row in r.rows() {
            let mut tmp :Vec<String>  = Vec::new();
            for x in 0..row.len() {
                tmp.push( row[x].to_string() );
            }
            data.insert(i, tmp);
            i+=1;
        }
    }
    println!("Count Data  {:?} ", data.len() );
    println!("Execution End  {:?} ", ( SystemTime::now().duration_since(start).unwrap().as_millis() ) );
}
