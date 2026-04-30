# Nector

## Chat-Based Non-Custodial Escrow for Digital & Physical Products

Nector is a chat-based non-custodial escrow platform built on Solana that enables safe transactions between strangers without requiring admins, KYC, or centralized intermediaries.

Users can create deals, fund escrow, deliver products, confirm delivery, and resolve disputes directly inside chat.

Smart contracts enforce outcomes automatically using timeout logic, economic incentives, and game theory.

Our goal is simple:

**Make trustless online commerce as easy as sending a message.**

---

# Problem

Today, fraud still exists even though many escrow services already exist.

Why?

Because most escrow platforms:

* are too complex to use
* require KYC before access
* depend on centralized admins
* separate escrow from communication
* create friction that makes users avoid using them

As a result, people still buy and sell through Telegram, Discord, Instagram, Facebook groups, and DMs without protection.

Buyers fear getting scammed.
Sellers fear not getting paid.

Trust remains one of the biggest unsolved problems on the internet.

---

# Solution

Nector embeds escrow directly into chat.

Instead of switching between messaging apps and external escrow platforms, users can handle the full transaction flow in one place:

* create order
* fund escrow
* deliver product
* confirm delivery
* open dispute
* resolve outcome

No admin required.

No trust required.

Only smart contract execution.

---

# Core Innovation — Draw Dispute Mode

Nector replaces manual arbitration with economic incentives.

Instead of human admins deciding disputes, users choose a **Draw Dispute Mode** during order creation.

There are only 2 modes:

## Buyer Take Risk (BTR)

Buyer loses the most if a dispute ends in draw.

## Seller Take Risk (STR)

Seller loses the most if a dispute ends in draw.

This forces both parties to stay honest because dragging disputes becomes expensive.

If neither side resolves the dispute before the deadline:

**all escrow funds are sent to the platform treasury.**

This prevents abuse and removes dependency on manual dispute resolution.

---

# How It Works

## Physical Product Flow

### 1. Seller Creates Order

Seller chooses:

* product type (physical)
* Draw Dispute Mode (BTR / STR)
* product image
* description
* price
* shipping deadline

Before buyer funds, clear warnings explain which side takes more risk.

---

### 2. Buyer Funds Escrow

Buyer pays:

* product price
* 20% bond
* platform fee

---

### 3. Seller Funds Escrow

Seller pays bond based on mode:

### BTR

20% bond

### STR

120% bond

After seller funds, shipping timer starts.

---

### 4. Shipping Phase

Seller must ship before deadline.

If seller misses shipping deadline:

* seller loses 20% penalty
* half goes to buyer
* half goes to treasury
* remaining funds are refunded

---

### 5. Buyer Review

After seller marks shipped, buyer has 24h to:

* confirm delivery
* open dispute

If buyer does nothing:

* seller receives payment automatically
* both bonds are returned

---

### 6. Dispute

Only buyer can open dispute.

Reasons:

* item not received
* item not as described

Seller then has 24h to:

* refund
* respond

If seller does nothing:

* buyer wins automatically

---

### 7. Discussion Phase

If seller responds:

24h discussion starts.

During this phase:

Seller can:

* refund buyer

Buyer can:

* pay seller

If nobody acts:

### Draw happens

All escrow funds go to treasury.

---

## Digital Product Flow

Digital products use:

### BTR only

Seller uploads the file after funding.

Buyer receives access before final confirmation.

If buyer says product is not as described:

Dispute opens.

---

# Timeout System

Nector uses 4 timeout types:

## 1. Shipping Timeout

Seller must ship before deadline.

## 2. Review Timeout

Buyer must confirm or dispute within 24h.

## 3. Response Timeout

Seller must respond to dispute within 24h.

## 4. Discussion Timeout

If discussion expires → Draw

---

# Keeper Bot

Solana programs cannot execute themselves.

Nector uses a keeper bot that continuously monitors deadlines and triggers timeout transactions automatically.

Important:

The logic exists inside the smart contract.

The bot is only the transaction sender.

This means:

* bot can trigger timeout
* anyone can trigger timeout
* but only after deadline is reached

Smart contracts use:

`block.timestamp`

for deadline validation.

---

# Features

* Chat-based escrow
* Non-custodial settlement
* Physical + digital products
* Draw Dispute Modes (BTR / STR)
* Timeout automation
* No admin dependency
* No KYC required
* Open-source smart contracts
* Open-source SDK
* Nector Mini for developers

---

# Nector Mini (https://www.nector.chat/docs/nector-mini)

Nector Mini allows developers and founders to build their own chat-based no-admin escrow platform without starting from scratch.

We open-source:

* smart contracts
* SDK
* keeper bot
* core escrow architecture

This makes Nector infrastructure, not just an app.

Our long-term vision is:

**Become the escrow layer of the internet.**

---

# Smart Contract Overview (https://www.nector.chat/docs/smart-contract)

Built using:

* Solana
* Anchor Framework
* Rust

Single Anchor program handles:

* create order
* buyer fund escrow
* seller fund escrow
* shipping flow
* confirm delivery
* dispute opening
* seller response
* refund path
* buyer win path
* draw resolution
* timeout execution
* bond and penalty logic

---

# Repository Structure

```text
/idl   → Anchor smart contracts
/bots               → Keeper timeout trigger bot
/app          → Main web app
```

---

# Tech Stack

* Solana
* Anchor
* Rust
* TypeScript
* Next.js
* React
* Supabase
* Vercel
* GitHub

---

# Why Only Possible on Solana

Nector requires:

* fast finality
* low fees
* cheap micro-transactions
* frequent escrow state updates
* scalable on-chain execution

This is only practical on Solana.

Escrow systems with multiple state transitions and timeout triggers become too expensive or too slow elsewhere.

---

# Business Model

Nector makes money through:

## 1. Transaction Fee

~1% fee on completed escrow orders

## 2. Penalty Revenue

Part of penalties from disputes and timeout violations goes to treasury

---

# Current Status

* Smart contract architecture completed
* Frontend mostly completed
* Running on devnet
* Preparing for mainnet launch
* Applying for Colosseum accelerator
* Fundraising for security, growth, and launch

---

# Demo

## Documentation

[https://www.nector.chat/docs/introduction](https://www.nector.chat/docs/introduction)

## GitHub

[https://github.com/p33mTheRealOne/Nector_repo](https://github.com/p33mTheRealOne/Nector_repo)

---

# Future Plans

* Mainnet launch
* Smart contract security audit
* First beta users
* Marketplace integrations
* Nector Mini expansion
* Launch Nector1k (https://www.nector.chat/docs/nector1k)

---

# License

MIT License

---

# Final Vision

Nector is not another marketplace.

We are building trustless commerce infrastructure.

A system where strangers can safely transact without trusting each other.

A world where trust comes from code, not from people.

**Nector = The Escrow Layer of the Internet**

## Getting Started
```
# Clone project:
git clone https://github.com/p33mTheRealOne/Nector_repo

cd Nector_repo-main

# Install dependencies:
yarn
```

Create a project in https://supabase.com/

Set Site URL in URL Configuration (Supabase):
```
https://localhost:3000
```

Add Redirect URLs in URL Configuration (Supabase):
```
https://localhost:3000/auth/callback
https://localhost:3000/auth/reset
```

## Create .env.local
```
# Create file:
touch .env.local

# Open .env.local
sudo nano .env.local
```

Put this in .env.local
```
NEXT_PUBLIC_SUPABASE_URL=https:// Your supabase url
NEXT_PUBLIC_SUPABASE_ANON_KEY=// Your supabase anon key

SUPABASE_SERVICE_ROLE_KEY=// Your supabase service role key
```

Save file
```
# exit file
Ctrl + X

# Press y to save

# Press Enter
```

## Run
```
npm run dev
```
Go to:
http://localhost:3000

## Learn more:
https://nector.chat/docs
