# Frontend Integration Guide

Guía para consumir el contrato Vault Distributor desde el frontend.

## 📋 Información del Contrato

```javascript
const CONFIG = {
  contractId: "CC4XEZG3JIVNTWGNPL4YKIWYECSOTS66SFLIDI3WU6RIJFNDNWPIMVHM",
  network: "testnet",
  networkPassphrase: "Test SDF Network ; September 2015",
  rpcUrl: "https://soroban-testnet.stellar.org",
  
  // Vault RefiUp
  vaultContractId: "CA3N53CPBLSVM5342DZ25LK47WFQDS6R3BT62327SGZCNJ54CDDXO7KZ",
  vaultManager: "GAUSHAKPQJEHLKT4FBUVMWXOX3HUVV5OXNFUY54OBWS2R7ZUQY6QUBR6",
  
  // XLM Native Token
  xlmTokenId: "CDLZFC3SYJYDZT7K67VZ75HPJVIEUVNIXF47ZG2FB2RMQQVU2HHGCYSC"
};
```

## 🔧 Instalación

```bash
npm install @stellar/stellar-sdk
# o
yarn add @stellar/stellar-sdk
```

## 📖 Funciones del Contrato

### 1. `get_admin()` - Obtener Admin

**Descripción:** Retorna la dirección del administrador del contrato.

**Parámetros:** Ninguno

**Retorno:** `Address` - Dirección del admin

**Ejemplo:**

```javascript
import { Contract, SorobanRpc } from '@stellar/stellar-sdk';

async function getAdmin() {
  const server = new SorobanRpc.Server(CONFIG.rpcUrl);
  const contract = new Contract(CONFIG.contractId);
  
  const result = await server.simulateTransaction(
    new TransactionBuilder(account, {
      fee: "100",
      networkPassphrase: CONFIG.networkPassphrase
    })
    .addOperation(contract.call('get_admin'))
    .setTimeout(30)
    .build()
  );
  
  return result.result.retval;
}
```

### 2. `distribute()` - Distribuir Fondos

**Descripción:** Distribuye tokens equitativamente entre múltiples beneficiarios.

**Parámetros:**
- `token`: `Address` - Contract ID del token a distribuir
- `recipients`: `Vec<Address>` - Array de direcciones beneficiarias
- `total_amount`: `i128` - Monto total a distribuir (en stroops)

**Requiere:** Autenticación del admin

**Eventos emitidos:**
- `vault/distribtd`: `[total_amount, num_recipients]`

**Ejemplo:**

```javascript
import { 
  Contract, 
  SorobanRpc, 
  TransactionBuilder, 
  Keypair,
  Address,
  nativeToScVal,
  xdr
} from '@stellar/stellar-sdk';

async function distribute(adminKeypair, tokenId, recipients, totalAmount) {
  const server = new SorobanRpc.Server(CONFIG.rpcUrl);
  const contract = new Contract(CONFIG.contractId);
  
  // Cargar cuenta admin
  const adminAccount = await server.getAccount(adminKeypair.publicKey());
  
  // Convertir parámetros a ScVal
  const tokenAddress = new Address(tokenId).toScVal();
  const recipientsVec = nativeToScVal(
    recipients.map(addr => new Address(addr)),
    { type: "Vec<Address>" }
  );
  const amount = nativeToScVal(totalAmount, { type: "i128" });
  
  // Construir transacción
  const transaction = new TransactionBuilder(adminAccount, {
    fee: "1000000",
    networkPassphrase: CONFIG.networkPassphrase
  })
  .addOperation(
    contract.call(
      'distribute',
      tokenAddress,
      recipientsVec,
      amount
    )
  )
  .setTimeout(30)
  .build();
  
  // Simular para obtener footprint
  const simulated = await server.simulateTransaction(transaction);
  
  if (SorobanRpc.Api.isSimulationError(simulated)) {
    throw new Error(`Simulation failed: ${simulated.error}`);
  }
  
  // Preparar transacción con footprint
  const prepared = SorobanRpc.assembleTransaction(transaction, simulated);
  
  // Firmar
  prepared.sign(adminKeypair);
  
  // Enviar
  const sendResult = await server.sendTransaction(prepared);
  
  // Esperar confirmación
  let getResult = await server.getTransaction(sendResult.hash);
  while (getResult.status === "NOT_FOUND") {
    await new Promise(resolve => setTimeout(resolve, 1000));
    getResult = await server.getTransaction(sendResult.hash);
  }
  
  if (getResult.status === "SUCCESS") {
    return {
      success: true,
      hash: sendResult.hash,
      events: getResult.resultMetaXdr.v3().sorobanMeta().events()
    };
  }
  
  throw new Error(`Transaction failed: ${getResult.status}`);
}

// Uso
const adminKeypair = Keypair.fromSecret('S...');
const result = await distribute(
  adminKeypair,
  CONFIG.xlmTokenId,
  [
    'GC6JKKMVWJRVLILN2AILAXWAH3EI5QCZWCEGHKYHMCWE5P2I7EIFLOXV',
    'GC4MOFQ7N4LFPXHTSKBBAJ2IZYJMHHBFJBSKC7TP64J3SZA7XX23YQOG'
  ],
  500000000 // 50 XLM
);
```

## 🔐 Autenticación con Freighter Wallet

```javascript
import freighter from '@stellar/freighter-api';

async function connectWallet() {
  // Verificar si Freighter está instalado
  const isInstalled = await freighter.isConnected();
  
  if (!isInstalled) {
    throw new Error('Freighter wallet no está instalado');
  }
  
  // Obtener clave pública del usuario
  const publicKey = await freighter.getPublicKey();
  
  return publicKey;
}

async function signAndSendWithFreighter(transaction) {
  // Freighter firma automáticamente
  const signedXdr = await freighter.signTransaction(
    transaction.toXDR(),
    {
      network: CONFIG.networkPassphrase,
      accountToSign: await freighter.getPublicKey()
    }
  );
  
  // Enviar transacción firmada
  const server = new SorobanRpc.Server(CONFIG.rpcUrl);
  const tx = TransactionBuilder.fromXDR(signedXdr, CONFIG.networkPassphrase);
  
  return await server.sendTransaction(tx);
}
```

## 📊 Consultar Balance del Contrato

```javascript
async function getContractBalance(tokenId, contractId) {
  const server = new SorobanRpc.Server(CONFIG.rpcUrl);
  const tokenContract = new Contract(tokenId);
  
  // Crear cuenta dummy para simulación
  const dummyKeypair = Keypair.random();
  const dummyAccount = new Account(dummyKeypair.publicKey(), "0");
  
  const transaction = new TransactionBuilder(dummyAccount, {
    fee: "100",
    networkPassphrase: CONFIG.networkPassphrase
  })
  .addOperation(
    tokenContract.call(
      'balance',
      new Address(contractId).toScVal()
    )
  )
  .setTimeout(30)
  .build();
  
  const result = await server.simulateTransaction(transaction);
  
  if (SorobanRpc.Api.isSimulationSuccess(result)) {
    return scValToNative(result.result.retval);
  }
  
  throw new Error('Failed to get balance');
}

// Uso
const balance = await getContractBalance(
  CONFIG.xlmTokenId,
  CONFIG.contractId
);
console.log(`Balance: ${balance / 10000000} XLM`);
```

## 🎯 Integración Completa con React

```javascript
import { useState, useEffect } from 'react';
import { Contract, SorobanRpc, Address } from '@stellar/stellar-sdk';
import freighter from '@stellar/freighter-api';

function DistributorApp() {
  const [wallet, setWallet] = useState(null);
  const [isAdmin, setIsAdmin] = useState(false);
  const [balance, setBalance] = useState(0);
  
  useEffect(() => {
    checkAdmin();
    loadBalance();
  }, [wallet]);
  
  async function connectWallet() {
    try {
      const publicKey = await freighter.getPublicKey();
      setWallet(publicKey);
    } catch (error) {
      console.error('Error connecting wallet:', error);
    }
  }
  
  async function checkAdmin() {
    if (!wallet) return;
    
    try {
      const server = new SorobanRpc.Server(CONFIG.rpcUrl);
      const contract = new Contract(CONFIG.contractId);
      
      // Simular get_admin
      const dummyKeypair = Keypair.random();
      const dummyAccount = new Account(dummyKeypair.publicKey(), "0");
      
      const tx = new TransactionBuilder(dummyAccount, {
        fee: "100",
        networkPassphrase: CONFIG.networkPassphrase
      })
      .addOperation(contract.call('get_admin'))
      .setTimeout(30)
      .build();
      
      const result = await server.simulateTransaction(tx);
      const adminAddress = Address.fromScVal(result.result.retval).toString();
      
      setIsAdmin(adminAddress === wallet);
    } catch (error) {
      console.error('Error checking admin:', error);
    }
  }
  
  async function loadBalance() {
    try {
      const bal = await getContractBalance(
        CONFIG.xlmTokenId,
        CONFIG.contractId
      );
      setBalance(bal / 10000000);
    } catch (error) {
      console.error('Error loading balance:', error);
    }
  }
  
  async function handleDistribute(recipients, amount) {
    if (!isAdmin) {
      alert('Solo el admin puede distribuir');
      return;
    }
    
    try {
      const server = new SorobanRpc.Server(CONFIG.rpcUrl);
      const contract = new Contract(CONFIG.contractId);
      
      const account = await server.getAccount(wallet);
      
      // Construir transacción
      const tx = new TransactionBuilder(account, {
        fee: "1000000",
        networkPassphrase: CONFIG.networkPassphrase
      })
      .addOperation(
        contract.call(
          'distribute',
          new Address(CONFIG.xlmTokenId).toScVal(),
          nativeToScVal(recipients.map(r => new Address(r)), { type: "Vec<Address>" }),
          nativeToScVal(amount * 10000000, { type: "i128" })
        )
      )
      .setTimeout(30)
      .build();
      
      // Simular
      const simulated = await server.simulateTransaction(tx);
      const prepared = SorobanRpc.assembleTransaction(tx, simulated);
      
      // Firmar con Freighter
      const signedXdr = await freighter.signTransaction(
        prepared.toXDR(),
        { network: CONFIG.networkPassphrase }
      );
      
      // Enviar
      const signedTx = TransactionBuilder.fromXDR(
        signedXdr,
        CONFIG.networkPassphrase
      );
      
      const result = await server.sendTransaction(signedTx);
      
      alert(`Distribución exitosa! Hash: ${result.hash}`);
      loadBalance();
      
    } catch (error) {
      console.error('Error distributing:', error);
      alert('Error en la distribución');
    }
  }
  
  return (
    <div>
      <h1>Vault Distributor</h1>
      
      {!wallet ? (
        <button onClick={connectWallet}>Conectar Freighter</button>
      ) : (
        <>
          <p>Wallet: {wallet}</p>
          <p>Admin: {isAdmin ? '✅' : '❌'}</p>
          <p>Balance: {balance} XLM</p>
          
          {isAdmin && (
            <DistributeForm onSubmit={handleDistribute} />
          )}
        </>
      )}
    </div>
  );
}
```

## 🔍 Eventos y Logs

### Escuchar Eventos de Distribución

```javascript
async function getDistributionEvents(transactionHash) {
  const server = new SorobanRpc.Server(CONFIG.rpcUrl);
  const tx = await server.getTransaction(transactionHash);
  
  if (tx.status !== "SUCCESS") {
    return [];
  }
  
  const events = tx.resultMetaXdr.v3().sorobanMeta().events();
  
  const distributionEvents = events.filter(event => {
    const topics = event.body().v0().topics();
    return topics.length === 2 && 
           scValToNative(topics[0]) === "vault" &&
           scValToNative(topics[1]) === "distribtd";
  });
  
  return distributionEvents.map(event => {
    const data = scValToNative(event.body().v0().data());
    return {
      totalAmount: data[0],
      numRecipients: data[1]
    };
  });
}
```

## 📝 Validaciones del Frontend

```javascript
function validateDistribution(recipients, totalAmount) {
  const errors = [];
  
  // Validar recipients no vacío
  if (!recipients || recipients.length === 0) {
    errors.push('Debe haber al menos un beneficiario');
  }
  
  // Validar direcciones válidas
  recipients.forEach((addr, index) => {
    if (!addr.match(/^G[A-Z0-9]{55}$/)) {
      errors.push(`Dirección ${index + 1} inválida`);
    }
  });
  
  // Validar monto positivo
  if (totalAmount <= 0) {
    errors.push('El monto debe ser mayor a 0');
  }
  
  // Validar que el monto sea divisible
  if (totalAmount % recipients.length !== 0) {
    errors.push('El monto debe ser divisible equitativamente');
  }
  
  return errors;
}
```

## 🌐 URLs Útiles

**Stellar Explorer:**
- Contrato: https://stellar.expert/explorer/testnet/contract/CC4XEZG3JIVNTWGNPL4YKIWYECSOTS66SFLIDI3WU6RIJFNDNWPIMVHM
- Vault: https://stellar.expert/explorer/testnet/contract/CA3N53CPBLSVM5342DZ25LK47WFQDS6R3BT62327SGZCNJ54CDDXO7KZ

**RPC Endpoint:** https://soroban-testnet.stellar.org

**Friendbot (para testnet):** https://friendbot.stellar.org/?addr={ADDRESS}

## ⚠️ Notas Importantes

1. **Stroops vs XLM:** 1 XLM = 10,000,000 stroops. Siempre convertir antes de enviar.

2. **Gas Fees:** Las transacciones Soroban requieren más fee que operaciones normales. Usar mínimo 1,000,000 stroops.

3. **Auth Requirement:** `distribute()` requiere firma del admin. El frontend debe verificar primero con `get_admin()`.

4. **División exacta:** El contrato valida que `total_amount % recipients.length === 0`.

5. **Simulación primero:** Siempre simular transacciones antes de enviar para obtener el footprint correcto.
