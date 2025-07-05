# Walrus Signer Guide

## Overview

The **signer** in the Walrus SDK is a component that signs transactions on the **Sui blockchain** to:

1. **Register the blob**
2. **Pay for storage**
3. **Pay the write fee**
4. **Prove the data was uploaded**

## 🔐 Why a Signer is Needed

Walrus is built on top of **Sui**, so writing a blob involves actual blockchain transactions. These transactions need to be:

- Authenticated (signed with your private key)
- Paid for (with **SUI** as gas, and **WAL** as storage token)

That's why you need a **signer** that:

- Has access to a valid private key
- Has sufficient token balances

---

## 🧪 Test Mode: `Ed25519Keypair.generate()`

This creates a random keypair in memory:

```ts
import { Ed25519Keypair } from "@mysten/sui/keypairs/ed25519";

const keypair = Ed25519Keypair.generate();
```

Useful for demos or testing, but:

- The key is not persistent
- The address is new and has no tokens
- You'd need to manually fund it (e.g., via Sui testnet faucet)

---

## 🛡 Production Mode: Real Wallet Integration

In production, users should sign using their actual wallet:

- [Sui Wallet (official)](https://wallet.sui.io/)
- [Suiet](https://suiet.app/)
- [Surf Wallet](https://surfwallet.io/)
- or any other Sui-compatible wallet

These wallets expose a signer object that can be used like this:

```ts
const signer = await wallet.getSigner(); // pseudocode
await walrusClient.writeBlob({ blob, signer, ... });
```

---

## 💸 Required Tokens

To write to Walrus:

- **SUI**: to pay gas fees for Sui transactions
- **WAL**: the native token to pay for storage (per blob, per epoch)

You'll need to **airdrop or buy WAL** (on testnet it may be faucetable) and ensure your wallet address holds both.

---

## 🧾 Summary

| Requirement    | Why?                                    |
| -------------- | --------------------------------------- |
| **Signer**     | Signs Sui transactions                  |
| **SUI tokens** | Pays for gas (blockchain fees)          |
| **WAL tokens** | Pays to store the file in Walrus        |
| **Wallet**     | Should be user-controlled in production |

---

## Implementation Example

### Current Test Implementation

```ts
// In FileUpload.tsx
const keypair = Ed25519Keypair.generate(); // Replace with real wallet integration
const { blobId } = await walrusClient.writeBlob({
  blob,
  deletable: false,
  epochs: 3,
  signer: keypair,
});
```

### Production Wallet Integration

```ts
// Connect to user's wallet
const connectWallet = async () => {
  if ("suiWallet" in window) {
    const wallet = window.suiWallet;
    await wallet.requestAccounts();
    return wallet;
  }
  throw new Error("Sui wallet not found");
};

// Use wallet signer
const uploadToWalrus = async (blob: Uint8Array) => {
  const wallet = await connectWallet();
  const signer = await wallet.getSigner();

  const { blobId } = await walrusClient.writeBlob({
    blob,
    deletable: false,
    epochs: 3,
    signer,
  });

  return blobId;
};
```

---

## Getting Testnet Tokens

### SUI Tokens

- Visit [Sui Testnet Faucet](https://suiexplorer.com/faucet)
- Enter your wallet address
- Receive test SUI tokens

### WAL Tokens

- Check if Walrus testnet has a faucet
- Or purchase WAL tokens on testnet DEX
- Ensure your wallet has sufficient balance for storage costs

---

## Error Handling

Common errors when using Walrus:

```ts
try {
  const { blobId } = await walrusClient.writeBlob({
    blob,
    deletable: false,
    epochs: 3,
    signer: keypair,
  });
} catch (error) {
  if (error.message.includes("insufficient balance")) {
    console.error("Need more SUI or WAL tokens");
  } else if (error.message.includes("signature")) {
    console.error("Signer authentication failed");
  }
}
```

---

## Next Steps

For production deployment:

1. **Integrate a real Sui wallet** (Sui Wallet, Suiet, etc.)
2. **Add wallet connection UI** (connect button, account display)
3. **Handle token balances** (check SUI/WAL before upload)
4. **Add error handling** for insufficient funds, network issues
5. **Consider using a fan-out proxy** for better performance

Let me know if you want help integrating a Sui wallet (e.g., connect button + signer extraction).

---

## TODO: Production Tasks

### 🔧 Wallet Integration

- [ ] Replace `Ed25519Keypair.generate()` with real wallet signer
- [ ] Add wallet connection button/UI
- [ ] Handle wallet connection errors
- [ ] Add wallet account display
- [ ] Implement wallet disconnection

### 💰 Token Management

- [ ] Check SUI balance before upload
- [ ] Check WAL balance before upload
- [ ] Show token balance in UI
- [ ] Add token purchase/faucet links
- [ ] Handle insufficient balance errors gracefully

### 🚀 Performance & UX

- [ ] Add loading states for wallet operations
- [ ] Implement retry logic for failed uploads
- [ ] Add progress indicators for large files
- [ ] Consider using fan-out proxy for better performance
- [ ] Add file size limits and validation

### 🛡️ Security & Error Handling

- [ ] Validate file types and sizes
- [ ] Add proper error messages for all failure cases
- [ ] Implement rate limiting
- [ ] Add transaction confirmation dialogs
- [ ] Handle network timeouts and retries

### 📱 UI/UX Improvements

- [ ] Add drag-and-drop file upload
- [ ] Show upload history
- [ ] Add file preview for supported types
- [ ] Implement file deletion (if deletable)
- [ ] Add upload queue for multiple files

### 🔍 Testing

- [ ] Test with real Sui wallets
- [ ] Test token balance scenarios
- [ ] Test network failure recovery
- [ ] Test large file uploads
- [ ] Test concurrent uploads

### 📊 Monitoring

- [ ] Add upload success/failure tracking
- [ ] Monitor gas costs and storage fees
- [ ] Track user upload patterns
- [ ] Add performance metrics
