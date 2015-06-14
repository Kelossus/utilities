

use std::thread;
use std::io;
use std::sync::{Mutex, Arc};
use std::mem;


///This function reads from standard input and return when timeout (ms) has passed
/// however, if enter was not pressed within 6 seconds it will return blank 
///
///# Examples
///
///```
///use util::read_line_with_timeout;
///
///read_line_with_timeout(1000);
///```

#[no_mangle]
pub extern fn read_line_with_timeout(timeout: u32) -> String {
    
    let prize = Arc::new(Mutex::new(0));
    let input= Arc::new(Mutex::new(String::new()));



    //first thread

    let prize1 = prize.clone();
    let input1 = input.clone(); 
    let _input_t = thread::spawn(move || {
        let mut input1 = input1.lock().unwrap();
        io::stdin().read_line(&mut *input1)
        .ok()
        .expect("Failed to read line"); 
        let mut prize1 = prize1.lock().unwrap();
        if *prize1 == 0 {
            *prize1 = 1;
        }

    });

    //second thread

    let prize2 = prize.clone();
    let timeout_t = thread::spawn(move || {
        thread::sleep_ms(timeout);
        let mut prize2 = prize2.lock().unwrap();
        if *prize2 == 0 {
            *prize2 = 2;
        }

    });

    timeout_t.join().unwrap();
    let prize = *prize.lock().unwrap();
    let mut result = input.lock().unwrap();
    if prize == 1 {
        return mem::replace(&mut *result, String::new());
    }
    "".to_string()

}


///This function creates the shifts of a pettern to be applied into a string, first step of KMP algorithm
///// ATENTION it's not working actually

#[no_mangle]
extern fn KMP_shifts(pattern: String) -> Vec<i32> {
    let mut shifts: Vec<i32> = vec!(0, pattern.len() as i32 +1 );
    let mut shift : i32 = 1;

    let mut pattern: Vec<char> =  pattern.chars().collect();

    for i in 0..(pattern.len() as i32 +1 ) {
        while shift < i && pattern.get((i-1) as usize) != pattern.get((i-shift-1) as usize) {
            println!("{}",(i-shift-1) as usize);
            shift += *shifts.get((i-shift-1) as usize).expect("failed");
        }
        shifts.insert((i-shift-1) as usize, shift);
    }

    shifts

}