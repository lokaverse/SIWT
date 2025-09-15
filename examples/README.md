## Quick start guide with copyable deployment code

### Backend Deployment

```bash
# Install dependencies and build
npm install

# Deploy to local replica for development
dfx start --background
dfx deploy backend

# Deploy to IC mainnet
dfx deploy --network ic backend

# Get canister ID and interface
dfx canister id backend --network ic
dfx generate backend  # Generate TypeScript declarations

# IMPORTANT: Set up authorization (required for all SIWT methods)
# Replace YOUR_PRINCIPAL with your actual principal ID
dfx canister call backend init '(record {
  authorized = vec { principal "YOUR_PRINCIPAL" };
  expiration = 28800000000000; // 8 hours in nanoseconds
})'

# Test the deployed canister (must be called by authorized principal)
dfx canister call backend prepare '(record { 
  user = "telegram_user_id"; 
  session = vec { /* session public key bytes */ }; 
  canisters = vec { principal "rdmx6-jaaaa-aaaah-qcaiq-cai" } 
})' --identity your-authorized-identity
```

### Authorization Setup

⚠️ **Critical**: All SIWT methods require authorization. You must:

1. **Initialize with authorized principals** during deployment
2. **Use an authorized identity** when calling methods
3. **Configure your frontend** to use an authorized agent

```bash
# Get your principal ID
dfx identity get-principal

# Initialize canister with your principal as authorized
dfx canister call backend init '(record {
  authorized = vec { principal "YOUR_PRINCIPAL_HERE" };
  expiration = 28800000000000;
})'
```

## Example Integration

```javascript
// Frontend integration with actual SIWT backend
import { createActor, canisterId } from './declarations/backend';
import { HttpAgent } from '@dfinity/agent';
import { Ed25519KeyIdentity } from '@dfinity/identity';
import { Principal } from '@dfinity/principal';

/**
 * Complete SIWT authentication flow with proper error handling
 * @param {string} telegramUserId - Telegram user ID as string
 * @param {Array<string>} targetCanisters - Target canister IDs for delegation
 * @param {Identity} authorizedIdentity - Identity that's authorized in the canister
 * @returns {Promise<Object>} Authentication result with delegation
 */
async function authenticateWithSIWT(telegramUserId, targetCanisters = [], authorizedIdentity) {
  // 1. Generate session identity
  const sessionIdentity = Ed25519KeyIdentity.generate();
  const sessionPublicKey = sessionIdentity.getPublicKey().toDer();
  
  // 2. Create actor with authorized identity
  const agent = new HttpAgent({ 
    host: process.env.NODE_ENV === 'production' ? 'https://ic0.app' : 'http://localhost:4943',
    identity: authorizedIdentity // REQUIRED: Must be authorized principal
  });
  
  // Fetch root key for local development
  if (process.env.NODE_ENV !== 'production') {
    await agent.fetchRootKey();
  }
  
  const actor = createActor(canisterId, { agent });
  
  try {
    // 3. Prepare authentication message
    const prepareResponse = await actor.prepare({
      user: telegramUserId,
      session: Array.from(sessionPublicKey),
      canisters: targetCanisters.map(id => Principal.fromText(id))
    });
    
    // Handle PrepareResponse enum
    if ('Err' in prepareResponse) {
      throw new Error(`Prepare failed: ${prepareResponse.Err}`);
    }
    
    const prepared = prepareResponse.Ok;
    console.log('Prepared message:', prepared.message);
    console.log('Expiration:', prepared.expired);
    
    // 4. Login with the prepared hash
    const loginResponse = await actor.login({
      hash: prepared.hash
    });
    
    // Handle LoginResponse enum
    if ('Err' in loginResponse) {
      throw new Error(`Login failed: ${loginResponse.Err}`);
    }
    
    const login = loginResponse.Ok;
    
    // 5. Get signed delegation (query call - no certificate needed)
    const delegationResponse = await actor.delegation({
      user: telegramUserId,
      session: Array.from(sessionPublicKey),
      expiration: login.expiration,
      canisters: Array.from(login.canisters) // Convert Set to Array
    });
    
    // Handle SignedDelegationResponse enum
    if ('Err' in delegationResponse) {
      throw new Error(`Delegation failed: ${delegationResponse.Err}`);
    }
    
    return {
      sessionIdentity,
      delegation: delegationResponse.Ok,
      expiration: login.expiration,
      canisters: Array.from(login.canisters)
    };
    
  } catch (error) {
    console.error('SIWT Authentication failed:', error);
    throw error;
  }
}

// Usage example with proper authorization
import { Ed25519KeyIdentity } from '@dfinity/identity';

// Load or create an authorized identity
const authorizedIdentity = Ed25519KeyIdentity.fromSecretKey(/* your authorized key */);

const authResult = await authenticateWithSIWT(
  '123456789', 
  ['rdmx6-jaaaa-aaaah-qcaiq-cai'], 
  authorizedIdentity
);
console.log('Authentication successful:', authResult);