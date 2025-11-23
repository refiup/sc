# 🌍 ReFi Universe - Celo Smart Contracts

Smart contracts for ReFi Universe deployed on Celo blockchain.

## 📦 Contracts

### EventDistributor
Main contract for managing validated participants and distributing funds.

**Features:**
- ✅ Human registry with IPFS metadata
- ✅ Validation system (admin-controlled)
- ✅ Event management
- ✅ Token distribution (CELO + ERC20)
- ✅ Access control
- ✅ Reentrancy protection

---

## 🚀 Quick Start

### 1. Installation

```bash
cd celo-contracts
npm install
```

### 2. Configuration

Create `.env` file:

```bash
cp .env.example .env
```

Edit `.env` and add:
- `PRIVATE_KEY`: Your deployer wallet private key
- `CELOSCAN_API_KEY`: API key from https://celoscan.io/

### 3. Get Testnet CELO

For Alfajores testnet:
- Visit: https://faucet.celo.org/alfajores
- Enter your address
- Get free testnet CELO

### 4. Compile

```bash
npm run compile
```

### 5. Deploy

**Alfajores Testnet:**
```bash
npm run deploy:alfajores
```

**Celo Mainnet:**
```bash
npm run deploy:celo
```

---

## 📊 Contract Interface

### Human Management

```solidity
// Add human
addHuman(address _address, string _ipfsHash)

// Update validation
updateHumanValidation(address _address, bool _validated)

// Update image
updateHumanImage(address _address, string _ipfsHash)

// Get human info
getHuman(address _address) returns (Human)

// Get all humans (paginated)
getAllHumans(uint256 _start, uint256 _limit) returns (Human[])

// Get validated humans
getValidatedHumans() returns (address[])
```

### Event Management

```solidity
// Create event
createEvent(string _name, address[] _participants) returns (uint256)

// Get event
getEvent(uint256 _eventId) returns (Event)

// Get validated participants
getValidatedParticipants(uint256 _eventId) returns (address[])
```

### Distribution

```solidity
// Distribute to event participants
distributeToEvent(uint256 _eventId, address _token, uint256 _totalAmount)

// Distribute to specific addresses
distributeToAddresses(address[] _recipients, address _token, uint256 _totalAmount)
```

**Token Options:**
- `address(0)` → Native CELO
- ERC20 address → Any Celo token (cUSD, cEUR, cREAL, etc.)

---

## 🧪 Testing

```bash
npm test
```

---

## 🔍 Verify Contract

After deployment:

```bash
npx hardhat verify --network alfajores <CONTRACT_ADDRESS>
```

---

## 📡 Networks

### Alfajores Testnet
- Chain ID: 44787
- RPC: https://alfajores-forno.celo-testnet.org
- Explorer: https://alfajores.celoscan.io
- Faucet: https://faucet.celo.org/alfajores

### Celo Mainnet
- Chain ID: 42220
- RPC: https://forno.celo.org
- Explorer: https://celoscan.io

---

## 📝 Contract Addresses

### Alfajores Testnet
- EventDistributor: `[To be deployed]`

### Celo Mainnet
- EventDistributor: `[To be deployed]`

---

## 🛠️ Development

### Compile
```bash
npm run compile
```

### Clean
```bash
npm run clean
```

### Test
```bash
npm test
```

---

## 🔐 Security

- Uses OpenZeppelin contracts
- ReentrancyGuard for distribution functions
- Ownable for access control
- Tested and audited code patterns

---

## 📚 Documentation

Full integration guides available in [`docs/UI-CELO/`](../docs/UI-CELO/)

---

## 🤝 Contributing

1. Fork the repository
2. Create your feature branch
3. Commit your changes
4. Push to the branch
5. Open a Pull Request

---

## 📄 License

MIT License - see LICENSE file for details

---

## 🔗 Links

- Website: https://refiuverse.com
- Documentation: [../docs/UI-CELO/](../docs/UI-CELO/)
- GitHub: https://github.com/refiup/sc
