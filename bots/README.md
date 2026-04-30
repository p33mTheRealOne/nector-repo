# Nector Keeper Bot

Nector keeper bot implementation — an automated timeout trigger system for the Nector smart contract. It continuously monitors escrow orders and executes timeout instructions when deadlines are reached.

Built in TypeScript, this is the same keeper architecture used by Nector to ensure disputes, shipping deadlines, and review deadlines are enforced automatically.

---

## Overview

Solana programs cannot execute themselves.

That means timeout logic inside the smart contract still requires an external transaction sender.

This repository contains the keeper bot responsible for monitoring deadlines and triggering timeout instructions automatically.

The bot does NOT control business logic.

All rules are enforced inside the smart contract.

The keeper bot is only responsible for:

- checking deadlines
- detecting timeout conditions
- sending valid transactions to trigger state transitions

This ensures users do not need to stay online for the system to work.

Timeouts happen automatically.

---

## Core Features

- Automatic timeout execution
- Shipping timeout trigger
- Review timeout trigger
- Seller response timeout trigger
- Discussion timeout trigger
- Fully compatible with open trigger design
- Works with public smart contract logic
- No admin privileges required
- Fully open-source & verifiable

---

## Supported Timeout Types

### 1. Shipping Timeout

If seller funds escrow but fails to mark shipped before deadline:

- seller loses penalty
- buyer receives compensation
- remaining funds are refunded automatically

---

### 2. Review Timeout

If seller marks shipped but buyer does not confirm or open dispute within 24h:

- seller gets paid automatically
- both sides receive bond refunds

---

### 3. Response Timeout

If buyer opens dispute but seller does not respond within 24h:

- buyer wins dispute automatically
- seller loses penalty

---

### 4. Discussion Timeout

If seller responds but neither side resolves the dispute within 24h:

- dispute ends in Draw
- all escrow funds go to treasury

---

## Prerequisites

Install:

- Node.js
- Yarn
- Solana CLI
- Anchor Framework

Recommended setup guide:

https://www.anchor-lang.com/docs/installation

Verify installation:

```bash
node --version
yarn --version
solana --version
anchor --version
```

## Test
https://github.com/p33mTheRealOne/nector-repo/blob/main/bots/timeout/(0)%20setup.txt
