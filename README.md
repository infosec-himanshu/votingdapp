# Soroban Voting DApp

A lightweight, secure, and transparent voting smart contract built on **Stellar's Soroban** smart contract platform.

## 📝 Project Description
The **Soroban Voting DApp** is a decentralized application designed to handle simple "Yes/No" or "Single Proposal" voting. By leveraging the Stellar network, the contract ensures that every vote is immutable, verifiable, and cast without the need for a central authority. This project serves as a foundational template for DAO governance or community polling.

## 🚀 What it does
This smart contract manages the lifecycle of a vote on the blockchain. It tracks the total number of votes cast and ensures the integrity of the process by preventing "double-voting." 

1. **Initialization**: Sets up the ledger state for a new voting session.
2. **Authentication**: Uses Soroban's native `require_auth` to verify the identity of the voter.
3. **State Management**: Updates the global vote tally while recording individual participation to enforce a "one-person-one-vote" rule.
4. **Data Retrieval**: Allows anyone to query the current vote count in real-time.

## ✨ Features
* **Sybil Resistance**: Each Stellar `Address` is restricted to a single vote.
* **Low Cost**: Optimized for Soroban’s efficient resource model, ensuring minimal gas fees.
* **Persistent Storage**: Data is stored in the persistent ledger, meaning results remain accessible even after the contract instance expires.
* **Secure Auth**: Built-in integration with Stellar wallets (like Freighter) for secure transaction signing.
* **Transparent Tally**: Publicly callable function to view results at any time during the election.


- [Rust](https://www.rust-lang.org/)
- [Stellar CLI](https://developers.stellar.org/docs/smart-contracts/getting-started/setup)
- Target `wasm32-unknown-unknown`

- wallet address- GAPYCJJJ5HEIV5VJTXYUA64BPBEEJA5CINLXGA7YDJTFBDBPWIPXCO74
- contract address- CAZONYXXRKSQWO2XJMJJOXIIQNG6MZQQT5PAV6XHKIVXJYTJPEKTB3BD
https://stellar.expert/explorer/testnet/contract/CAZONYXXRKSQWO2XJMJJOXIIQNG6MZQQT5PAV6XHKIVXJYTJPEKTB3BD
<img width="1871" height="853" alt="image" src="https://github.com/user-attachments/assets/98b9236d-61a7-43ee-aea8-7e606b4b5c27" />




1. **Build the contract:**
   ```bash
   stellar contract build
