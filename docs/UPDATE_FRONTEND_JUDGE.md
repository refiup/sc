# 🎯 ReFi Universe Contracts - Resumen Ejecutivo

## 📦 Contratos Disponibles

### 1. Vault Distributor - Distribución Simple
**Qué hace:** Distribuye tokens equitativamente entre múltiples destinatarios en una sola transacción.

**Caso de uso:** Payroll, airdrops, recompensas rápidas.

**Testnet:**
- Contract: `CC4XEZG3JIVNTWGNPL4YKIWYECSOTS66SFLIDI3WU6RIJFNDNWPIMVHM`
- [Ver en Explorer](https://stellar.expert/explorer/testnet/contract/CC4XEZG3JIVNTWGNPL4YKIWYECSOTS66SFLIDI3WU6RIJFNDNWPIMVHM)

**Funciones:**
- `init(admin)` - Inicializar con admin
- `distribute(token, recipients[], amount)` - Distribuir fondos
- `get_admin()` - Ver admin

---

### 2. Event Distributor - Distribución con Validación
**Qué hace:** Gestiona participantes on-chain con validación, crea eventos, y distribuye solo a validados.

**Caso de uso:** Eventos ReFi, comunidades curadas, distribuciones selectivas.

**Testnet:**
- Contract: `CCHFGFX3S52UX46HZEBEGW5N2LDDYMSJDLZF4CQOZ6TSWWKFEG4TFPLS`
- [Ver en Explorer](https://stellar.expert/explorer/testnet/contract/CCHFGFX3S52UX46HZEBEGW5N2LDDYMSJDLZF4CQOZ6TSWWKFEG4TFPLS)

**Funciones principales:**
- `add_human(address, ipfs_hash)` - Agregar participante
- `update_human_validation(address, bool)` - Validar participante
- `create_event(id, location, pool)` - Crear evento
- `add_human_to_event(event_id, address)` - Agregar a evento
- `distribute_event_pool(event_id, token)` - Distribuir solo a validados
- `get_all_humans(start, limit)` - Listar participantes (paginado)

---

## 🔗 Links Importantes

**Repository:** https://github.com/refiup/sc

**Documentación:**
- [README Principal](https://github.com/refiup/sc/blob/Master/README.md)
- [Vault Distributor API](https://github.com/refiup/sc/blob/Master/contracts/vault-distributor/README.md)
- [Event Distributor API](https://github.com/refiup/sc/blob/Master/contracts/event-distributor/README.md)
- [Guía de Deployment](https://github.com/refiup/sc/blob/Master/docs/DEPLOYMENT.md)
- [Guía de Integración Frontend](https://github.com/refiup/sc/blob/Master/docs/INTEGRATION.md)

**Explorers:**
- [Vault Distributor Testnet](https://stellar.expert/explorer/testnet/contract/CC4XEZG3JIVNTWGNPL4YKIWYECSOTS66SFLIDI3WU6RIJFNDNWPIMVHM)
- [Event Distributor Testnet](https://stellar.expert/explorer/testnet/contract/CCHFGFX3S52UX46HZEBEGW5N2LDDYMSJDLZF4CQOZ6TSWWKFEG4TFPLS)

---

## 📊 Tests Disponibles

**Total:** 42 tests (100% passing)
- Vault Distributor: 19 tests
- Event Distributor: 23 tests

```bash
# Ejecutar todos los tests
cargo test --all

# Tests individuales
cd contracts/vault-distributor && cargo test
cd contracts/event-distributor && cargo test
```

---

## 🚀 Integración con judge-ai-simulator

### Paso 1: Instalación de Dependencias

```bash
npm install @stellar/stellar-sdk
# o
yarn add @stellar/stellar-sdk
```

### Paso 2: Configuración de Contratos

```javascript
// src/config/contracts.js
export const CONTRACTS = {
  network: 'testnet',
  networkPassphrase: 'Test SDF Network ; September 2015',
  rpcUrl: 'https://soroban-testnet.stellar.org',
  
  vaultDistributor: {
    id: 'CC4XEZG3JIVNTWGNPL4YKIWYECSOTS66SFLIDI3WU6RIJFNDNWPIMVHM',
    name: 'Vault Distributor',
    type: 'simple'
  },
  
  eventDistributor: {
    id: 'CCHFGFX3S52UX46HZEBEGW5N2LDDYMSJDLZF4CQOZ6TSWWKFEG4TFPLS',
    name: 'Event Distributor',
    type: 'advanced'
  },
  
  tokens: {
    xlm: 'CDLZFC3SYJYDZT7K67VZ75HPJVIEUVNIXF47ZG2FB2RMQQVU2HHGCYSC'
  }
};
```

### Paso 3: Cliente de Contratos

```javascript
// src/services/ContractClient.js
import { Contract, SorobanRpc, TransactionBuilder } from '@stellar/stellar-sdk';
import { CONTRACTS } from '../config/contracts';

export class ContractClient {
  constructor(contractId) {
    this.contractId = contractId;
    this.server = new SorobanRpc.Server(CONTRACTS.rpcUrl);
    this.contract = new Contract(contractId);
  }

  async call(method, params = [], wallet) {
    const account = await this.server.getAccount(wallet.publicKey);
    
    const operation = this.contract.call(method, ...params);
    
    const tx = new TransactionBuilder(account, {
      fee: '1000',
      networkPassphrase: CONTRACTS.networkPassphrase
    })
    .addOperation(operation)
    .setTimeout(30)
    .build();
    
    const signedTx = await wallet.signTransaction(tx.toXDR());
    return await this.server.sendTransaction(signedTx);
  }

  async simulate(method, params = []) {
    const operation = this.contract.call(method, ...params);
    // Implementar lógica de simulación
  }
}
```

### Paso 4: Componentes React

#### Wallet Manager
```jsx
// src/components/WalletManager.jsx
import React, { useState, useEffect } from 'react';
import { CONTRACTS } from '../config/contracts';

export function WalletManager() {
  const [wallets, setWallets] = useState([]);
  const [newWallet, setNewWallet] = useState({ address: '', name: '' });

  const addWallet = () => {
    setWallets([...wallets, {
      id: Date.now(),
      address: newWallet.address,
      name: newWallet.name,
      validated: false,
      balance: 0
    }]);
    setNewWallet({ address: '', name: '' });
  };

  const fundWallet = async (address, amount) => {
    // Implementar funding via Friendbot o transferencia
    try {
      const response = await fetch(
        `https://friendbot.stellar.org?addr=${address}`
      );
      if (response.ok) {
        alert(`Wallet ${address} funded with testnet XLM!`);
      }
    } catch (error) {
      console.error('Funding error:', error);
    }
  };

  return (
    <div className="wallet-manager">
      <h2>Gestión de Wallets</h2>
      
      {/* Add Wallet Form */}
      <div className="add-wallet">
        <input
          placeholder="Stellar Address (G...)"
          value={newWallet.address}
          onChange={(e) => setNewWallet({...newWallet, address: e.target.value})}
        />
        <input
          placeholder="Nombre"
          value={newWallet.name}
          onChange={(e) => setNewWallet({...newWallet, name: e.target.value})}
        />
        <button onClick={addWallet}>Agregar Wallet</button>
      </div>

      {/* Wallets List */}
      <div className="wallets-list">
        {wallets.map(wallet => (
          <div key={wallet.id} className="wallet-card">
            <h3>{wallet.name}</h3>
            <p>{wallet.address.substring(0, 10)}...</p>
            <p>Balance: {wallet.balance} XLM</p>
            <button onClick={() => fundWallet(wallet.address, 1000)}>
              Fund with Testnet XLM
            </button>
          </div>
        ))}
      </div>
    </div>
  );
}
```

#### Event Manager (Event Distributor)
```jsx
// src/components/EventManager.jsx
import React, { useState, useEffect } from 'react';
import { ContractClient } from '../services/ContractClient';
import { CONTRACTS } from '../config/contracts';

export function EventManager() {
  const [humans, setHumans] = useState([]);
  const [events, setEvents] = useState([]);
  const client = new ContractClient(CONTRACTS.eventDistributor.id);

  // Agregar participante
  const addHuman = async (address, ipfsHash) => {
    try {
      await client.call('add_human', [address, ipfsHash], wallet);
      loadHumans();
    } catch (error) {
      console.error('Error adding human:', error);
    }
  };

  // Validar participante
  const validateHuman = async (address, validated) => {
    try {
      await client.call('update_human_validation', [address, validated], wallet);
      loadHumans();
    } catch (error) {
      console.error('Error validating human:', error);
    }
  };

  // Crear evento
  const createEvent = async (eventId, location, pool) => {
    try {
      await client.call('create_event', [eventId, location, pool], wallet);
      loadEvents();
    } catch (error) {
      console.error('Error creating event:', error);
    }
  };

  return (
    <div className="event-manager">
      <h2>Event Distributor - Gestión</h2>
      
      {/* Humans Section */}
      <section>
        <h3>Participantes</h3>
        <button onClick={() => loadHumans()}>Cargar Participantes</button>
        <table>
          <thead>
            <tr>
              <th>Address</th>
              <th>IPFS Hash</th>
              <th>Validated</th>
              <th>Actions</th>
            </tr>
          </thead>
          <tbody>
            {humans.map(human => (
              <tr key={human.address}>
                <td>{human.address.substring(0, 15)}...</td>
                <td>{human.ipfsHash.substring(0, 15)}...</td>
                <td>{human.validated ? '✅' : '❌'}</td>
                <td>
                  <button onClick={() => validateHuman(human.address, !human.validated)}>
                    Toggle Validation
                  </button>
                </td>
              </tr>
            ))}
          </tbody>
        </table>
      </section>

      {/* Events Section */}
      <section>
        <h3>Eventos</h3>
        {events.map(event => (
          <div key={event.id} className="event-card">
            <h4>{event.eventName}</h4>
            <p>Location: {event.location}</p>
            <p>Pool: {event.pool / 10000000} XLM</p>
            <p>Participants: {event.humans.length}</p>
            <button onClick={() => distributeEvent(event.id)}>
              Distribute
            </button>
          </div>
        ))}
      </section>
    </div>
  );
}
```

#### Simple Distributor (Vault Distributor)
```jsx
// src/components/SimpleDistributor.jsx
import React, { useState } from 'react';
import { ContractClient } from '../services/ContractClient';
import { CONTRACTS } from '../config/contracts';

export function SimpleDistributor() {
  const [recipients, setRecipients] = useState(['', '', '']);
  const [amount, setAmount] = useState(1000000000); // 100 XLM
  const client = new ContractClient(CONTRACTS.vaultDistributor.id);

  const distribute = async () => {
    const validRecipients = recipients.filter(r => r !== '');
    
    try {
      const result = await client.call(
        'distribute',
        [
          CONTRACTS.tokens.xlm,
          validRecipients,
          amount
        ],
        wallet
      );
      
      alert('Distribución exitosa!');
    } catch (error) {
      console.error('Distribution error:', error);
      alert('Error: ' + error.message);
    }
  };

  return (
    <div className="simple-distributor">
      <h2>Vault Distributor - Distribución Simple</h2>
      
      <div className="form">
        <label>Recipients:</label>
        {recipients.map((r, i) => (
          <input
            key={i}
            value={r}
            onChange={(e) => {
              const newRecipients = [...recipients];
              newRecipients[i] = e.target.value;
              setRecipients(newRecipients);
            }}
            placeholder={`Recipient ${i + 1} (G...)`}
          />
        ))}
        
        <label>Total Amount (stroops):</label>
        <input
          type="number"
          value={amount}
          onChange={(e) => setAmount(parseInt(e.target.value))}
        />
        
        <button onClick={distribute}>
          Distribute
        </button>
      </div>
    </div>
  );
}
```

### Paso 5: Integración en App Principal

```jsx
// src/App.jsx
import React, { useState } from 'react';
import { WalletManager } from './components/WalletManager';
import { EventManager } from './components/EventManager';
import { SimpleDistributor } from './components/SimpleDistributor';

export function App() {
  const [activeTab, setActiveTab] = useState('wallets');

  return (
    <div className="app">
      <nav>
        <button onClick={() => setActiveTab('wallets')}>Wallets</button>
        <button onClick={() => setActiveTab('simple')}>Simple Distribution</button>
        <button onClick={() => setActiveTab('events')}>Event Distribution</button>
      </nav>

      <main>
        {activeTab === 'wallets' && <WalletManager />}
        {activeTab === 'simple' && <SimpleDistributor />}
        {activeTab === 'events' && <EventManager />}
      </main>
    </div>
  );
}
```

### Paso 6: Comandos CLI para Testing

```bash
# Vault Distributor - Distribución simple
stellar contract invoke \
  --id CC4XEZG3JIVNTWGNPL4YKIWYECSOTS66SFLIDI3WU6RIJFNDNWPIMVHM \
  --source admin --network testnet \
  --network-passphrase "Test SDF Network ; September 2015" \
  -- distribute \
  --token CDLZFC3SYJYDZT7K67VZ75HPJVIEUVNIXF47ZG2FB2RMQQVU2HHGCYSC \
  --recipients '["GABC...", "GDEF..."]' \
  --total_amount 1000000000

# Event Distributor - Agregar participante
stellar contract invoke \
  --id CCHFGFX3S52UX46HZEBEGW5N2LDDYMSJDLZF4CQOZ6TSWWKFEG4TFPLS \
  --source admin --network testnet \
  --network-passphrase "Test SDF Network ; September 2015" \
  -- add_human \
  --address GABC... \
  --ipfs_hash '"QmTest123"'

# Event Distributor - Validar participante
stellar contract invoke \
  --id CCHFGFX3S52UX46HZEBEGW5N2LDDYMSJDLZF4CQOZ6TSWWKFEG4TFPLS \
  --source admin --network testnet \
  --network-passphrase "Test SDF Network ; September 2015" \
  -- update_human_validation \
  --address GABC... \
  --validated true

# Event Distributor - Crear evento
stellar contract invoke \
  --id CCHFGFX3S52UX46HZEBEGW5N2LDDYMSJDLZF4CQOZ6TSWWKFEG4TFPLS \
  --source admin --network testnet \
  --network-passphrase "Test SDF Network ; September 2015" \
  -- create_event \
  --event_id '"refi_ba_001"' \
  --location '"Buenos Aires"' \
  --pool 5000000000

# Event Distributor - Distribuir pool
stellar contract invoke \
  --id CCHFGFX3S52UX46HZEBEGW5N2LDDYMSJDLZF4CQOZ6TSWWKFEG4TFPLS \
  --source admin --network testnet \
  --network-passphrase "Test SDF Network ; September 2015" \
  -- distribute_event_pool \
  --event_id '"refi_ba_001"' \
  --token CDLZFC3SYJYDZT7K67VZ75HPJVIEUVNIXF47ZG2FB2RMQQVU2HHGCYSC
```

---

## 📝 Checklist de Integración

- [ ] Instalar @stellar/stellar-sdk
- [ ] Crear archivo de configuración con Contract IDs
- [ ] Implementar ContractClient service
- [ ] Crear componente WalletManager
- [ ] Crear componente EventManager
- [ ] Crear componente SimpleDistributor
- [ ] Integrar Freighter Wallet o similar
- [ ] Implementar manejo de errores
- [ ] Agregar loading states
- [ ] Testing con wallets de testnet
- [ ] Validar transacciones en Stellar Expert

---

## 🔧 Troubleshooting

**Error: Network passphrase missing**
```bash
# Agregar siempre: --network-passphrase "Test SDF Network ; September 2015"
```

**Error: Account not found**
```bash
# Fondear wallet con Friendbot
curl "https://friendbot.stellar.org?addr=G..."
```

**Error: Contract invocation failed**
```bash
# Verificar que eres admin
stellar contract invoke --id <CONTRACT> --source admin \
  --network testnet \
  --network-passphrase "Test SDF Network ; September 2015" \
  -- get_admin
```