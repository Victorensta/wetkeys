// setupDevWallet.ts
import * as fs from "fs";
import * as path from "path";
import { Ed25519Keypair } from "@mysten/sui/keypairs/ed25519";

/**
 * Save the keypair to the .env file, preserving other variables.
 * Updates VITE_SUI_SECRET_KEY and VITE_SUI_ADDRESS if they exist, or adds them if not.
 * Never commit .env files with private keys to version control!
 */
function saveKeypair(keypair: Ed25519Keypair) {
  // Use the full 64-byte secret key (private + public) or slice as you wish
  const privateKey = keypair.getSecretKey().slice(0, 32); // Only the first 32 bytes!
  const privateKeyBase64 = Buffer.from(privateKey).toString("base64");
  const address = keypair.getPublicKey().toSuiAddress();

  console.log("Secret key length (bytes):", privateKey.length);
  if (privateKey.length !== 64) {
    console.warn("⚠️ Warning: Secret key is not 64 bytes! It is:", privateKey.length, "bytes.");
  }

  // --- Append to .env file ---
  const envPath = path.join(process.cwd(), ".env");
  const timestamp = new Date().toISOString();
  const envAppend = `\n# Sui Dev Wallet generated ${timestamp}\nVITE_SUI_SECRET_KEY=${privateKeyBase64}\nVITE_SUI_ADDRESS=${address}\n`;
  fs.appendFileSync(envPath, envAppend);
  console.log(`✅ Appended wallet to .env file`);
  console.log(`📝 Address: ${address}`);
  console.log(`🔐 Private key appended as VITE_SUI_SECRET_KEY (should be 64 bytes)`);
  console.log(`🚨 Never commit .env files with private keys to version control!`);

  // --- Save to .sui-dev-wallet/wallet.json ---
  const walletDir = path.join(process.cwd(), ".sui-dev-wallet");
  if (!fs.existsSync(walletDir)) {
    fs.mkdirSync(walletDir, { recursive: true });
  }
  const walletJsonPath = path.join(walletDir, "wallet.json");
  const walletJson = {
    privateKey: privateKeyBase64,
    address: address,
  };
  fs.writeFileSync(walletJsonPath, JSON.stringify(walletJson, null, 2));
  console.log(`💾 Wallet also saved to .sui-dev-wallet/wallet.json`);
}

/**
 * Load the keypair from the .env file, if present.
 */
function loadKeypair(): Ed25519Keypair | null {
  const envPath = path.join(process.cwd(), ".env");
  if (!fs.existsSync(envPath)) {
    return null;
  }

  const envContent = fs.readFileSync(envPath, "utf-8");
  const env: { [key: string]: string } = {};

  envContent.split("\n").forEach((line) => {
    const trimmed = line.trim();
    if (trimmed && !trimmed.startsWith("#")) {
      const [key, ...valueParts] = trimmed.split("=");
      if (key && valueParts.length > 0) {
        env[key] = valueParts.join("=");
      }
    }
  });

  const secretKey = env.VITE_SUI_SECRET_KEY;
  if (secretKey) {
    const privateKeyBytes = Buffer.from(secretKey, "base64");
    return Ed25519Keypair.fromSecretKey(privateKeyBytes);
  }
  return null;
}

/**
 * Ensure a dev wallet exists: load if present, otherwise generate and save a new one.
 */
function ensureWallet(): Ed25519Keypair {
  // Always generate a new keypair and save it
  const keypair = new Ed25519Keypair();
  saveKeypair(keypair);
  console.log("🔧 New dev wallet generated.");
  return keypair;
}

// MAIN
const keypair = ensureWallet();
const address = keypair.getPublicKey().toSuiAddress();

console.log(`📮 Your dev wallet address: ${address}`);
console.log(`🌐 Fund it from the faucet: https://faucet.testnet.sui.io/`);
