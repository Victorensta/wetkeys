<img width="892" alt="Screenshot 2025-07-06 at 07 00 42" src="https://github.com/user-attachments/assets/a773ff74-2cfe-4e9f-883d-4c3f1bd2cf60" />

#WETKEYS - Web3 Transfers, Safe and Simple

## Overview

WETKEYS was developed during the ETH Global hackathon to solve a critical problem in Web3: while blockchain technology empowers users with ownership and transparency, it also exposes raw content to anyone with a link. This is a critical problem

Our solution leverages cutting-edge **Identity-Based Encryption (IBE)** technology to enable secure, private Web3 transfers while maintaining the benefits of decentralized infrastructure.

## The Problem
<img width="937" alt="Screenshot 2025-07-06 at 07 00 58" src="https://github.com/user-attachments/assets/15112b7e-6a6c-4947-9b19-3b3876561467" />

Web3 data security presents unique challenges:
- **Transparency vs Privacy**: Public blockchain data is accessible to anyone
- **Link-based exposure**: Raw content becomes visible to anyone with access
- **Security gaps**: Traditional encryption methods don't align with Web3's decentralized nature


## Our Solution

WETKEYS combines multiple advanced technologies to create a secure, user-friendly transfer system:

### Core Technology Stack
- **Vetkeys IBE**: Identity-based encryption from Internet Computer (ICP)
- **Walrus Storage**: Decentralized storage on Sui blockchain
- **Base Integration**: Payment abstraction layer using USDC
- **Cross-chain compatibility**: Chain-agnostic architecture

### Key Features
- 🔐 **Identity-Based Encryption**: Based on threshold-variant of Boneh-Franklin IBE
- 💰 **Simple Payments**: Users pay in USDC with ICP payment abstraction
- 🌐 **Chain Agnostic**: Works across multiple blockchain networks
- 🛡️ **Production Ready**: Built with enterprise-grade security

## Technology Stack

### Blockchain & Encryption
- **Vetkeys**: Identity-based encryption on ICP chain
- **Walrus**: Decentralized storage on Sui
- **Base**: Payment abstraction layer
- **Solidity**: Smart contracts for Base integration

### Development Stack
- **Backend**: Rust
- **Frontend**: Next.js
- **Smart Contracts**: Solidity
- **Development Tools**: Hardhat
- **Local Development**: DFX (Internet Computer SDK)

## Getting Started

### Prerequisites
- Node.js and npm
- DFX (Internet Computer SDK)
- Hardhat

### Installation & Setup

1. **Start the local ICP chain**
   ```bash
   dfx start --clean
   ```

2. **Deploy contracts** (in a new terminal)
   ```bash
   dfx deploy
   ```

3. **Configure Internet Identity**
   - Copy the "internet identity" "recommended" address from the deployment output
   - Update `vtk/src/vtk_frontend/src/App.jsx`:
   ```javascript
   const identityProvider =
     process.env.DFX_NETWORK === "ic"
       ? "https://identity.ic0.app"
       : "$THISADDRESS"; // Replace with your copied address
   ```

4. **Redeploy with updated configuration**
   ```bash
   dfx deploy
   ```

5. **Start the frontend** (in a third terminal)
   ```bash
   npm run dev
   ```

6. **Test the application**
   - Navigate to the local frontend URL provided by the dev server
   - Test user functionalities through the web interface

## Development Status

### Current Phase
- ✅ **Alpha Testing**: Core functionality implemented and testing in production
- ✅ **Vetkeys Integration**: IBE encryption system operational
- ✅ **Multi-chain Support**: Base and ICP integration complete

### Roadmap
- **Today**: Kealler app development and testing
- **Tomorrow**: Developer tools and SDK
- **Near Future**: Integration with additional projects and chains

## Architecture

### Security Layer
- **Vetkeys IBE**: Provides identity-based encryption aligned with Mysten Labs' strategy
- **Threshold Encryption**: Based on Boneh-Franklin IBE with threshold variants
- **Decentralized Storage**: Walrus ensures data availability without single points of failure

### Payment Layer
- **USDC Integration**: Simplified user payments
- **Base Abstraction**: Removes complexity of ICP payment handling
- **Cross-chain Compatibility**: Seamless experience across different networks



## Key Innovations

1. **Identity-Based Encryption**: Eliminates complex key management
2. **Payment Abstraction**: Users interact with familiar USDC payments
3. **Chain-Agnostic Design**: Works across multiple blockchain ecosystems
4. **Production-Ready Security**: Built with enterprise-grade encryption standards

## Resources

- [Walrus Documentation](https://www.walrus.xyz/blog/seal-decentralized-storage-encryption)
- [Vetkeys Technical Details](https://internetcomputer.org/docs/current/developer-docs/integrations/vetkeys/)
- [Internet Computer Development Guide](https://internetcomputer.org/docs/current/developer-docs/setup/install/)


## Team

Our team brings together expertise in blockchain development, cryptography, and financial engineering:
- **Stefano & Leo**: ICP grant recipients with deep blockchain experience
- **Victor**: Dual master's degrees in Finance and AI Engineering

<img width="1040" alt="Screenshot 2025-07-06 at 07 00 27" src="https://github.com/user-attachments/assets/10123109-e576-42eb-9935-5672427329ec" />


*WETKEYS: Making Web3 transfers safe and simple through advanced cryptography and seamless user experience.*
