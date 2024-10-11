// use mysql::*;
// use mysql::prelude::*;
// use std::env;
// use dotenv::dotenv;

// fn main() {
//     // Load environment variables from a .env file
//     dotenv().ok();

//     // Get the connection URL from environment variables
//     let database_url = env::var("DATABASE_URL")
//         .expect("DATABASE_URL must be set in .env file or environment");

//     // Create a connection pool
//     let pool = Pool::new(database_url).expect("Failed to create MySQL connection pool");

//     // Get a connection from the pool
//     let mut conn = pool.get_conn().expect("Failed to get MySQL connection");

//     // Example: Create a table if it doesn't exist
//     conn.query_drop(
//         r"CREATE TABLE IF NOT EXISTS users (
//             id INT PRIMARY KEY AUTO_INCREMENT,
//             name VARCHAR(50),
//             age INT
//         )"
//     ).expect("Failed to execute query");

//     // Example: Insert some data
//     conn.exec_drop(
//         "INSERT INTO users (name, age) VALUES (:name, :age)",
//         params! {
//             "name" => "Alice",
//             "age" => 30,
//         }
//     ).expect("Failed to insert data");

//     // Example: Select data
//     let users: Vec<(i32, String, i32)> = conn.query("SELECT id, name, age FROM users")
//         .expect("Failed to fetch data");

//     for (id, name, age) in users {
//         println!("ID: {}, Name: {}, Age: {}", id, name, age);
//     }
// }
