# Esusu Savings on Stellar

A Soroban-based rotating savings system inspired by traditional esusu/ajo/aduro models, built for trustless group savings on Stellar.

## Overview

This project aims to create a transparent, automated savings circle where members contribute a fixed amount on a schedule and receive payouts in rotation. By using Soroban smart contracts, the system removes the need for manual collection, trusted intermediaries, and fragile off-chain coordination.

The first version focuses on a simple, secure MVP for recurring contribution groups with automatic payout scheduling and rules enforcement.

## Problem

Traditional rotating savings groups often fail because of:

- missed contributions
- disputes over who should receive the pooled funds
- lack of transparency
- dependence on a trusted organizer
- poor record keeping
- limited accountability across members

On-chain coordination can improve trust, automate enforcement, and make contribution history visible to all participants.

## Goals

- Create a group savings smart contract on Stellar/Soroban
- Support recurring contributions with a fixed amount and schedule
- Rotate payouts to members in a predictable order
- Enforce contribution deadlines and penalties for missed payments
- Keep the logic simple enough for a strong MVP
- Prepare the contract for future extension with governance, token support, or mobile-first dashboards

## Core User Flow

1. A group organizer creates a savings circle.
2. Members join with a defined contribution amount and role.
3. The system sets a cycle length and payout order.
4. Each member contributes funds according to the schedule.
5. On each cycle, the pot is paid to the designated recipient.
6. The contract records contribution history and current cycle state.
7. Members can be marked late or removed if they fail to meet obligations.

## MVP Features

### Group Setup
- Create a savings group
- Set group name and description
- Define contribution amount
- Define cycle duration
- Set member limit
- Set maximum number of cycles

### Membership
- Join a group
- Assign each participant a position in the rotation
- Store user status: active, pending, or removed

### Contributions
- Deposit the agreed amount into the group pot
- Record deposit amount and timestamp
- Prevent duplicate contributions before the cycle deadline
- Allow only valid members to contribute

### Payout Logic
- Determine the current receiver based on rotation order
- Release funds to the selected member at the correct time
- Record payout events for auditability

### Enforcement Rules
- Detect missed contributions
- Flag late members
- Enforce penalties or restrictions in a future version
- Prevent unauthorized withdrawals from the group pool

## Proposed Contract Structure

The project will likely be organized as one or more Soroban contracts:

```text
contracts/
  esusu_pool/
    src/
      lib.rs
      test.rs
```

### Planned Contract Modules

- create_group
- join_group
- contribute
- trigger_payout
- get_group_state
- get_member_state
- get_cycle_state
- get_history

## Storage Model

The smart contract will need to store:

- group metadata
- member list and member status
- contribution ledger
- rotation order
- current cycle index
- total pool balance
- payout history
- timestamps and cycle deadlines

Because Soroban storage is limited and performance-sensitive, the contract should prefer compact, well-structured state records rather than over-normalized storage.

## Security Considerations

This is essential for a savings product:

- Validate caller authorization for all sensitive functions
- Prevent double-spending or duplicate deposits
- Ensure only active members can contribute or receive payouts
- Keep payout sequencing deterministic and transparent
- Avoid unrestricted withdrawals from the pool
- Add emergency pause or admin controls only if required by the MVP

## Contract Design Principles

- Simple and auditable logic
- Deterministic state transitions
- Explicit error handling for invalid states
- Minimal external dependencies
- Clear separation between group administration and member actions

## Example Business Rules

- A group has 5 members
- Each member contributes 100 XLM or a chosen asset
- Payout occurs in a round-robin order
- If a member misses a contribution, they are flagged
- The organizer can only perform admin actions for valid group states

## Future Enhancements

Once the MVP is stable, the following improvements are reasonable:

- configurable penalties for missed payments
- multi-asset support
- native Stellar token integration
- member voting / governance
- admin dashboards and analytics
- mobile-friendly front-end wallet experience
- contribution reminder notifications
- automated cycle rotation logic with recovery flows

## Tech Stack

- Stellar network
- Soroban smart contracts
- Rust
- Stellar CLI
- Testnet deployment for validation
- Optional frontend later for wallet interaction and dashboards

## Roadmap

### Phase 1: Foundation
- define group and member state
- create the contract skeleton
- implement membership logic
- implement contribution tracking

### Phase 2: Core Savings Flow
- payout rotation
- cycle state management
- member validation and timeout rules

### Phase 3: Security and Testing
- write unit tests for edge cases
- validate contribution edge cases
- simulate missed payments and payout sequencing

### Phase 4: Release
- deploy to testnet
- verify end-to-end flows
- prepare a user-facing onboarding flow

## Success Criteria

The MVP is successful when:

- a group can be created
- members can join successfully
- contributions can be tracked reliably
- payouts happen in order
- invalid states fail gracefully with clear errors
- contract behavior is covered by tests

## Notes

This README is intentionally written as a project brief to guide the first Soroban contract implementation. The next step is to formalize the exact state machine, write the failing tests, and build the contract to match the planned flow.

## Recommended Next Step

Start by defining the minimal contract state and writing tests for:

1. group creation
2. member joining
3. contribution acceptance
4. payout rotation
5. missed payment handling

After that, implement the contract logic and validate it against the test cases.
