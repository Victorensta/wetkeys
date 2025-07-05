// check_balance.ts
import * as fs from "fs";
import * as path from "path";
import { getFullnodeUrl, SuiClient } from "@mysten/sui/client";
import { Ed25519Keypair } from "@mysten/sui/keypairs/ed25519";

// Initialize Sui client for testnet
const suiClient = new SuiClient({ url: getFullnodeUrl("testnet") });

function loadEnvFile(): { [key: string]: string } {
  const envPath = path.join(process.cwd(), ".env");
  if (!fs.existsSync(envPath)) {
    return {};
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

  return env;
}

function loadKeypair(): Ed25519Keypair | null {
  const env = loadEnvFile();
  const secretKey = env.VITE_SUI_SECRET_KEY;

  if (secretKey) {
    const privateKeyBytes = Buffer.from(secretKey, "base64");
    return Ed25519Keypair.fromSecretKey(privateKeyBytes);
  }
  return null;
}

async function checkBalances(address: string) {
  try {
    console.log(`🔍 Checking balances for address: ${address}`);
    console.log("─".repeat(60));

    // Get SUI balance
    const suiBalance = await suiClient.getBalance({
      owner: address,
      coinType: "0x2::sui::SUI",
    });

    console.log(`💰 SUI Balance:`);
    console.log(`   Amount: ${suiBalance.totalBalance} MIST`);
    console.log(`   Formatted: ${(parseInt(suiBalance.totalBalance) / 1_000_000_000).toFixed(4)} SUI`);
    console.log(`   Objects: ${suiBalance.coinObjectCount}`);

    // Check if SUI balance is sufficient for gas
    const suiAmount = parseInt(suiBalance.totalBalance) / 1_000_000_000;
    if (suiAmount < 0.01) {
      console.log(`   ⚠️  Low SUI balance! Need at least 0.01 SUI for gas fees`);
      console.log(`   🌐 Fund here: https://faucet.testnet.sui.io/`);
    } else {
      console.log(`   ✅ Sufficient SUI for gas fees`);
    }

    console.log();

    // Try to get WAL balance (Walrus token)
    try {
      const walBalance = await suiClient.getBalance({
        owner: address,
        coinType: "0x2::walrus::WAL", // This might need adjustment based on actual WAL token address
      });

      console.log(`🦭 WAL Balance:`);
      console.log(`   Amount: ${walBalance.totalBalance} WAL`);
      console.log(`   Objects: ${walBalance.coinObjectCount}`);

      const walAmount = parseInt(walBalance.totalBalance);
      if (walAmount < 1) {
        console.log(`   ⚠️  Low WAL balance! Need WAL tokens for storage`);
        console.log(`   🔍 Check Walrus docs for WAL token faucet`);
      } else {
        console.log(`   ✅ Sufficient WAL for storage`);
      }
    } catch (error) {
      console.log(`🦭 WAL Balance: No WAL tokens found`);
      console.log(`   ℹ️  You may need to get WAL tokens from Walrus testnet`);
    }

    console.log();
    console.log("─".repeat(60));

    // Overall status
    const hasSui = suiAmount >= 0.01;
    console.log(`📊 Status: ${hasSui ? "✅ Ready for testing" : "❌ Needs funding"}`);

    if (!hasSui) {
      console.log(`\n🚀 Next steps:`);
      console.log(`   1. Visit: https://faucet.testnet.sui.io/`);
      console.log(`   2. Enter your address: ${address}`);
      console.log(`   3. Request testnet SUI tokens`);
      console.log(`   4. Run this script again to verify`);
    }
  } catch (error) {
    console.error("❌ Error checking balances:", error);
  }
}

async function main() {
  const env = loadEnvFile();
  const address = env.VITE_SUI_ADDRESS;

  if (!address) {
    console.log("❌ No wallet found. Run 'npx tsx scripts/walrus_wallet.ts' first to generate a wallet.");
    return;
  }

  await checkBalances(address);
}

main().catch(console.error);
