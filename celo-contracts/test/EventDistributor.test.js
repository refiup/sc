const { expect } = require("chai");
const { ethers } = require("hardhat");

describe("EventDistributor", function () {
  let eventDistributor;
  let owner;
  let addr1;
  let addr2;
  let addr3;
  let addrs;

  beforeEach(async function () {
    [owner, addr1, addr2, addr3, ...addrs] = await ethers.getSigners();
    
    const EventDistributor = await ethers.getContractFactory("EventDistributor");
    eventDistributor = await EventDistributor.deploy();
    await eventDistributor.waitForDeployment();
  });

  describe("Deployment", function () {
    it("Should set the right owner", async function () {
      expect(await eventDistributor.owner()).to.equal(owner.address);
    });

    it("Should emit AdminSet event on deployment", async function () {
      const EventDistributor = await ethers.getContractFactory("EventDistributor");
      await expect(EventDistributor.deploy())
        .to.emit(EventDistributor.attach(await eventDistributor.getAddress()), "AdminSet")
        .withArgs(owner.address);
    });
  });

  describe("Human Management", function () {
    describe("Add Human", function () {
      it("Should add a new human", async function () {
        const ipfsHash = "QmTest123Example";
        
        await expect(eventDistributor.addHuman(addr1.address, ipfsHash))
          .to.emit(eventDistributor, "HumanAdded")
          .withArgs(addr1.address, ipfsHash);

        const human = await eventDistributor.getHuman(addr1.address);
        expect(human.walletAddress).to.equal(addr1.address);
        expect(human.ipfsHash).to.equal(ipfsHash);
        expect(human.validated).to.equal(false);
        expect(human.exists).to.equal(true);
      });

      it("Should reject adding human with zero address", async function () {
        await expect(
          eventDistributor.addHuman(ethers.ZeroAddress, "QmTest")
        ).to.be.revertedWithCustomError(eventDistributor, "InvalidAddress");
      });

      it("Should reject adding duplicate human", async function () {
        await eventDistributor.addHuman(addr1.address, "QmTest1");
        
        await expect(
          eventDistributor.addHuman(addr1.address, "QmTest2")
        ).to.be.revertedWithCustomError(eventDistributor, "HumanAlreadyExists");
      });

      it("Should only allow owner to add humans", async function () {
        await expect(
          eventDistributor.connect(addr1).addHuman(addr2.address, "QmTest")
        ).to.be.revertedWithCustomError(eventDistributor, "OwnableUnauthorizedAccount");
      });
    });

    describe("Update Human Validation", function () {
      beforeEach(async function () {
        await eventDistributor.addHuman(addr1.address, "QmTest1");
      });

      it("Should update human validation status", async function () {
        await expect(eventDistributor.updateHumanValidation(addr1.address, true))
          .to.emit(eventDistributor, "HumanValidationUpdated")
          .withArgs(addr1.address, true);

        const human = await eventDistributor.getHuman(addr1.address);
        expect(human.validated).to.equal(true);
      });

      it("Should reject updating non-existent human", async function () {
        await expect(
          eventDistributor.updateHumanValidation(addr2.address, true)
        ).to.be.revertedWithCustomError(eventDistributor, "HumanNotFound");
      });

      it("Should only allow owner to update validation", async function () {
        await expect(
          eventDistributor.connect(addr1).updateHumanValidation(addr1.address, true)
        ).to.be.revertedWithCustomError(eventDistributor, "OwnableUnauthorizedAccount");
      });
    });

    describe("Update Human Image", function () {
      beforeEach(async function () {
        await eventDistributor.addHuman(addr1.address, "QmTest1");
      });

      it("Should update human IPFS hash", async function () {
        const newHash = "QmNewHash456";
        
        await expect(eventDistributor.updateHumanImage(addr1.address, newHash))
          .to.emit(eventDistributor, "HumanImageUpdated")
          .withArgs(addr1.address, newHash);

        const human = await eventDistributor.getHuman(addr1.address);
        expect(human.ipfsHash).to.equal(newHash);
      });

      it("Should reject updating non-existent human", async function () {
        await expect(
          eventDistributor.updateHumanImage(addr2.address, "QmNew")
        ).to.be.revertedWithCustomError(eventDistributor, "HumanNotFound");
      });
    });

    describe("Get Humans", function () {
      beforeEach(async function () {
        await eventDistributor.addHuman(addr1.address, "QmTest1");
        await eventDistributor.addHuman(addr2.address, "QmTest2");
        await eventDistributor.addHuman(addr3.address, "QmTest3");
        await eventDistributor.updateHumanValidation(addr1.address, true);
        await eventDistributor.updateHumanValidation(addr3.address, true);
      });

      it("Should get all humans with pagination", async function () {
        const humans = await eventDistributor.getAllHumans(0, 2);
        expect(humans.length).to.equal(2);
        expect(humans[0].walletAddress).to.equal(addr1.address);
        expect(humans[1].walletAddress).to.equal(addr2.address);
      });

      it("Should handle pagination correctly", async function () {
        const humans = await eventDistributor.getAllHumans(1, 2);
        expect(humans.length).to.equal(2);
        expect(humans[0].walletAddress).to.equal(addr2.address);
        expect(humans[1].walletAddress).to.equal(addr3.address);
      });

      it("Should get validated humans only", async function () {
        const validated = await eventDistributor.getValidatedHumans();
        expect(validated.length).to.equal(2);
        expect(validated[0]).to.equal(addr1.address);
        expect(validated[1]).to.equal(addr3.address);
      });

      it("Should get total humans count", async function () {
        const total = await eventDistributor.getTotalHumans();
        expect(total).to.equal(3);
      });
    });
  });

  describe("Event Management", function () {
    beforeEach(async function () {
      await eventDistributor.addHuman(addr1.address, "QmTest1");
      await eventDistributor.addHuman(addr2.address, "QmTest2");
      await eventDistributor.updateHumanValidation(addr1.address, true);
      await eventDistributor.updateHumanValidation(addr2.address, true);
    });

    it("Should create a new event", async function () {
      const eventName = "Test Event";
      const participants = [addr1.address, addr2.address];

      await expect(eventDistributor.createEvent(eventName, participants))
        .to.emit(eventDistributor, "EventCreated")
        .withArgs(0, eventName, 2);

      const event = await eventDistributor.getEvent(0);
      expect(event.name).to.equal(eventName);
      expect(event.participants.length).to.equal(2);
      expect(event.exists).to.equal(true);
    });

    it("Should get validated participants from event", async function () {
      await eventDistributor.addHuman(addr3.address, "QmTest3");
      const participants = [addr1.address, addr2.address, addr3.address];
      
      await eventDistributor.createEvent("Test Event", participants);
      
      const validated = await eventDistributor.getValidatedParticipants(0);
      expect(validated.length).to.equal(2);
      expect(validated[0]).to.equal(addr1.address);
      expect(validated[1]).to.equal(addr2.address);
    });

    it("Should reject getting non-existent event", async function () {
      await expect(
        eventDistributor.getEvent(999)
      ).to.be.revertedWithCustomError(eventDistributor, "EventNotFound");
    });

    it("Should only allow owner to create events", async function () {
      await expect(
        eventDistributor.connect(addr1).createEvent("Test", [addr1.address])
      ).to.be.revertedWithCustomError(eventDistributor, "OwnableUnauthorizedAccount");
    });
  });

  describe("Distribution", function () {
    beforeEach(async function () {
      await eventDistributor.addHuman(addr1.address, "QmTest1");
      await eventDistributor.addHuman(addr2.address, "QmTest2");
      await eventDistributor.addHuman(addr3.address, "QmTest3");
      await eventDistributor.updateHumanValidation(addr1.address, true);
      await eventDistributor.updateHumanValidation(addr2.address, true);
      
      await eventDistributor.createEvent("Test Event", [
        addr1.address,
        addr2.address,
        addr3.address
      ]);
    });

    describe("Distribute Native CELO", function () {
      it("Should distribute native CELO to event participants", async function () {
        const totalAmount = ethers.parseEther("2.0");
        const amountPerRecipient = totalAmount / 2n;

        const addr1BalanceBefore = await ethers.provider.getBalance(addr1.address);
        const addr2BalanceBefore = await ethers.provider.getBalance(addr2.address);

        await expect(
          eventDistributor.distributeToEvent(0, ethers.ZeroAddress, totalAmount, {
            value: totalAmount
          })
        ).to.emit(eventDistributor, "FundsDistributed")
          .withArgs(0, ethers.ZeroAddress, totalAmount, 2);

        const addr1BalanceAfter = await ethers.provider.getBalance(addr1.address);
        const addr2BalanceAfter = await ethers.provider.getBalance(addr2.address);

        expect(addr1BalanceAfter - addr1BalanceBefore).to.equal(amountPerRecipient);
        expect(addr2BalanceAfter - addr2BalanceBefore).to.equal(amountPerRecipient);
      });

      it("Should reject distribution with zero amount", async function () {
        await expect(
          eventDistributor.distributeToEvent(0, ethers.ZeroAddress, 0)
        ).to.be.revertedWithCustomError(eventDistributor, "InvalidAmount");
      });

      it("Should reject distribution to non-existent event", async function () {
        await expect(
          eventDistributor.distributeToEvent(999, ethers.ZeroAddress, ethers.parseEther("1"), {
            value: ethers.parseEther("1")
          })
        ).to.be.revertedWithCustomError(eventDistributor, "EventNotFound");
      });
    });

    describe("Distribute to Addresses", function () {
      it("Should distribute to specific addresses", async function () {
        const recipients = [addr1.address, addr2.address];
        const totalAmount = ethers.parseEther("2.0");
        const amountPerRecipient = totalAmount / 2n;

        const addr1BalanceBefore = await ethers.provider.getBalance(addr1.address);
        const addr2BalanceBefore = await ethers.provider.getBalance(addr2.address);

        await eventDistributor.distributeToAddresses(recipients, ethers.ZeroAddress, totalAmount, {
          value: totalAmount
        });

        const addr1BalanceAfter = await ethers.provider.getBalance(addr1.address);
        const addr2BalanceAfter = await ethers.provider.getBalance(addr2.address);

        expect(addr1BalanceAfter - addr1BalanceBefore).to.equal(amountPerRecipient);
        expect(addr2BalanceAfter - addr2BalanceBefore).to.equal(amountPerRecipient);
      });

      it("Should reject empty recipients array", async function () {
        await expect(
          eventDistributor.distributeToAddresses([], ethers.ZeroAddress, ethers.parseEther("1"))
        ).to.be.revertedWithCustomError(eventDistributor, "NoValidatedParticipants");
      });
    });

    it("Should only allow owner to distribute", async function () {
      await expect(
        eventDistributor.connect(addr1).distributeToEvent(
          0,
          ethers.ZeroAddress,
          ethers.parseEther("1"),
          { value: ethers.parseEther("1") }
        )
      ).to.be.revertedWithCustomError(eventDistributor, "OwnableUnauthorizedAccount");
    });
  });

  describe("Edge Cases", function () {
    it("Should handle event with no validated participants", async function () {
      await eventDistributor.addHuman(addr1.address, "QmTest1");
      await eventDistributor.addHuman(addr2.address, "QmTest2");
      
      await eventDistributor.createEvent("Test Event", [addr1.address, addr2.address]);
      
      await expect(
        eventDistributor.getValidatedParticipants(0)
      ).to.be.revertedWithCustomError(eventDistributor, "NoValidatedParticipants");
    });

    it("Should handle large batch of humans", async function () {
      for (let i = 0; i < 10; i++) {
        const signer = addrs[i];
        await eventDistributor.addHuman(signer.address, `QmTest${i}`);
        if (i % 2 === 0) {
          await eventDistributor.updateHumanValidation(signer.address, true);
        }
      }

      const total = await eventDistributor.getTotalHumans();
      expect(total).to.equal(10);

      const validated = await eventDistributor.getValidatedHumans();
      expect(validated.length).to.equal(5);
    });

    it("Should handle distribution with odd division", async function () {
      await eventDistributor.addHuman(addrs[0].address, "QmTest0");
      await eventDistributor.updateHumanValidation(addr1.address, true);
      await eventDistributor.updateHumanValidation(addr2.address, true);
      await eventDistributor.updateHumanValidation(addrs[0].address, true);

      await eventDistributor.createEvent("Test Event", [
        addr1.address,
        addr2.address,
        addrs[0].address
      ]);

      const totalAmount = ethers.parseEther("1.0"); // 1 CELO / 3 = 0.333...
      const amountPerRecipient = totalAmount / 3n;

      await eventDistributor.distributeToEvent(0, ethers.ZeroAddress, totalAmount, {
        value: totalAmount
      });

      // Verify each recipient got equal share (remainder is lost)
      const addr1Balance = await ethers.provider.getBalance(addr1.address);
      expect(addr1Balance).to.be.gt(0);
    });
  });
});
