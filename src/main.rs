use std::{io::{self, Read}, thread, sync::{Arc, Mutex}};

use ringbuf::{HeapRb, HeapProducer, HeapConsumer};

fn main() {

    let (mut producer, mut consumer) = HeapRb::<u8>::new(1024).split();
    // let rb = HeapRb::<u8>::new(1024);
    // let stdin = io::stdin();


    // producer.


    let h = thread::spawn(move || {
        let mut buf: [u8; 10] = [0; 10];
        loop {
            while let Ok(v) = io::stdin().read(&mut buf) {
                println!("[PRODUCER] Read {} bytes", v);
                producer.push_slice(&buf);  
            }
        }
        // match io::stdin().read(&mut buf) {
        //     Ok(v) => {
        //         println!("[PRODUCER] Read {} bytes", v);
        //         producer.push_slice(&buf);
        //     }
        //     Err(e) => {
        //         println!("cant read coz {}", e);
        //     }
        // }
    });
    
    // h.join().unwrap();

    let mut buf2: [u8; 10] = [0; 10];
    
    loop {
        while let Ok(v) = consumer.read(&mut buf2) {
            println!("[CONSUMER] Read {} bytes", v);
        }
    }
    //     match 
    //     Ok(v) => {
    //         // println!("Buffer is {:?}", buf2);
    //     }
    //     Err(e) => {
    //         println!("cant read coz {}", e);
    //     }
    // }



    // println!("Buf is {:?}", buf);
    // buf[0] = 4;
    // println!("Buf is {:?}", buf);

}
