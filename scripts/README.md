# Scripts Usage Guide

This folder contains utility scripts for managing your Sui dev wallet and checking balances for Walrus/Sui integration.

## 📂 Where to Run Scripts

**Always run these scripts from the project root directory** (not from inside the `scripts/` folder):

```bash
cd /path/to/your/project/root
npx tsx scripts/walrus_wallet.ts
npx tsx scripts/check_balance.ts
```

If you run them from inside the `scripts/` folder, you may get errors about missing files or wrong paths.

---

## 🪪 Wallet Management

### Generate or Load a Dev Wallet

```bash
npx tsx scripts/walrus_wallet.ts
```

- If a `.env` file exists with wallet credentials, it will be loaded.
- If not, a new wallet will be generated and saved to `.env` file.
- The `.env` file contains:
  - `VITE_SUI_SECRET_KEY`: base64-encoded private key
  - `VITE_SUI_ADDRESS`: wallet address (hex string)

### Fund Your Wallet

- Use the address shown by the script.
- Fund it with SUI (and WAL if needed) from the Sui testnet faucet: https://faucet.testnet.sui.io/

---

## 💰 Check Wallet Balance

```bash
npx tsx scripts/check_balance.ts
```

- Checks SUI and WAL token balances for your dev wallet.
- Warns if you need to fund your wallet.

---

## 📝 Notes

- These scripts are for development/testing only.
- Never use a dev wallet for production or with real funds.
- If you want to reset your wallet, delete the `.env` file and run the wallet script again.
- The `.env` file is automatically ignored by git for security.

---

## 🛠 Requirements

- Node.js 18+
- [tsx](https://npmjs.com/package/tsx) (runs automatically with `npx`)
- All dependencies installed (run `npm install` in the project root)
