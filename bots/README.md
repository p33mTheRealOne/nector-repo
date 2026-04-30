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
