// setupDevWallet.ts
import * as fs from "fs";
import * as path from "path";
import { Ed25519Keypair } from "@mysten/sui/keypairs/ed25519";

const WALLET_DIR = path.join(process.cwd(), ".sui-dev-wallet");
const WALLET_FILE = path.join(WALLET_DIR, "wallet.json");

function saveKeypair(keypair: Ed25519Keypair) {
  const privateKey = keypair.getSecretKey();
  const privateKeyBase64 = Buffer.from(privateKey).toString("base64");
  const address = keypair.getPublicKey().toSuiAddress();
  fs.writeFileSync(WALLET_FILE, JSON.stringify({ privateKey: privateKeyBase64, address }, null, 2));
  console.log(`✅ Wallet saved to ${WALLET_FILE}`);
}

function loadKeypair(): Ed25519Keypair | null {
  if (fs.existsSync(WALLET_FILE)) {
    const data = fs.readFileSync(WALLET_FILE, "utf-8");
    const parsed = JSON.parse(data);
    // Load from base64 string
    const privateKeyBytes = Buffer.from(parsed.privateKey, "base64");
    return Ed25519Keypair.fromSecretKey(privateKeyBytes);
  }
  return null;
}

function ensureWallet(): Ed25519Keypair {
  if (!fs.existsSync(WALLET_DIR)) {
    fs.mkdirSync(WALLET_DIR);
  }

  const existing = loadKeypair();
  if (existing) {
    console.log("🔐 Existing dev wallet loaded.");
    return existing;
  }

  const keypair = Ed25519Keypair.generate();
  saveKeypair(keypair);
  console.log("🔧 New dev wallet generated.");
  return keypair;
}

// MAIN
const keypair = ensureWallet();
const address = keypair.getPublicKey().toSuiAddress();

console.log(`📮 Your dev wallet address: ${address}`);
console.log(`🌐 Fund it from the faucet: https://faucet.testnet.sui.io/`);
