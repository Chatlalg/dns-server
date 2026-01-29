// youtube.com = without concurrent processing
// Average latency: 769
// p50: 728, p95: 929, p99: 1377

use dns_server::helpers::lookup;
use dns_server::query::QueryType;
use std::net::Ipv4Addr;
use std::time::Instant;

fn main(){
    let server: (Ipv4Addr, u16) = (Ipv4Addr::new(127,0,0,1), 2053);
    let mut average = 0;
    let mut response_times: Vec<u128> = Vec::new();
    for _ in 0..100 {
        let now = Instant::now();
        let _ = lookup("chatlachaddar.com",QueryType::A, server).unwrap();
        let elapsed_time = now.elapsed();
        response_times.push(elapsed_time.as_millis());
        average = elapsed_time.as_millis() + average;
        println!("{}",elapsed_time.as_millis());
    }
    response_times.sort();
    let p95 = response_times[94];
    let p99 = response_times[98];
    let p50 = response_times[49];
    average = average / 100;
    println!("Average latency: {}\np50: {}, p95: {}, p99: {}", average, p50, p95, p99); 
}


