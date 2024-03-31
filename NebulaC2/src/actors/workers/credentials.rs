

// This file will be responsible for handling the credentials
// of the workers. The credentials will be stored in the database using Polars

use std::fs::File;

// Import time shit
use chrono::prelude::*;

// Define your schema
table! {
    credentials (id) {
        id -> Integer,
        username -> Varchar,
        
        // We need to implement some functionality for ID'ing hashes in Rust.
        // I want to be able to ID if a password is being stored as a hash or not. If it is a hash, it should be sent to a cracker like hashcat.
        // If it is not a hash, it should be cleartext, and sent to the database for storage.
        hashed_password -> Varchar,
        cracked_password -> Varchar,

        // We need to identify what the purpose of the credential is
        // Like what site is it for type shit
        purpose -> Varchar,

    }
}

// Define a struct matching your Diesel schema
#[derive(Insertable)]
#[table_name="credentials"]
struct NewCredential {
    username: String,
    hashed_password: String,
    cracked_password: String,
    purpose: String,
}

// Assume we have established a database connection
// let connection = establish_connection(); // Defined elsewhere


use polars::prelude::*;

fn process_data_with_polars() -> Result<Vec<NewCredential>> {
    let df = CsvReader::from_path("credentials.csv")?.infer_schema(None).finish()?;

    // Process your data here as needed with Polars
    // For simplicity, let's assume we just convert it directly

    let usernames: Vec<String> = df.column("username")?.utf8()?.into_iter().collect();
    let passwords: Vec<String> = df.column("password")?.utf8()?.into_iter().collect();

    // Combine usernames and passwords into NewCredential structs
    let credentials = usernames.into_iter().zip(passwords.into_iter())
        .map(|(username, password)| NewCredential { username, password })
        .collect();

    Ok(credentials)
}
