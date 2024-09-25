#![allow(non_snake_case)]
#![no_std]
use soroban_sdk::{contract, contracttype, contractimpl, log, Env, Symbol, String, symbol_short};

// Define the structure for storing loyalty points
#[contracttype]
#[derive(Clone)]
pub struct LoyaltyPoints {
    pub user_id: u64,      // Unique identifier for the user
    pub points: u64,       // Number of loyalty points
    pub total_issued: u64, // Total points issued
    pub total_redeemed: u64, // Total points redeemed
}

// Define symbols for storage
const POINTS_BOOK: Symbol = symbol_short!("POINTS");
const TOTAL_ISSUED: Symbol = symbol_short!("TOTAL");
const TOTAL_REDEEMED: Symbol = symbol_short!("TOTAL_RED");

#[contract]
pub struct LoyaltyProgramContract;

#[contractimpl]
impl LoyaltyProgramContract {

    // Initialize the contract
    pub fn initialize(env: Env) {
        env.storage().instance().set(&TOTAL_ISSUED, &0);
        env.storage().instance().set(&TOTAL_REDEEMED, &0);
    }

    // Issue loyalty points to a user
    pub fn issue_points(env: Env, user_id: u64, points: u64) {
        let mut user_points = Self::view_user_points(env.clone(), user_id.clone());
        user_points.points += points;
        user_points.total_issued += points;

        // Update the total issued points
        let mut total_issued = env.storage().instance().get(&TOTAL_ISSUED).unwrap_or(0);
        total_issued += points;
        env.storage().instance().set(&TOTAL_ISSUED, &total_issued);

        // Store updated user points
        let key = (POINTS_BOOK, user_id);
        env.storage().instance().set(&key, &user_points);

        log!(&env, "Issued {} points to user ID: {}", points, user_id);
    }

    // Redeem loyalty points for a user
    pub fn redeem_points(env: Env, user_id: u64, points: u64) {
        let mut user_points = Self::view_user_points(env.clone(), user_id.clone());

        if user_points.points < points {
            log!(&env, "Insufficient points for user ID: {}", user_id);
            panic!("Insufficient points!");
        }

        user_points.points -= points;
        user_points.total_redeemed += points;

        // Update the total redeemed points
        let mut total_redeemed = env.storage().instance().get(&TOTAL_REDEEMED).unwrap_or(0);
        total_redeemed += points;
        env.storage().instance().set(&TOTAL_REDEEMED, &total_redeemed);

        // Store updated user points
        let key = (POINTS_BOOK, user_id);
        env.storage().instance().set(&key, &user_points);

        log!(&env, "Redeemed {} points for user ID: {}", points, user_id);
    }

    // View loyalty points of a user
    pub fn view_user_points(env: Env, user_id: u64) -> LoyaltyPoints {
        let key = (POINTS_BOOK, user_id);
        env.storage().instance().get(&key).unwrap_or(LoyaltyPoints {
            user_id,
            points: 0,
            total_issued: 0,
            total_redeemed: 0,
        })
    }

    // View total issued points
    pub fn view_total_issued(env: Env) -> u64 {
        env.storage().instance().get(&TOTAL_ISSUED).unwrap_or(0)
    }

    // View total redeemed points
    pub fn view_total_redeemed(env: Env) -> u64 {
        env.storage().instance().get(&TOTAL_REDEEMED).unwrap_or(0)
    }
}
