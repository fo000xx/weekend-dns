use weekend_dns::{TYPE_A, build_query};

fn main() {
    let domain = "www.example.com".to_string();
    let query_bytes = build_query(domain.clone(), TYPE_A);

    println!("DNS Query for: {}", domain);
    println!("Hex: {:02x?}", query_bytes);
    println!("Total bytes: {}", query_bytes.len());
}
