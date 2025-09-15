# SIWT API Reference

This document provides a comprehensive reference for all SIWT (Sign In With Telegram) canister methods, including detailed examples and usage patterns.

## Table of Contents

- [Authentication Methods](#authentication-methods)
- [Account Management](#account-management)
- [Global State Management](#global-state-management)
- [Configuration Methods](#configuration-methods)
- [Utility Methods](#utility-methods)
- [Data Types](#data-types)
- [Error Handling](#error-handling)

## Authentication Methods

### `prepare(PreparePayload) -> PrepareResponse`

**Type**: Update Method  
**Description**: Prepares a delegation for a Telegram user by validating their session and generating a hash for signing.

#### Parameters

```candid
type PreparePayload = record {
  user : text;              // Telegram user ID
  session : blob;           // Session data from Telegram WebApp
  canisters : vec principal; // Target canisters for delegation
};
```

#### Response

```candid
type PrepareResponse = variant {
  Ok : Prepared;
  Err : text;
};

type Prepared = record {
  expired : text;     // Human-readable expiration time
  hash : blob;        // Hash to be signed by user
  expiration : nat64; // Unix timestamp expiration
  message : text;     // Message to display to user
};
```

#### Example Usage

```javascript
// JavaScript example
const prepareResult = await actor.prepare({
  user: "123456789",
  session: new Uint8Array([/* session data */]),
  canisters: [Principal.fromText("rdmx6-jaaaa-aaaah-qcaiq-cai")]
});

if ('Ok' in prepareResult) {
  const { hash, expiration, message } = prepareResult.Ok;
  console.log(`Please sign this hash: ${Array.from(hash).map(b => b.toString(16).padStart(2, '0')).join('')}`);
  console.log(`Expires: ${new Date(Number(expiration) / 1000000).toISOString()}`);
}
```

### `login(LoginPayload) -> LoginResponse`

**Type**: Update Method  
**Description**: Completes the authentication process by verifying the signed hash and returning delegation details.

#### Parameters

```candid
type LoginPayload = record {
  hash : blob; // Signed hash from prepare step
};
```

#### Response

```candid
type LoginResponse = variant {
  Ok : Login;
  Err : text;
};

type Login = record {
  expired : text;           // Human-readable expiration
  hash : blob;              // Original hash
  expiration : nat64;       // Unix timestamp expiration
  canisters : vec principal; // Authorized canisters
};
```

#### Example Usage

```javascript
// After user signs the hash from prepare step
const signedHash = new Uint8Array([/* user's signature */]);

const loginResult = await actor.login({
  hash: signedHash
});

if ('Ok' in loginResult) {
  const { expiration, canisters } = loginResult.Ok;
  console.log(`Authenticated until: ${new Date(Number(expiration) / 1000000).toISOString()}`);
  console.log(`Authorized canisters: ${canisters.map(p => p.toString())}`);
}
```

### `delegation(DelegationPayload) -> SignedDelegationResponse`

**Type**: Query Method  
**Description**: Retrieves a signed delegation for an authenticated user.

#### Parameters

```candid
type DelegationPayload = record {
  user : text;              // Telegram user ID
  expiration : nat64;       // Requested expiration time
  session : blob;           // Session data
  canisters : vec principal; // Target canisters
};
```

#### Response

```candid
type SignedDelegationResponse = variant {
  Ok : SignedDelegation;
  Err : text;
};

type SignedDelegation = record {
  signature : blob;   // Delegation signature
  delegation : Delegated; // Delegation details
  pubkey : blob;      // Public key
};

type Delegated = record {
  pubkey : blob;            // Public key
  targets : vec principal;  // Authorized canisters
  expiration : nat64;       // Expiration timestamp
};
```

#### Example Usage

```javascript
const delegationResult = await actor.delegation({
  user: "123456789",
  expiration: BigInt(Date.now() + 3600000) * 1000000n, // 1 hour from now
  session: sessionData,
  canisters: [targetCanisterId]
});

if ('Ok' in delegationResult) {
  const { signature, delegation, pubkey } = delegationResult.Ok;
  // Use delegation to authenticate with target canisters
}
```

## Account Management

### `accountDerivedAddress(AccountDerivedAddressPayload) -> AccountDerivedAddressResponse`

**Type**: Update Method  
**Description**: Derives blockchain addresses for a Telegram user account.

#### Parameters

```candid
type AccountDerivedAddressPayload = record {
  ckbtc : AccountCkBtcPayload;
  user : text; // Telegram user ID
};

type AccountCkBtcPayload = record {
  owners : vec principal; // ckBTC account owners
};
```

#### Response

```candid
type AccountDerivedAddressResponse = variant {
  Ok : AccountDerivedAddress;
  Err : text;
};

type AccountDerivedAddress = record {
  btc : AccountDerivedBtcAddress;
  "principal" : principal;
  pubkey : blob;
};

type AccountDerivedBtcAddress = record {
  accounts : vec record { principal; opt text };
  address : text; // Bitcoin address
};
```

#### Example Usage

```javascript
const addressResult = await actor.accountDerivedAddress({
  user: "123456789",
  ckbtc: {
    owners: [Principal.fromText("rdmx6-jaaaa-aaaah-qcaiq-cai")]
  }
});

if ('Ok' in addressResult) {
  const { btc, principal, pubkey } = addressResult.Ok;
  console.log(`Bitcoin address: ${btc.address}`);
  console.log(`IC Principal: ${principal.toString()}`);
}
```

### `user(principal) -> opt text`

**Type**: Query Method  
**Description**: Retrieves the Telegram user ID associated with an IC principal.

#### Example Usage

```javascript
const userId = await actor.user(Principal.fromText("rdmx6-jaaaa-aaaah-qcaiq-cai"));
if (userId.length > 0) {
  console.log(`Telegram User ID: ${userId[0]}`);
}
```

### `principal(text) -> opt principal`

**Type**: Query Method  
**Description**: Retrieves the IC principal associated with a Telegram user ID.

#### Example Usage

```javascript
const principal = await actor.principal("123456789");
if (principal.length > 0) {
  console.log(`IC Principal: ${principal[0].toString()}`);
}
```

### `all() -> vec record { text; principal }`

**Type**: Query Method  
**Description**: Returns all user mappings (Telegram ID to IC Principal).

#### Example Usage

```javascript
const allUsers = await actor.all();
console.log(`Total users: ${allUsers.length}`);
allUsers.forEach(([telegramId, principal]) => {
  console.log(`${telegramId} -> ${principal.toString()}`);
});
```

## Global State Management

### `globalsStore(text, blob) -> ()`

**Type**: Update Method  
**Description**: Stores a key-value pair in the global state.

#### Example Usage

```javascript
const key = "user_preferences";
const value = new TextEncoder().encode(JSON.stringify({ theme: "dark" }));
await actor.globalsStore(key, value);
```

### `globalsGet(text) -> opt blob`

**Type**: Query Method  
**Description**: Retrieves a value from global state by key.

#### Example Usage

```javascript
const value = await actor.globalsGet("user_preferences");
if (value.length > 0) {
  const decoded = JSON.parse(new TextDecoder().decode(value[0]));
  console.log(`User preferences:`, decoded);
}
```

### `globalsHas(text) -> bool`

**Type**: Query Method  
**Description**: Checks if a key exists in global state.

#### Example Usage

```javascript
const exists = await actor.globalsHas("user_preferences");
console.log(`Preferences exist: ${exists}`);
```

### `globalsRemove(text) -> opt blob`

**Type**: Update Method  
**Description**: Removes a key-value pair from global state and returns the removed value.

#### Example Usage

```javascript
const removed = await actor.globalsRemove("old_data");
if (removed.length > 0) {
  console.log(`Removed value:`, new TextDecoder().decode(removed[0]));
}
```

### `globalsKeys() -> vec text`

**Type**: Query Method  
**Description**: Returns all keys in global state.

#### Example Usage

```javascript
const keys = await actor.globalsKeys();
console.log(`Available keys:`, keys);
```

### `globals() -> vec record { text; blob }`

**Type**: Query Method  
**Description**: Returns all key-value pairs in global state.

#### Example Usage

```javascript
const allData = await actor.globals();
allData.forEach(([key, value]) => {
  console.log(`${key}: ${new TextDecoder().decode(value)}`);
});
```

### Batch Operations

#### `globalsStores(vec record { text; blob }) -> vec record { text; blob }`

**Type**: Update Method  
**Description**: Stores multiple key-value pairs in a single call.

```javascript
const data = [
  ["key1", new TextEncoder().encode("value1")],
  ["key2", new TextEncoder().encode("value2")]
];
const stored = await actor.globalsStores(data);
```

#### `globalsRemoves(vec text) -> vec record { text; blob }`

**Type**: Update Method  
**Description**: Removes multiple keys in a single call.

```javascript
const keysToRemove = ["old_key1", "old_key2"];
const removed = await actor.globalsRemoves(keysToRemove);
```

#### `globalsIn(vec text) -> vec record { text; blob }`

**Type**: Query Method  
**Description**: Retrieves multiple values by their keys.

```javascript
const keys = ["user_prefs", "app_config"];
const values = await actor.globalsIn(keys);
```

#### `globalsContains(vec text) -> bool`

**Type**: Query Method  
**Description**: Checks if all specified keys exist in global state.

```javascript
const requiredKeys = ["config", "settings"];
const allExist = await actor.globalsContains(requiredKeys);
```

## Configuration Methods

### `setting() -> Setting`

**Type**: Query Method  
**Description**: Returns the current canister configuration.

#### Response

```candid
type Setting = record {
  expiration_minute : nat64;    // Delegation expiration in minutes
  canisters : vec principal;    // Authorized canisters
  authorities : vec principal;  // Authorized administrators
};
```

#### Example Usage

```javascript
const config = await actor.setting();
console.log(`Expiration: ${config.expiration_minute} minutes`);
console.log(`Authorized canisters: ${config.canisters.length}`);
console.log(`Authorities: ${config.authorities.length}`);
```

### `setExpirationMinute(nat64) -> ()`

**Type**: Update Method  
**Description**: Sets the delegation expiration time in minutes. Only callable by authorities.

#### Example Usage

```javascript
// Set expiration to 2 hours (120 minutes)
await actor.setExpirationMinute(120n);
```

### `extends(SettingExtendsPayload) -> ()`

**Type**: Update Method  
**Description**: Extends the canister configuration with additional authorities and canisters.

#### Parameters

```candid
type SettingExtendsPayload = record {
  canisters : vec principal;   // Additional authorized canisters
  authorities : vec principal; // Additional authorities
};
```

#### Example Usage

```javascript
await actor.extends({
  canisters: [Principal.fromText("new-canister-id")],
  authorities: [Principal.fromText("new-authority-id")]
});
```

## Utility Methods

### `features() -> vec record { text; bool }`

**Type**: Query Method  
**Description**: Returns available features and their enabled status.

#### Example Usage

```javascript
const features = await actor.features();
features.forEach(([feature, enabled]) => {
  console.log(`${feature}: ${enabled ? 'enabled' : 'disabled'}`);
});

// Check for specific feature
const ckbtcEnabled = features.find(([name]) => name === 'ckbtc')?.[1] || false;
```

### `caller() -> (principal, opt text)`

**Type**: Query Method  
**Description**: Returns the caller's principal and associated Telegram user ID (if any).

#### Example Usage

```javascript
const [callerPrincipal, telegramId] = await actor.caller();
console.log(`Caller: ${callerPrincipal.toString()}`);
if (telegramId.length > 0) {
  console.log(`Telegram ID: ${telegramId[0]}`);
}
```

## Data Types

### Core Types

```candid
// Authentication types
type PreparePayload = record {
  user : text;
  session : blob;
  canisters : vec principal;
};

type LoginPayload = record {
  hash : blob;
};

type DelegationPayload = record {
  user : text;
  expiration : nat64;
  session : blob;
  canisters : vec principal;
};

// Response types
type PrepareResponse = variant { Ok : Prepared; Err : text };
type LoginResponse = variant { Ok : Login; Err : text };
type SignedDelegationResponse = variant { Ok : SignedDelegation; Err : text };

// Account types
type AccountDerivedAddressPayload = record {
  ckbtc : AccountCkBtcPayload;
  user : text;
};

type AccountDerivedAddressResponse = variant {
  Ok : AccountDerivedAddress;
  Err : text;
};

// Configuration types
type Setting = record {
  expiration_minute : nat64;
  canisters : vec principal;
  authorities : vec principal;
};

type SettingExtendsPayload = record {
  canisters : vec principal;
  authorities : vec principal;
};
```

## Error Handling

### Common Error Patterns

```javascript
// Generic error handling pattern
try {
  const result = await actor.someMethod(payload);
  
  if ('Ok' in result) {
    // Success case
    const data = result.Ok;
    // Process data
  } else {
    // Error case
    console.error('Operation failed:', result.Err);
    // Handle error
  }
} catch (error) {
  console.error('Network or canister error:', error);
}
```

### Common Error Messages

- `"Unauthorized"` - Caller lacks required permissions
- `"Invalid session"` - Session data is malformed or expired
- `"User not found"` - Telegram user ID not registered
- `"Invalid hash"` - Signature verification failed
- `"Expired"` - Delegation or session has expired
- `"Invalid canister"` - Target canister not authorized

### Best Practices

1. **Always check response variants** before accessing data
2. **Handle network errors** with try-catch blocks
3. **Validate inputs** before making canister calls
4. **Cache delegation results** to avoid unnecessary calls
5. **Monitor expiration times** and refresh delegations proactively
6. **Use batch operations** for multiple global state operations
7. **Implement retry logic** for transient failures

### Rate Limiting

The canister implements internal rate limiting. If you encounter rate limit errors:

- Implement exponential backoff
- Batch operations when possible
- Cache results to reduce call frequency
- Use query methods when data doesn't need to be fresh

### Security Considerations

1. **Session Data**: Never log or expose session data in client-side code
2. **Private Keys**: Keep signing keys secure and never transmit them
3. **Delegation Scope**: Always specify the minimum required canisters
4. **Expiration Times**: Use reasonable expiration times (not too long)
5. **Error Messages**: Don't expose sensitive information in error handling

## Integration Examples

### React Hook Example

```javascript
import { useState, useEffect } from 'react';
import { Actor, HttpAgent } from '@dfinity/agent';
import { idlFactory } from './declarations/backend';

function useSIWT(canisterId) {
  const [actor, setActor] = useState(null);
  const [user, setUser] = useState(null);
  const [loading, setLoading] = useState(false);

  useEffect(() => {
    const agent = new HttpAgent({ host: 'https://ic0.app' });
    const actorInstance = Actor.createActor(idlFactory, {
      agent,
      canisterId,
    });
    setActor(actorInstance);
  }, [canisterId]);

  const authenticate = async (telegramData) => {
    if (!actor) return;
    
    setLoading(true);
    try {
      // Step 1: Prepare delegation
      const prepareResult = await actor.prepare({
        user: telegramData.user.id.toString(),
        session: new TextEncoder().encode(telegramData.initData),
        canisters: [canisterId]
      });

      if ('Err' in prepareResult) {
        throw new Error(prepareResult.Err);
      }

      const { hash } = prepareResult.Ok;
      
      // Step 2: Sign hash (implement your signing logic)
      const signedHash = await signHash(hash);
      
      // Step 3: Complete login
      const loginResult = await actor.login({ hash: signedHash });
      
      if ('Err' in loginResult) {
        throw new Error(loginResult.Err);
      }

      setUser(telegramData.user);
      return loginResult.Ok;
    } catch (error) {
      console.error('Authentication failed:', error);
      throw error;
    } finally {
      setLoading(false);
    }
  };

  return { actor, user, loading, authenticate };
}
```

### Node.js Backend Example

```javascript
const { Actor, HttpAgent } = require('@dfinity/agent');
const { idlFactory } = require('./declarations/backend');

class SIWTService {
  constructor(canisterId, host = 'https://ic0.app') {
    this.agent = new HttpAgent({ host });
    this.actor = Actor.createActor(idlFactory, {
      agent: this.agent,
      canisterId,
    });
  }

  async validateUser(telegramUserId) {
    try {
      const principal = await this.actor.principal(telegramUserId);
      return principal.length > 0 ? principal[0] : null;
    } catch (error) {
      console.error('User validation failed:', error);
      return null;
    }
  }

  async getUserPreferences(key) {
    try {
      const value = await this.actor.globalsGet(key);
      if (value.length > 0) {
        return JSON.parse(new TextDecoder().decode(value[0]));
      }
      return null;
    } catch (error) {
      console.error('Failed to get preferences:', error);
      return null;
    }
  }

  async setUserPreferences(key, preferences) {
    try {
      const value = new TextEncoder().encode(JSON.stringify(preferences));
      await this.actor.globalsStore(key, value);
      return true;
    } catch (error) {
      console.error('Failed to set preferences:', error);
      return false;
    }
  }
}

module.exports = SIWTService;
```