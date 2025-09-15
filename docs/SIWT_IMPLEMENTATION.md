# Sign-In With Telegram (SIWT) Implementation

## Overview

SIWT (Sign-In With Telegram) is an authentication system that allows users to authenticate using their Telegram identity. This implementation provides a secure way to create delegations for Internet Computer (IC) applications using Telegram user credentials.

## Architecture

### Core Components

1. **Authentication Flow**: Users authenticate using their Telegram credentials
2. **Delegation Management**: Creates and manages IC delegations for authenticated users
3. **Session Management**: Handles user sessions with configurable expiration times
4. **Account Management**: Maps Telegram users to IC principals

### Key Modules

- `lib.rs`: Main canister interface with public endpoints
- `delegation.rs`: Handles delegation creation and management
- `accounts.rs`: User account storage and retrieval
- `setting.rs`: Canister configuration and settings
- `messages.rs`: Message handling and validation
- `signatures.rs`: Cryptographic signature management

## Security Model

### Authentication Process

1. User provides Telegram credentials (user ID, session data)
2. System validates the Telegram authentication
3. Creates a delegation with specified expiration time
4. Returns signed delegation for use in IC applications

### Security Features

- **Expiration-based Sessions**: All delegations have configurable expiration times
- **Principal-based Authorization**: Only authorized principals can modify settings
- **Cryptographic Signatures**: All delegations are cryptographically signed
- **Memory-safe Storage**: Uses stable memory for persistent data

## API Endpoints

### Query Methods

- `authorized(principal)`: Check if a principal is authorized
- `features()`: Get available canister features
- `setting()`: Get current canister settings
- `account_derived_address(payload)`: Get derived address for user
- `account_derived_btc_address(payload)`: Get derived Bitcoin address
- `prepare_delegation(payload)`: Prepare delegation for user
- `get_delegation(payload)`: Get signed delegation

### Update Methods

- `init(setting)`: Initialize canister with settings
- `extends(payload)`: Extend canister settings
- `setExpirationMinute(minute)`: Set delegation expiration time

## Configuration

### Settings Structure

```rust
pub struct Setting {
    expiration_minute: u64,        // Delegation expiration in minutes
    authorities: Set<Principal>,   // Authorized principals
    canisters: Set<Principal>,     // Authorized canisters
}
```

### Default Configuration

- **Expiration Time**: 120 minutes (2 hours)
- **Authorities**: Includes the caller principal
- **Canisters**: Includes the canister principal

## Usage Examples

### Basic Authentication Flow

1. **Prepare Delegation**:
   ```
   prepare_delegation({
     user: "telegram_user_id",
     session: session_data,
     targets: [target_canister_principal]
   })
   ```

2. **Get Signed Delegation**:
   ```
   get_delegation({
     user: "telegram_user_id",
     session: session_data,
     targets: [target_canister_principal]
   })
   ```

### Administrative Operations

1. **Extend Settings**:
   ```
   extends({
     authorities: [new_authority_principal],
     canisters: [new_canister_principal]
   })
   ```

2. **Set Expiration Time**:
   ```
   setExpirationMinute(60)  // 1 hour expiration
   ```

## Deployment

### Prerequisites

- Rust toolchain with `wasm32-unknown-unknown` target
- DFX (Internet Computer SDK)
- Candid extractor tool

### Build Process

```bash
# Build the canister
cargo build --target wasm32-unknown-unknown --release -p backend

# Extract Candid interface
candid-extractor target/wasm32-unknown-unknown/release/backend.wasm > src/backend/backend.did
```

### Deployment Commands

```bash
# Deploy to local network
dfx deploy --network local

# Deploy to IC mainnet
dfx deploy --network ic
```