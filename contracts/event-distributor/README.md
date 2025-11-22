# Event Distributor Smart Contract

> Advanced Soroban smart contract for managing validated participants and equitable pool distribution for regenerative finance events.

## Overview

The Event Distributor contract provides on-chain storage for humans (participants) with validation status, event management with participant lists, and automatic equitable distribution to validated participants only.

**Key Features:**
- On-chain participant storage with IPFS metadata
- Validation status management for participants
- Event creation with participant lists and funding pools
- Automatic filtering and distribution to validated participants only
- Comprehensive event emission for auditability
- Pagination support for large participant lists

## Architecture

```
Event Distributor
├── Human Management
│   ├── Add participants with IPFS hash
│   ├── Update validation status
│   ├── Update IPFS metadata
│   └── Query with pagination
├── Event Management
│   ├── Create events with location and pool
│   ├── Add participants to events
│   ├── Query event details
│   └── Get validated participants
└── Distribution
    └── Equitable distribution to validated humans only
```

## Data Models

### Human
```rust
pub struct Human {
    pub address: Address,      // Participant's address
    pub validated: bool,        // Validation status
    pub ipfs_hash: String,      // IPFS hash for metadata (image, bio, etc.)
}
```

### Event
```rust
pub struct Event {
    pub location: String,       // Event location
    pub event_name: String,     // Event name/identifier
    pub humans: Vec<Address>,   // List of participant addresses
    pub pool: i128,             // Total funding pool amount
}
```

## API Reference

### Initialization

#### `init(admin: Address)`
Initialize contract with admin address. Can only be called once.

**Parameters:**
- `admin`: Address with admin privileges

**Errors:**
- `AlreadyInitialized`: Contract already initialized

---

### Admin Functions

#### `get_admin() -> Address`
Get the admin address.

**Returns:** Admin address

**Errors:**
- `AdminNotFound`: Contract not initialized

---

### Human Management

#### `add_human(human_address: Address, ipfs_hash: String)`
Add a new participant to the system.

**Parameters:**
- `human_address`: Participant's address
- `ipfs_hash`: IPFS hash containing metadata (profile image, bio, etc.)

**Events:** `human_added`

**Errors:**
- `HumanAlreadyExists`: Participant already registered

**Example:**
```rust
client.add_human(
    &participant_addr,
    &String::from_str(&env, "QmX7fK8R...")
);
```

---

#### `update_human_validation(human_address: Address, validated: bool)`
Update validation status of a participant. Only admin can call.

**Parameters:**
- `human_address`: Participant's address
- `validated`: New validation status (true/false)

**Events:** `human_validation_updated`

**Errors:**
- `Unauthorized`: Caller is not admin
- `HumanNotFound`: Participant doesn't exist

**Example:**
```rust
client.update_human_validation(&participant_addr, &true);
```

---

#### `update_human_image(human_address: Address, ipfs_hash: String)`
Update IPFS metadata for a participant.

**Parameters:**
- `human_address`: Participant's address
- `ipfs_hash`: New IPFS hash

**Events:** `human_image_updated`

**Errors:**
- `HumanNotFound`: Participant doesn't exist

---

#### `get_human(human_address: Address) -> Human`
Get participant details.

**Parameters:**
- `human_address`: Participant's address

**Returns:** Human struct with all details

**Errors:**
- `HumanNotFound`: Participant doesn't exist

---

#### `get_all_humans(start_index: u32, limit: u32) -> Vec<Human>`
Get paginated list of all participants.

**Parameters:**
- `start_index`: Starting index (0-based)
- `limit`: Maximum number of results

**Returns:** Vector of Human structs

**Example:**
```rust
// Get first 10 participants
let page1 = client.get_all_humans(&0, &10);

// Get next 10 participants
let page2 = client.get_all_humans(&10, &10);
```

---

### Event Management

#### `create_event(event_id: String, location: String, pool: i128)`
Create a new event with funding pool.

**Parameters:**
- `event_id`: Unique event identifier
- `location`: Event location (address, city, venue, etc.)
- `pool`: Total funding pool amount (in stroops)

**Events:** `event_created`

**Errors:**
- `Unauthorized`: Caller is not admin
- `InvalidAmount`: Pool amount is zero or negative
- `EventAlreadyExists`: Event ID already used

**Example:**
```rust
client.create_event(
    &String::from_str(&env, "event_001"),
    &String::from_str(&env, "Buenos Aires"),
    &1000000000  // 100 XLM
);
```

---

#### `add_human_to_event(event_id: String, human_address: Address)`
Add a participant to an event.

**Parameters:**
- `event_id`: Event identifier
- `human_address`: Participant's address

**Events:** `human_added_to_event`

**Errors:**
- `Unauthorized`: Caller is not admin
- `EventNotFound`: Event doesn't exist
- `HumanNotFound`: Participant doesn't exist
- `HumanAlreadyInEvent`: Participant already in event

---

#### `get_event(event_id: String) -> Event`
Get event details including all participants.

**Parameters:**
- `event_id`: Event identifier

**Returns:** Event struct with all details

**Errors:**
- `EventNotFound`: Event doesn't exist

---

#### `get_event_validated_humans(event_id: String) -> Vec<Address>`
Get list of validated participants for an event.

**Parameters:**
- `event_id`: Event identifier

**Returns:** Vector of addresses (only validated participants)

**Errors:**
- `EventNotFound`: Event doesn't exist

**Example:**
```rust
let validated = client.get_event_validated_humans(
    &String::from_str(&env, "event_001")
);
```

---

### Distribution

#### `distribute_event_pool(event_id: String, token: Address)`
Distribute event pool equitably among validated participants only.

**Parameters:**
- `event_id`: Event identifier
- `token`: Token contract address (e.g., USDC, XLM)

**Events:** `distribution_completed`

**Process:**
1. Retrieves event and validates it exists
2. Filters only validated participants
3. Calculates amount per participant (pool / validated_count)
4. Transfers equal amounts to each validated participant

**Errors:**
- `Unauthorized`: Caller is not admin
- `EventNotFound`: Event doesn't exist
- `NoValidatedHumans`: No validated participants in event
- `ZeroAmountPerRecipient`: Pool too small (would result in 0 per person)
- `MathError`: Arithmetic overflow during calculations

**Example:**
```rust
// Distribute 100 XLM among 5 validated participants
// Each receives 20 XLM
client.distribute_event_pool(
    &String::from_str(&env, "event_001"),
    &token_address
);
```

---

## Error Codes

| Code | Error | Description |
|------|-------|-------------|
| 1 | `AlreadyInitialized` | Contract already initialized |
| 2 | `AdminNotFound` | Contract not initialized |
| 3 | `Unauthorized` | Caller is not admin |
| 4 | `HumanAlreadyExists` | Participant already registered |
| 5 | `HumanNotFound` | Participant doesn't exist |
| 6 | `EventAlreadyExists` | Event ID already used |
| 7 | `EventNotFound` | Event doesn't exist |
| 8 | `HumanAlreadyInEvent` | Participant already in event |
| 9 | `NoValidatedHumans` | No validated participants in event |
| 10 | `InvalidAmount` | Amount is zero or negative |
| 11 | `ZeroAmountPerRecipient` | Pool too small for distribution |
| 12 | `MathError` | Arithmetic overflow |

---

## Events

All operations emit events for auditability:

- `admin_set`: Admin initialized
- `human_added`: New participant added
- `human_validation_updated`: Validation status changed
- `human_image_updated`: IPFS metadata updated
- `event_created`: New event created
- `human_added_to_event`: Participant added to event
- `distribution_completed`: Pool distributed successfully

---

## Usage Example

```rust
use soroban_sdk::{Address, Env, String};

// Initialize contract
let admin = Address::generate(&env);
client.init(&admin);

// Add participants
client.add_human(&human1, &String::from_str(&env, "QmHash1"));
client.add_human(&human2, &String::from_str(&env, "QmHash2"));
client.add_human(&human3, &String::from_str(&env, "QmHash3"));

// Validate some participants
client.update_human_validation(&human1, &true);
client.update_human_validation(&human2, &true);

// Create event
client.create_event(
    &String::from_str(&env, "refi_meetup_001"),
    &String::from_str(&env, "Buenos Aires"),
    &1000000000  // 100 XLM
);

// Add participants to event
client.add_human_to_event(&String::from_str(&env, "refi_meetup_001"), &human1);
client.add_human_to_event(&String::from_str(&env, "refi_meetup_001"), &human2);
client.add_human_to_event(&String::from_str(&env, "refi_meetup_001"), &human3);

// Distribute pool to validated participants only (human1 and human2)
// Each receives 50 XLM (100 / 2)
client.distribute_event_pool(
    &String::from_str(&env, "refi_meetup_001"),
    &token_address
);
```

---

## Building & Testing

```bash
# Build optimized WASM
cargo build --target wasm32-unknown-unknown --release

# Run tests
cargo test

# Run specific test
cargo test test_distribute_success_multiple_recipients
```

---

## Deployment

See [DEPLOYMENT.md](../../docs/DEPLOYMENT.md) for testnet and mainnet deployment instructions.

---

## Security Considerations

1. **Admin Privileges**: Only admin can:
   - Update validation status
   - Create events
   - Add participants to events
   - Trigger distributions

2. **Validation**: Distribution only sends to validated participants, regardless of event participant list

3. **IPFS Storage**: Contract stores IPFS hashes, not actual images/data. Ensures low storage costs.

4. **Amount Validation**: Pool must be large enough to distribute non-zero amounts to all validated participants.

---

## License

MIT License - See [LICENSE](../../LICENSE)

---

## Related Contracts

- [Vault Distributor](../vault-distributor/README.md) - Simple parameter-based distribution contract
