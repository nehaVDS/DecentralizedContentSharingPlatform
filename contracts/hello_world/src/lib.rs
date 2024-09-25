#![allow(non_snake_case)]
#![no_std]
use soroban_sdk::{contract, contracttype, contractimpl, log, Env, Symbol, String, symbol_short};

#[contracttype]
#[derive(Clone)]
pub struct Content {
    pub content_id: u64,
    pub title: String,
    pub description: String,
    pub creator: String,
    pub price: u64,
    pub is_published: bool,
}


const CONTENT_COUNT: Symbol = symbol_short!("CONT_COU");

#[contract]
pub struct ContentContract;

#[contractimpl]
impl ContentContract {

    pub fn create_content(env: Env, title: String, description: String, creator: String, price: u64) -> u64 {
        let mut content_count: u64 = env.storage().instance().get(&CONTENT_COUNT).unwrap_or(0);
        content_count += 1;
        
        let content = Content {
            content_id: content_count,
            title,
            description,
            creator,
            price,
            is_published: false,
        };

        // Store the new content
        env.storage().instance().set(&content_count, &content); // Using content_count as the key
        env.storage().instance().set(&CONTENT_COUNT, &content_count);

        log!(&env, "Content Created with ID: {}", content_count);
        content_count
    }

    pub fn publish_content(env: Env, content_id: u64) {
        let mut content = Self::view_content(env.clone(), content_id);

        if content.is_published {
            log!(&env, "Content ID: {} is already published", content_id);
            panic!("Content already published!");
        }

        content.is_published = true;
        env.storage().instance().set(&content_id, &content); // Using content_id as the key

        log!(&env, "Content ID: {} is now published", content_id);
    }

    pub fn get_content_details(env: Env, content_id: u64) -> Content {
        Self::view_content(env, content_id)
    }

    fn view_content(env: Env, content_id: u64) -> Content {
        env.storage().instance().get(&content_id).unwrap_or(Content {
            content_id: 0,
            title: String::from_str(&env, "Not Found"),
            description: String::from_str(&env, "Not Found"),
            creator: String::from_str(&env, "Unknown"),
            price: 0,
            is_published: false,
        })
    }
}
