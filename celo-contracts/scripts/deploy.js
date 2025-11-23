const hre = require("hardhat");
const fs = require("fs");
const path = require("path");

async function main() {
  console.log("🚀 Starting EventDistributor deployment to Celo...\n");

  const [deployer] = await hre.ethers.getSigners();
  const network = hre.network.name;

  console.log("📍 Network:", network);
  console.log("👤 Deployer address:", deployer.address);
  
  const balance = await hre.ethers.provider.getBalance(deployer.address);
  console.log("💰 Deployer balance:", hre.ethers.formatEther(balance), "CELO\n");

  if (balance === 0n) {
    console.error("❌ Error: Deployer account has no balance!");
    console.log("💡 Fund your account with CELO:");
    if (network === "alfajores") {
      console.log("   Alfajores Faucet: https://faucet.celo.org/alfajores");
    }
    process.exit(1);
  }

  // Deploy EventDistributor
  console.log("📦 Deploying EventDistributor...");
  const EventDistributor = await hre.ethers.getContractFactory("EventDistributor");
  const eventDistributor = await EventDistributor.deploy();
  
  await eventDistributor.waitForDeployment();
  const eventDistributorAddress = await eventDistributor.getAddress();

  console.log("✅ EventDistributor deployed to:", eventDistributorAddress);
  console.log("👨‍⚖️ Owner/Admin:", deployer.address);

  // Save deployment info
  const deploymentInfo = {
    network: network,
    chainId: (await hre.ethers.provider.getNetwork()).chainId,
    deployer: deployer.address,
    timestamp: new Date().toISOString(),
    contracts: {
      EventDistributor: {
        address: eventDistributorAddress,
        txHash: eventDistributor.deploymentTransaction()?.hash
      }
    }
  };

  const deploymentsDir = path.join(__dirname, "..", "deployments");
  if (!fs.existsSync(deploymentsDir)) {
    fs.mkdirSync(deploymentsDir);
  }

  const deploymentFile = path.join(deploymentsDir, `${network}.json`);
  fs.writeFileSync(deploymentFile, JSON.stringify(deploymentInfo, null, 2));

  console.log("\n📄 Deployment info saved to:", deploymentFile);

  // Explorer links
  const explorerUrl = network === "alfajores" 
    ? "https://alfajores.celoscan.io" 
    : "https://celoscan.io";

  console.log("\n🔍 View on Celoscan:");
  console.log(`   ${explorerUrl}/address/${eventDistributorAddress}`);

  // Wait for block confirmations before verification
  if (network !== "hardhat" && network !== "localhost") {
    console.log("\n⏳ Waiting for 5 block confirmations...");
    await eventDistributor.deploymentTransaction()?.wait(5);
    console.log("✅ Confirmed!");

    // Verify contract
    console.log("\n🔍 Verifying contract on Celoscan...");
    try {
      await hre.run("verify:verify", {
        address: eventDistributorAddress,
        constructorArguments: []
      });
      console.log("✅ Contract verified!");
    } catch (error) {
      console.log("⚠️ Verification failed:", error.message);
      console.log("   You can verify manually later with:");
      console.log(`   npx hardhat verify --network ${network} ${eventDistributorAddress}`);
    }
  }

  console.log("\n✨ Deployment complete!\n");
  console.log("📋 Summary:");
  console.log("   Contract: EventDistributor");
  console.log("   Address:", eventDistributorAddress);
  console.log("   Network:", network);
  console.log("   Explorer:", `${explorerUrl}/address/${eventDistributorAddress}`);
  console.log("\n🎉 Ready to use!");
}

main()
  .then(() => process.exit(0))
  .catch((error) => {
    console.error(error);
    process.exit(1);
  });
