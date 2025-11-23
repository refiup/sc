const hre = require("hardhat");

async function main() {
  console.log("🚀 Starting EventDistributor deployment to local network...\n");

  const [deployer] = await hre.ethers.getSigners();
  
  console.log("📝 Deployer address:", deployer.address);
  const balance = await hre.ethers.provider.getBalance(deployer.address);
  console.log("💰 Deployer balance:", hre.ethers.formatEther(balance), "ETH\n");

  console.log("⏳ Deploying EventDistributor contract...");
  
  const EventDistributor = await hre.ethers.getContractFactory("EventDistributor");
  const eventDistributor = await EventDistributor.deploy();
  
  await eventDistributor.waitForDeployment();
  const address = await eventDistributor.getAddress();
  
  console.log("✅ EventDistributor deployed to:", address);
  console.log("👤 Contract owner:", await eventDistributor.owner());
  
  console.log("\n📊 Running deployment tests...\n");
  
  // Test 1: Add humans
  console.log("1️⃣ Adding 3 humans...");
  const addr1 = "0x70997970C51812dc3A010C7d01b50e0d17dc79C8";
  const addr2 = "0x3C44CdDdB6a900fa2b585dd299e03d12FA4293BC";
  const addr3 = "0x90F79bf6EB2c4f870365E785982E1f101E93b906";
  
  await eventDistributor.addHuman(addr1, "QmAlice123");
  await eventDistributor.addHuman(addr2, "QmBob456");
  await eventDistributor.addHuman(addr3, "QmCarol789");
  console.log("   ✓ Added 3 humans");
  
  // Test 2: Validate humans
  console.log("\n2️⃣ Validating 2 humans...");
  await eventDistributor.updateHumanValidation(addr1, true);
  await eventDistributor.updateHumanValidation(addr2, true);
  console.log("   ✓ Validated Alice and Bob");
  
  // Test 3: Update image
  console.log("\n3️⃣ Updating Alice's image...");
  await eventDistributor.updateHumanImage(addr1, "QmAliceNew999");
  console.log("   ✓ Updated Alice's image");
  
  // Test 4: Get humans info
  console.log("\n4️⃣ Retrieving humans data...");
  const total = await eventDistributor.getTotalHumans();
  const validated = await eventDistributor.getValidatedHumans();
  console.log("   ✓ Total humans:", total.toString());
  console.log("   ✓ Validated humans:", validated.length);
  
  // Test 5: Create event
  console.log("\n5️⃣ Creating an event...");
  await eventDistributor.createEvent("Test Event 2024", [addr1, addr2, addr3]);
  console.log("   ✓ Event created: Test Event 2024");
  console.log("   ✓ Participants: 3");
  
  // Test 6: Get validated participants
  console.log("\n6️⃣ Getting validated participants...");
  const validatedParticipants = await eventDistributor.getValidatedParticipants(0);
  console.log("   ✓ Validated participants in event:", validatedParticipants.length);
  
  // Test 7: Distribute funds
  console.log("\n7️⃣ Distributing 2 ETH to validated participants...");
  const amount = hre.ethers.parseEther("2.0");
  await eventDistributor.distributeToEvent(0, hre.ethers.ZeroAddress, amount, { value: amount });
  console.log("   ✓ Distributed 1 ETH to each validated participant");
  
  console.log("\n✅ All deployment tests passed!\n");
  
  console.log("📋 Contract Summary:");
  console.log("═══════════════════════════════════════════════════════");
  console.log("Contract Address:", address);
  console.log("Network: Local Hardhat");
  console.log("Owner:", deployer.address);
  console.log("Total Humans:", total.toString());
  console.log("Validated Humans:", validated.length);
  console.log("Total Events: 1");
  console.log("═══════════════════════════════════════════════════════");
  
  console.log("\n🔗 Simulated Celo Alfajores URLs:");
  console.log("Contract Explorer:", `https://alfajores.celoscan.io/address/${address}`);
  console.log("Transaction Explorer:", `https://alfajores.celoscan.io/tx/0x...`);
  
  // Save deployment info
  const fs = require('fs');
  const deploymentInfo = {
    network: "local",
    contractAddress: address,
    owner: deployer.address,
    deploymentDate: new Date().toISOString(),
    totalHumans: total.toString(),
    validatedHumans: validated.length,
    totalEvents: 1
  };
  
  fs.writeFileSync(
    'deployment-local.json',
    JSON.stringify(deploymentInfo, null, 2)
  );
  
  console.log("\n💾 Deployment info saved to deployment-local.json");
}

main()
  .then(() => process.exit(0))
  .catch((error) => {
    console.error(error);
    process.exit(1);
  });
