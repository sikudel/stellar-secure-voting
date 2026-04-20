#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, String, Vec};

// =========================================================================
// DATA STRUCTURES
// =========================================================================

/// Represents a single candidate in the election.
/// Stored as part of a Vec<Candidate> on the blockchain.
#[contracttype]
#[derive(Clone, Debug)]
pub struct Candidate {
    pub id: u32,           // Auto-assigned ID (index-based)
    pub name: String,      // Candidate name (e.g. "Alice")
    pub vote_count: u32,   // Number of votes received
}

/// Enum used as typed storage keys.
/// This prevents key collisions and keeps data organised on-chain.
#[contracttype]
pub enum DataKey {
    Candidates,       // Points to the Vec<Candidate> list
    Voter(Address),   // Points to the voting status (bool) of a specific wallet
}

// =========================================================================
// CONTRACT
// =========================================================================

#[contract]
pub struct SecureVotingContract;

#[contractimpl]
impl SecureVotingContract {

    // -------------------------------------------------------------------------
    // FUNCTION 1: add_candidate
    // -------------------------------------------------------------------------
    /// Adds a new candidate to the election.
    /// The ID is assigned automatically based on the current list length.
    ///
    /// # Arguments
    /// * `env`  - The Soroban environment (auto-injected by the SDK)
    /// * `name` - The candidate's display name
    ///
    /// # Returns
    /// The newly assigned candidate ID (u32)
    pub fn add_candidate(env: Env, name: String) -> u32 {
        // Load the current candidate list from storage, or create a new empty one
        let mut candidates: Vec<Candidate> = env
            .storage()
            .instance()
            .get(&DataKey::Candidates)
            .unwrap_or(Vec::new(&env));

        // The new ID equals the current length of the list (0-based)
        let new_id: u32 = candidates.len();

        // Build the new Candidate struct
        let candidate = Candidate {
            id: new_id,
            name,
            vote_count: 0,
        };

        // Append it to the list
        candidates.push_back(candidate);

        // Persist the updated list back to blockchain storage
        env.storage()
            .instance()
            .set(&DataKey::Candidates, &candidates);

        // Return the new candidate's ID
        new_id
    }

    // -------------------------------------------------------------------------
    // FUNCTION 2: vote
    // -------------------------------------------------------------------------
    /// Casts a vote for a candidate.
    ///
    /// Validations:
    ///   - The caller must not have voted before (prevents double voting)
    ///   - The candidate_id must point to an existing candidate
    ///
    /// # Arguments
    /// * `env`          - Soroban environment
    /// * `voter`        - The Address of the wallet casting the vote
    /// * `candidate_id` - The ID of the candidate to vote for
    pub fn vote(env: Env, voter: Address, candidate_id: u32) {
        // Require the transaction to be signed by `voter`.
        // This ensures nobody can vote on behalf of someone else.
        voter.require_auth();

        // ---- VALIDATION 1: Prevent double voting ----
        if Self::has_voted(env.clone(), voter.clone()) {
            panic!("Error: This wallet has already voted!");
        }

        // Load the current candidate list
        let mut candidates: Vec<Candidate> = env
            .storage()
            .instance()
            .get(&DataKey::Candidates)
            .unwrap_or(Vec::new(&env));

        // ---- VALIDATION 2: Ensure the candidate ID is valid ----
        if candidate_id >= candidates.len() {
            panic!("Error: Candidate ID does not exist!");
        }

        // Retrieve the target candidate, increment its vote count
        let mut candidate = candidates.get(candidate_id).unwrap();
        candidate.vote_count += 1;

        // Write the updated candidate back into its slot in the vector
        candidates.set(candidate_id, candidate);

        // Save the updated candidate list to storage
        env.storage()
            .instance()
            .set(&DataKey::Candidates, &candidates);

        // Mark the voter's address as having voted (true)
        env.storage()
            .instance()
            .set(&DataKey::Voter(voter), &true);
    }

    // -------------------------------------------------------------------------
    // FUNCTION 3: get_candidates
    // -------------------------------------------------------------------------
    /// Returns the complete list of candidates and their current vote counts.
    ///
    /// # Returns
    /// Vec<Candidate> — all candidates stored on-chain
    pub fn get_candidates(env: Env) -> Vec<Candidate> {
        // Return the stored list, or an empty vector if none exist yet
        env.storage()
            .instance()
            .get(&DataKey::Candidates)
            .unwrap_or(Vec::new(&env))
    }

    // -------------------------------------------------------------------------
    // FUNCTION 4: has_voted
    // -------------------------------------------------------------------------
    /// Checks whether a wallet address has already cast a vote.
    ///
    /// # Arguments
    /// * `env`   - Soroban environment
    /// * `voter` - The wallet Address to check
    ///
    /// # Returns
    /// `true` if the address has voted, `false` otherwise
    pub fn has_voted(env: Env, voter: Address) -> bool {
        // Look up the voter's status; default to false if not found
        env.storage()
            .instance()
            .get(&DataKey::Voter(voter))
            .unwrap_or(false)
    }
}
