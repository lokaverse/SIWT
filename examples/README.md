# SIWT Examples

This directory contains practical examples demonstrating how to integrate SIWT (Sign In With Telegram) into your Internet Computer applications.

## 📁 Examples Overview

### 🌐 Modern Web Integration
- **[nextjs-15/](./nextjs-15/)** - Next.js 15 with App Router and JavaScript
- **[react-19-vite/](./react-19-vite/)** - React 19 with Vite and modern hooks
- **[sveltekit/](./sveltekit/)** - SvelteKit with server-side rendering
- **[astro/](./astro/)** - Astro with islands architecture

### 🖥️ Backend Integration (Standards)
- **[nodejs-javascript/](./nodejs-javascript/)** - Node.js with modern JavaScript and Zod validation
- **[nextjs-api-routes/](./nextjs-api-routes/)** - Next.js API routes with edge runtime
- **[rust-axum/](./rust-axum/)** - Modern async Rust backend
- **[python-fastapi/](./python-fastapi/)** - FastAPI with Pydantic v2

### 📱 Advanced Examples
- **[telegram-mini-apps/](./telegram-mini-apps/)** - WebApp integration with initData validation
- **[telegram-bot/](./telegram-bot/)** - Telegram bot with SIWT authentication
- **[internet-identity-dual/](./internet-identity-dual/)** - Dual authentication with II + Telegram
- **[mobile-pwa/](./mobile-pwa/)** - Progressive Web App with offline capabilities
- **[multi-canister-2025/](./multi-canister-2025/)** - Scalable IC architecture with delegation chains

## 🚀 Quick Start

Each example includes:
- **README.md** - Setup and usage instructions
- **JavaScript source** - Modern ES6+ implementation with JSDoc types
- **Environment config** - Secure setup with validation
- **Test suites** - Vitest/Jest testing
- **Docker setup** - Containerized deployment

## 🔧 Prerequisites

Before running any example:

1. **Install dependencies**
   ```bash
   # Enable corepack for pnpm
   corepack enable
   # Install with pnpm (preferred)
   pnpm install
   ```

2. **Start local IC replica**
   ```bash
   dfx start --background --clean
   ```

3. **Deploy SIWT canister**
   ```bash
   dfx deploy --network local
   dfx generate
   ```

4. **Configure Telegram Bot**
   - Create a bot with [@BotFather](https://t.me/botfather)
   - Enable WebApp features if needed
   - Set up environment variables with validation
   - Configure webhook for production

## 📖 Example Categories

### Beginner Examples
- **basic-web/** - Start here for simple integration
- **vanilla-js/** - Pure JavaScript without frameworks

### Intermediate Examples
- **react-app/** - Modern React application
- **node-backend/** - Server-side integration

### Advanced Examples
- **telegram-bot/** - Full bot implementation
- **multi-canister/** - Complex application architecture

## 🛠️ Common Patterns

### Modern Authentication Flow with Validation
```javascript
import { z } from 'zod';
import { SIWT } from '@siwt/core';

/**
 * @typedef {Object} TelegramAuthData
 * @property {number} id - Telegram user ID
 * @property {string} first_name - User's first name
 * @property {string} [username] - Optional username
 * @property {string} [photo_url] - Optional profile photo URL
 * @property {number} auth_date - Authentication timestamp
 * @property {string} hash - Telegram auth hash
 */

const TelegramAuthSchema = z.object({
  id: z.number(),
  first_name: z.string(),
  username: z.string().optional(),
  photo_url: z.string().url().optional(),
  auth_date: z.number(),
  hash: z.string()
});

// 1. Prepare delegation with validation
const prepared = await siwt.prepare({
  user: TelegramAuthSchema.parse(telegramData),
  session: sessionData,
  canisters: [targetCanisterId],
  maxTimeToLive: BigInt(8 * 60 * 60 * 1000 * 1000 * 1000) // 8 hours
});

// 2. Login with secure hash verification
const login = await siwt.login({
  hash: prepared.hash,
  validateInitData: true
});

// 3. Get signed delegation with proper expiration
const delegation = await siwt.delegation({
  user: telegramUserId,
  session: sessionData,
  expiration: login.expiration,
  canisters: [targetCanisterId]
});
```

### Modern Error Handling with Result Pattern
```javascript
import { Result } from '@badrap/result';

/**
 * @typedef {Object} UserData
 * @property {string} id - User identifier
 * @property {string} name - User display name
 */

/**
 * @typedef {Object} AuthError
 * @property {string} code - Error code
 * @property {string} message - Error message
 * @property {Array} [details] - Optional error details
 */

/**
 * Authenticate user with proper error handling
 * @param {unknown} data - Raw authentication data
 * @returns {Promise<Result<UserData, AuthError>>} Authentication result
 */
const authenticate = async (data) => {
  try {
    const validated = TelegramAuthSchema.parse(data);
    const result = await siwt.authenticate(validated);
    return Result.ok(result);
  } catch (error) {
    if (error instanceof z.ZodError) {
      return Result.err({ 
        code: 'INVALID_DATA', 
        details: error.errors,
        message: 'Validation failed'
      });
    }
    return Result.err({ 
      code: 'AUTH_FAILED', 
      message: error.message 
    });
  }
};
```

## 🔒 Security Notes

⚠️ **Important**: SIWT lacks a message signing step for Telegram ID ownership verification. This makes it less secure than SIWB (Sign In With Bitcoin). Users should be aware that Telegram ID ownership cannot be cryptographically verified.

### Best Practices
- Always validate user sessions
- Use HTTPS in production
- Set appropriate delegation expiration times
- Implement proper error handling
- Never expose sensitive tokens client-side

## 🤝 Contributing

Want to add an example? Please:

1. Create a new directory with a descriptive name
2. Include a comprehensive README.md
3. Add proper error handling and comments
4. Test thoroughly before submitting
5. Follow the existing code style

## 📞 Support

If you have questions about any example:

- Check the example's README.md
- Review the [main documentation](../docs/)
- Open an issue with the `question` label
- Join our community discussions

---