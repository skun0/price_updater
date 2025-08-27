use std::io::{self, Read, Write};
use std::fs::{File, OpenOptions};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct Product {
    id: u32,
    name: String,
    price: i32,
}

fn main() {
    let mut products: Vec<Product> = if let Ok(mut file) = File::open("products.json") {
        let mut content = String::new();
        file.read_to_string(&mut content).unwrap();
        serde_json::from_str(&content).unwrap_or_else(|_| vec![
            Product { id: 1, name: "NVIDIA GeForce RTX 5090".to_string(), price: 2539 },
            Product { id: 2, name: "NVIDIA GeForce RTX 5070".to_string(), price: 549 },
        ])
    } else {
        vec![
            Product { id: 1, name: "NVIDIA GeForce RTX 5090".to_string(), price: 2539 },
            Product { id: 2, name: "NVIDIA GeForce RTX 5070".to_string(), price: 549 },
        ]
    };

    for p in &products {
        println!("Product: {} [ID: {}] - Price: {}", p.name, p.id, p.price);
    }
    println!();

    let mut id_input = String::new();
    println!("Enter Product ID:");
    io::stdin()
        .read_line(&mut id_input)
        .expect("Error reading input");
    let id_input = id_input.trim();

    if let Some(prod) = products.iter_mut().find(|p| p.id.to_string() == id_input) {
        let mut new_price = String::new();
        println!("Current price: {}.\nEnter new price:", prod.price);

        io::stdin()
            .read_line(&mut new_price)
            .expect("Error reading input");

        if let Ok(price) = new_price.trim().parse::<i32>() {
            prod.price = price;
            println!("Price update for {}: {}", prod.name, prod.price);
        } else {
            println!("Invalid value!");
        }
    } else {
        println!("Invalid ID!");
    }

    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open("products.json")
        .expect("Error opening file");

    let json = serde_json::to_string_pretty(&products).expect("Error serializing json");
    file.write_all(json.as_bytes()).expect("Error writing json");

    let mut exit = String::new();
    println!("Press any key to exit");
    io::stdin()
        .read_line(&mut exit)
        .expect("Error");
}
