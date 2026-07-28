# Anchor SOL Vault

Simple SOL vault: deposit / withdraw / close, keyed by owner PDAs.

## Layout

- `programs/vault/src/lib.rs` — program logic
- `tests/vault.ts` — integration tests
- `Anchor.toml` — Anchor config

## Instructions

| Name | What it does |
|------|----------------|
| `initialize` | Create state PDA; remember bumps |
| `deposit` | Transfer SOL owner → vault PDA |
| `withdraw` | Transfer SOL vault PDA → owner (PDA signer) |
| `close` | Drain vault + close state (rent to owner) |

## Note

You need the Solana + Anchor CLIs installed to build/test (`anchor build`, `anchor test`).
