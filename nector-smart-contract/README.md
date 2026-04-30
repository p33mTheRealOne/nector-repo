# Nector Smart Contract
Nector smart contract implementation — a deterministic escrow state machine handling funding, delivery, disputes, and timeouts. Built in Rust and TypeScript, and this is the exact same smart contract used in production on Nector.

## Overview

Nector is a chat-native, non-custodial escrow system designed to enable safe transactions between strangers.

This repository contains the exact smart contract used in production, written in:

- Rust (on-chain program)
- TypeScript (clients, scripts, automation)

The contract is designed as a deterministic state machine, where every transition is enforced by:

- strict rules
- time-based constraints
- economic incentives

## Core Features

- Non-custodial escrow (no trusted third party)
- Built-in timeout system (confirm, discussion, respond, shipping)
- Dispute resolution with economic penalties
- Bond-based incentive system
- Keeper-compatible (anyone can trigger timeouts)
- Fully open-source & verifiable

## How it Works
https://nector.chat/docs/smart-contract

## Prerequites
Install Rust, the Solana CLI, and Anchor Framework on Windows (WSL), Linux, or Mac.

https://www.anchor-lang.com/docs/installation

```bash
# Verify that the installation was successful, check the Rust version:
rustc --version

# Verify that the installation was successful, check the Solana CLI version:
solana --version

# Verify that the installation was successful, check the Anchor CLI version:
anchor --version

# See your current config:
solana config get

# Set to devnet:
solana config set --url devnet

# Create new wallet:
solana-keygen new

# Request an airdrop of devnet SOL:
solana airdrop 5

# Check your wallet's SOL balance:
solana balance # If it show 5 you good to go!
```

Prepare the project
```bash
# Clone repo:
git clone https://github.com/...

# Install node modules
yarn
```

## Start

### Build Program

```bash
# Build the program
# This will generate new keypair for the program if doesn't exist
anchor build

# Sync all keys:
anchor keys sync

# If the program_id in lib.rs changed. Build the program again:
anchor build
```

### Deploy program

```bash
# Set to devnet:
solana config set --url devnet

# Make sure you have some SOL in your wallet beacuse this will cost you some SOL
solana balance

# Deploy program to devnet 
anchor deploy
```

### Test

https://github.com/p33mTheRealOne/nector-smart-contract/tree/main/tests/how_to_use

# Learn more:
https://nector.chat/docs
