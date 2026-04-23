#![no_std]
use soroban_sdk::{contract, contractimpl, symbol_short, Address, Env, Symbol, log};

#[contract]
pub struct VotingContract;

// Define storage keys
const VOTE_COUNT: Symbol = symbol_short!("VOTES");
const HAS_VOTED: Symbol = symbol_short!("VOTED");

#[contractimpl]
impl VotingContract {
    /// Initialize the voting process. 
    /// Can be called once to set the starting state.
    pub fn init(env: Env) {
        if !env.storage().persistent().has(&VOTE_COUNT) {
            env.storage().persistent().set(&VOTE_COUNT, &0u32);
        }
    }

    /// Cast a vote.
    /// @param voter The Address of the person voting.
    pub fn cast_vote(env: Env, voter: Address) {
        // 1. Verify the caller is who they say they are
        voter.require_auth();

        // 2. Check if this address has already voted
        // We use a tuple (HAS_VOTED, voter) as a unique key in persistent storage
        let voter_key = (HAS_VOTED, voter.clone());
        if env.storage().persistent().has(&voter_key) {
            panic!("Address has already cast a vote");
        }

        // 3. Increment the total vote count
        let mut total_votes: u32 = env.storage().persistent().get(&VOTE_COUNT).unwrap_or(0);
        total_votes += 1;

        // 4. Update storage: Save new count and mark voter as used
        env.storage().persistent().set(&VOTE_COUNT, &total_votes);
        env.storage().persistent().set(&voter_key, &true);

        log!(&env, "Vote cast successfully by: {}", voter);
    }

    /// Returns the current total number of votes.
    pub fn get_tally(env: Env) -> u32 {
        env.storage().persistent().get(&VOTE_COUNT).unwrap_or(0)
    }
}
