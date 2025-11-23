# 🚀 Frontend Integration Guide - Event Distributor

## 📋 Resumen Ejecutivo

Este documento proporciona **TODO** lo necesario para que el frontend integre el contrato **Event Distributor** desplegado en Stellar Testnet.

**Flujo Principal del Frontend:**
1. Usuario toma una foto o sube una imagen → **Imagen en Base64**
2. Frontend sube la imagen Base64 a **IPFS** (Pinata/Lighthouse)
3. Frontend obtiene el **IPFS Hash** (ej: `QmXxx...`)
4. Frontend llama al contrato con: **Public Address** + **IPFS Hash**

---

## 🔗 Contrato Desplegado

```bash
# Event Distributor Contract
CONTRACT_ID=CCHFGFX3S52UX46HZEBEGW5N2LDDYMSJDLZF4CQOZ6TSWWKFEG4TFPLS
NETWORK=testnet
NETWORK_PASSPHRASE="Test SDF Network ; September 2015"

# Explorer URLs
Contract: https://stellar.expert/explorer/testnet/contract/CCHFGFX3S52UX46HZEBEGW5N2LDDYMSJDLZF4CQOZ6TSWWKFEG4TFPLS
```

---

## 📦 Instalación de Dependencias

```bash
npm install @stellar/stellar-sdk axios
# O con yarn
yarn add @stellar/stellar-sdk axios
```

---

## 🎯 Flujo Completo: De Imagen Base64 a Blockchain

### Paso 1: Capturar Imagen (Frontend)

```jsx
// React Component: Capturar imagen de cámara o upload
import React, { useState } from 'react';

function ImageCapture() {
  const [imageBase64, setImageBase64] = useState('');

  const handleFileUpload = (e) => {
    const file = e.target.files[0];
    const reader = new FileReader();
    
    reader.onloadend = () => {
      // Resultado: "data:image/jpeg;base64,/9j/4AAQSkZJRg..."
      setImageBase64(reader.result);
    };
    
    reader.readAsDataURL(file);
  };

  const handleCameraCapture = async () => {
    const stream = await navigator.mediaDevices.getUserMedia({ video: true });
    const video = document.createElement('video');
    video.srcObject = stream;
    await video.play();
    
    const canvas = document.createElement('canvas');
    canvas.width = video.videoWidth;
    canvas.height = video.videoHeight;
    canvas.getContext('2d').drawImage(video, 0, 0);
    
    const base64 = canvas.toDataURL('image/jpeg');
    setImageBase64(base64);
    
    stream.getTracks().forEach(track => track.stop());
  };

  return (
    <div>
      <input type="file" accept="image/*" onChange={handleFileUpload} />
      <button onClick={handleCameraCapture}>📸 Tomar Foto</button>
      
      {imageBase64 && (
        <img src={imageBase64} alt="Preview" style={{maxWidth: '300px'}} />
      )}
    </div>
  );
}
```

### Paso 2: Subir Base64 a IPFS

```javascript
// services/ipfsService.js
import axios from 'axios';

const PINATA_API_KEY = 'tu_pinata_api_key';
const PINATA_SECRET_KEY = 'tu_pinata_secret_key';

/**
 * Convierte Base64 a archivo Blob
 */
function base64ToBlob(base64String) {
  // Remover el prefijo "data:image/jpeg;base64,"
  const base64Data = base64String.split(',')[1];
  const byteCharacters = atob(base64Data);
  const byteNumbers = new Array(byteCharacters.length);
  
  for (let i = 0; i < byteCharacters.length; i++) {
    byteNumbers[i] = byteCharacters.charCodeAt(i);
  }
  
  const byteArray = new Uint8Array(byteNumbers);
  return new Blob([byteArray], { type: 'image/jpeg' });
}

/**
 * Sube imagen Base64 a IPFS vía Pinata
 * @param {string} base64Image - Imagen en formato "data:image/jpeg;base64,..."
 * @param {string} filename - Nombre del archivo (opcional)
 * @returns {Promise<string>} - IPFS Hash (ej: "QmXxx...")
 */
export async function uploadImageToIPFS(base64Image, filename = 'user-photo.jpg') {
  const blob = base64ToBlob(base64Image);
  const formData = new FormData();
  formData.append('file', blob, filename);

  const metadata = JSON.stringify({
    name: filename,
    keyvalues: {
      uploaded_by: 'refi-universe',
      timestamp: new Date().toISOString()
    }
  });
  formData.append('pinataMetadata', metadata);

  try {
    const response = await axios.post(
      'https://api.pinata.cloud/pinning/pinFileToIPFS',
      formData,
      {
        headers: {
          'Content-Type': 'multipart/form-data',
          'pinata_api_key': PINATA_API_KEY,
          'pinata_secret_api_key': PINATA_SECRET_KEY
        }
      }
    );

    // Retorna el hash IPFS: "QmXxx..."
    return response.data.IpfsHash;
  } catch (error) {
    console.error('Error uploading to IPFS:', error);
    throw new Error('Failed to upload image to IPFS');
  }
}

/**
 * Alternativa con Lighthouse (más económico)
 */
export async function uploadToLighthouse(base64Image) {
  const LIGHTHOUSE_API_KEY = 'tu_lighthouse_api_key';
  
  const blob = base64ToBlob(base64Image);
  const formData = new FormData();
  formData.append('file', blob);

  const response = await axios.post(
    'https://node.lighthouse.storage/api/v0/add',
    formData,
    {
      headers: {
        'Authorization': `Bearer ${LIGHTHOUSE_API_KEY}`,
        'Content-Type': 'multipart/form-data'
      }
    }
  );

  return response.data.Hash;
}
```

### Paso 3: Conectar Wallet y Enviar a Contrato

```javascript
// services/stellarService.js
import {
  SorobanRpc,
  Contract,
  TransactionBuilder,
  Networks,
  BASE_FEE,
  nativeToScVal,
  Address,
  xdr
} from '@stellar/stellar-sdk';

const CONTRACT_ID = 'CCHFGFX3S52UX46HZEBEGW5N2LDDYMSJDLZF4CQOZ6TSWWKFEG4TFPLS';
const NETWORK_PASSPHRASE = Networks.TESTNET;
const RPC_URL = 'https://soroban-testnet.stellar.org';

const server = new SorobanRpc.Server(RPC_URL);

/**
 * Conectar wallet usando Freighter
 */
export async function connectWallet() {
  if (!window.freighter) {
    throw new Error('Freighter wallet no está instalado');
  }

  const publicKey = await window.freighter.getPublicKey();
  return publicKey;
}

/**
 * Agregar humano al contrato
 * @param {string} userAddress - Dirección pública del usuario (ej: "GXXX...")
 * @param {string} ipfsHash - Hash IPFS de la imagen (ej: "QmXxx...")
 * @returns {Promise<string>} - Transaction hash
 */
export async function addHumanToContract(userAddress, ipfsHash) {
  // 1. Obtener account del usuario
  const sourceAccount = await server.getAccount(userAddress);

  // 2. Crear contrato
  const contract = new Contract(CONTRACT_ID);

  // 3. Preparar parámetros
  const addressParam = new Address(userAddress).toScVal();
  const ipfsHashParam = nativeToScVal(ipfsHash, { type: 'string' });

  // 4. Construir operación
  const transaction = new TransactionBuilder(sourceAccount, {
    fee: BASE_FEE,
    networkPassphrase: NETWORK_PASSPHRASE
  })
    .addOperation(
      contract.call(
        'add_human',
        addressParam,
        ipfsHashParam
      )
    )
    .setTimeout(180)
    .build();

  // 5. Simular transacción
  const preparedTx = await server.prepareTransaction(transaction);

  // 6. Firmar con Freighter
  const signedTx = await window.freighter.signTransaction(
    preparedTx.toXDR(),
    {
      network: 'TESTNET',
      networkPassphrase: NETWORK_PASSPHRASE
    }
  );

  // 7. Enviar transacción
  const txFromXDR = TransactionBuilder.fromXDR(signedTx, NETWORK_PASSPHRASE);
  const sendResponse = await server.sendTransaction(txFromXDR);

  // 8. Esperar confirmación
  let txStatus = await server.getTransaction(sendResponse.hash);
  while (txStatus.status === 'NOT_FOUND') {
    await new Promise(resolve => setTimeout(resolve, 1000));
    txStatus = await server.getTransaction(sendResponse.hash);
  }

  if (txStatus.status === 'SUCCESS') {
    return sendResponse.hash;
  } else {
    throw new Error(`Transaction failed: ${txStatus.status}`);
  }
}

/**
 * Obtener información de un humano
 */
export async function getHuman(address) {
  const contract = new Contract(CONTRACT_ID);
  const sourceAccount = await server.getAccount(address);

  const transaction = new TransactionBuilder(sourceAccount, {
    fee: BASE_FEE,
    networkPassphrase: NETWORK_PASSPHRASE
  })
    .addOperation(
      contract.call('get_human', new Address(address).toScVal())
    )
    .setTimeout(180)
    .build();

  const result = await server.simulateTransaction(transaction);
  
  if (result.results?.[0]?.retval) {
    const scVal = result.results[0].retval;
    // Parsear resultado
    return {
      address: scVal.address().toString(),
      ipfsHash: scVal.ipfs_hash().toString(),
      validated: scVal.validated()
    };
  }
  
  return null;
}

/**
 * Obtener todos los humanos (paginado)
 */
export async function getAllHumans(start = 0, limit = 20) {
  const contract = new Contract(CONTRACT_ID);
  const dummyAccount = await server.getAccount(
    'GAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAWHF'
  );

  const transaction = new TransactionBuilder(dummyAccount, {
    fee: BASE_FEE,
    networkPassphrase: NETWORK_PASSPHRASE
  })
    .addOperation(
      contract.call(
        'get_all_humans',
        nativeToScVal(start, { type: 'u32' }),
        nativeToScVal(limit, { type: 'u32' })
      )
    )
    .setTimeout(180)
    .build();

  const result = await server.simulateTransaction(transaction);
  
  if (result.results?.[0]?.retval) {
    const vec = result.results[0].retval;
    return vec.vec().map(item => ({
      address: item.address().toString(),
      ipfsHash: item.ipfs_hash().toString(),
      validated: item.validated()
    }));
  }
  
  return [];
}
```

### Paso 4: Componente React Completo

```jsx
// components/HumanRegistration.jsx
import React, { useState } from 'react';
import { uploadImageToIPFS } from '../services/ipfsService';
import { connectWallet, addHumanToContract } from '../services/stellarService';

function HumanRegistration() {
  const [imageBase64, setImageBase64] = useState('');
  const [walletAddress, setWalletAddress] = useState('');
  const [loading, setLoading] = useState(false);
  const [status, setStatus] = useState('');

  const handleConnect = async () => {
    try {
      const address = await connectWallet();
      setWalletAddress(address);
      setStatus(`✅ Wallet conectada: ${address.slice(0, 8)}...`);
    } catch (error) {
      setStatus(`❌ Error: ${error.message}`);
    }
  };

  const handleImageUpload = (e) => {
    const file = e.target.files[0];
    const reader = new FileReader();
    reader.onloadend = () => setImageBase64(reader.result);
    reader.readAsDataURL(file);
  };

  const handleSubmit = async () => {
    if (!walletAddress) {
      setStatus('❌ Primero conecta tu wallet');
      return;
    }
    
    if (!imageBase64) {
      setStatus('❌ Selecciona una imagen');
      return;
    }

    try {
      setLoading(true);
      setStatus('📤 Subiendo imagen a IPFS...');
      
      // 1. Subir a IPFS
      const ipfsHash = await uploadImageToIPFS(imageBase64);
      setStatus(`✅ Imagen en IPFS: ${ipfsHash}`);
      
      // 2. Agregar al contrato
      setStatus('⏳ Enviando transacción al contrato...');
      const txHash = await addHumanToContract(walletAddress, ipfsHash);
      
      setStatus(`🎉 ¡Éxito! TX: ${txHash.slice(0, 16)}...`);
      setImageBase64('');
      
    } catch (error) {
      setStatus(`❌ Error: ${error.message}`);
    } finally {
      setLoading(false);
    }
  };

  return (
    <div style={{padding: '20px', maxWidth: '600px', margin: '0 auto'}}>
      <h1>🌍 ReFi Universe - Registro</h1>
      
      {!walletAddress ? (
        <button onClick={handleConnect} style={{padding: '10px 20px', fontSize: '16px'}}>
          🔗 Conectar Wallet
        </button>
      ) : (
        <p>✅ Conectado: {walletAddress.slice(0, 8)}...{walletAddress.slice(-4)}</p>
      )}

      <div style={{marginTop: '20px'}}>
        <input 
          type="file" 
          accept="image/*" 
          onChange={handleImageUpload}
          disabled={!walletAddress}
        />
        
        {imageBase64 && (
          <div style={{marginTop: '10px'}}>
            <img src={imageBase64} alt="Preview" style={{maxWidth: '100%', borderRadius: '8px'}} />
          </div>
        )}
      </div>

      <button 
        onClick={handleSubmit}
        disabled={loading || !walletAddress || !imageBase64}
        style={{
          marginTop: '20px',
          padding: '15px 30px',
          fontSize: '16px',
          backgroundColor: loading ? '#ccc' : '#0066ff',
          color: 'white',
          border: 'none',
          borderRadius: '8px',
          cursor: loading ? 'not-allowed' : 'pointer'
        }}
      >
        {loading ? '⏳ Procesando...' : '🚀 Registrar en Blockchain'}
      </button>

      {status && (
        <div style={{
          marginTop: '20px',
          padding: '10px',
          backgroundColor: status.includes('❌') ? '#ffebee' : '#e8f5e9',
          borderRadius: '8px'
        }}>
          {status}
        </div>
      )}
    </div>
  );
}

export default HumanRegistration;
```

---

## 🔧 Variables de Entorno

```env
# .env.local
REACT_APP_CONTRACT_ID=CCHFGFX3S52UX46HZEBEGW5N2LDDYMSJDLZF4CQOZ6TSWWKFEG4TFPLS
REACT_APP_NETWORK=testnet
REACT_APP_RPC_URL=https://soroban-testnet.stellar.org

# IPFS (Pinata)
REACT_APP_PINATA_API_KEY=tu_api_key
REACT_APP_PINATA_SECRET_KEY=tu_secret_key

# IPFS (Lighthouse - Alternativa)
REACT_APP_LIGHTHOUSE_API_KEY=tu_lighthouse_key
```

---

## 📊 Funciones del Contrato Disponibles

### 1. `add_human(address: Address, ipfs_hash: String)`
**Descripción:** Agrega un nuevo participante con su imagen  
**Requiere:** Admin authentication  
**Eventos:** `human_add`

**Ejemplo CLI:**
```bash
stellar contract invoke \
  --id CCHFGFX3S52UX46HZEBEGW5N2LDDYMSJDLZF4CQOZ6TSWWKFEG4TFPLS \
  --source admin \
  --network testnet \
  -- add_human \
  --address GBFPNPXPCH6D5XQ73M2POBVJXKE7Z3X57EFVJCKC2MB6YPDHV6P2LJJF \
  --ipfs_hash '"QmExampleHash123"'
```

### 2. `update_human_validation(address: Address, validated: bool)`
**Descripción:** Marca un participante como validado  
**Requiere:** Admin authentication  
**Eventos:** `human_val`

### 3. `update_human_image(address: Address, ipfs_hash: String)`
**Descripción:** Actualiza la imagen de un participante  
**Requiere:** Admin authentication  
**Eventos:** `human_img`

### 4. `get_human(address: Address) -> Human`
**Descripción:** Obtiene información de un participante  
**Read-only:** Sí

**Respuesta:**
```json
{
  "address": "GBFPNPXPCH6D5XQ73M2POBVJXKE7Z3X57EFVJCKC2MB6YPDHV6P2LJJF",
  "ipfs_hash": "QmExampleHash123",
  "validated": true
}
```

### 5. `get_all_humans(start: u32, limit: u32) -> Vec<Human>`
**Descripción:** Obtiene lista paginada de participantes  
**Read-only:** Sí

**Ejemplo:**
```bash
stellar contract invoke \
  --id CCHFGFX3S52UX46HZEBEGW5N2LDDYMSJDLZF4CQOZ6TSWWKFEG4TFPLS \
  --source admin \
  --network testnet \
  -- get_all_humans \
  --start 0 \
  --limit 20
```

**Respuesta:**
```json
[
  {
    "address": "GBFPNPXPCH6D5XQ73M2POBVJXKE7Z3X57EFVJCKC2MB6YPDHV6P2LJJF",
    "ipfs_hash": "QmAliceUpdatedPhoto999xyz",
    "validated": true
  },
  {
    "address": "GB5SLLMBKBK7TOQLEIX75ZSMVZLKEZK3FLL6E2FLYCDKOAAQHTG3Y6AE",
    "ipfs_hash": "QmBobAvatarPhoto456def",
    "validated": true
  }
]
```

### 6. `get_validated_humans() -> Vec<Address>`
**Descripción:** Obtiene solo participantes validados  
**Read-only:** Sí

---

## 🧪 Testing en Testnet

El contrato ya tiene **6 participantes registrados** que puedes usar para pruebas:

| Nombre | Address | IPFS Hash | Validado |
|--------|---------|-----------|----------|
| Admin Test | `GDL5432N2J...` | `QmTest123Example` | ✅ |
| Alice | `GBFPNPXPCH...` | `QmAliceUpdatedPhoto999xyz` | ✅ |
| Bob | `GB5SLLMBKB...` | `QmBobAvatarPhoto456def` | ✅ |
| Carol | `GANGFTMSOJ...` | `QmCarolSelfie789ghi` | ❌ |
| David | `GBW6LPSCSP...` | `QmDavidPicture321jkl` | ✅ |
| Emma | `GA3VK7Y7SC...` | `QmEmmaPhoto654mno` | ❌ |

---

## 🎨 UI/UX Recomendaciones

### Flujo Ideal
1. **Pantalla inicial:** Botón "Conectar Wallet"
2. **Pantalla de captura:** Cámara o upload de foto
3. **Preview:** Mostrar imagen antes de enviar
4. **Loading states:**
   - ⏳ Subiendo a IPFS...
   - ⏳ Enviando a blockchain...
5. **Confirmación:** Mostrar transaction hash y link a explorer

### Loading States
```jsx
const LoadingStates = {
  IDLE: 'idle',
  UPLOADING_IPFS: 'uploading_ipfs',
  SENDING_TX: 'sending_tx',
  CONFIRMING: 'confirming',
  SUCCESS: 'success',
  ERROR: 'error'
};
```

### Error Handling
```javascript
const handleError = (error) => {
  if (error.message.includes('Freighter')) {
    return '❌ Instala Freighter Wallet';
  }
  if (error.message.includes('IPFS')) {
    return '❌ Error subiendo imagen. Intenta de nuevo';
  }
  if (error.message.includes('Transaction failed')) {
    return '❌ Transacción rechazada. Verifica que tienes fondos';
  }
  return `❌ Error: ${error.message}`;
};
```

---

## 🔍 Verificación en Stellar Explorer

Después de cada transacción, puedes verificar:

**Contract Explorer:**
```
https://stellar.expert/explorer/testnet/contract/CCHFGFX3S52UX46HZEBEGW5N2LDDYMSJDLZF4CQOZ6TSWWKFEG4TFPLS
```

**Transaction Explorer:**
```
https://stellar.expert/explorer/testnet/tx/[TX_HASH]
```

---

## 📱 Soporte Mobile

Para PWA o mobile web:

```javascript
// Detectar mobile
const isMobile = /iPhone|iPad|iPod|Android/i.test(navigator.userAgent);

// Captura desde cámara mobile
<input 
  type="file" 
  accept="image/*" 
  capture="environment"  // Cámara trasera
  onChange={handleImageUpload}
/>
```

---

## 🚨 Troubleshooting

### Error: "Freighter not installed"
**Solución:** Instalar [Freighter Wallet](https://www.freighter.app/)

### Error: "Transaction failed - insufficient funds"
**Solución:** Fondear cuenta en [Friendbot](https://laboratory.stellar.org/#account-creator?network=test)

### Error: "IPFS upload timeout"
**Solución:** Reducir tamaño de imagen antes de subir:
```javascript
function resizeImage(base64, maxWidth = 800) {
  return new Promise((resolve) => {
    const img = new Image();
    img.onload = () => {
      const canvas = document.createElement('canvas');
      const ratio = maxWidth / img.width;
      canvas.width = maxWidth;
      canvas.height = img.height * ratio;
      canvas.getContext('2d').drawImage(img, 0, 0, canvas.width, canvas.height);
      resolve(canvas.toDataURL('image/jpeg', 0.8));
    };
    img.src = base64;
  });
}
```

### Error: "Human already exists"
**Solución:** Cada address solo puede registrarse una vez. Usar `update_human_image` para cambiar foto.

---

## 📚 Recursos Adicionales

- [Stellar SDK Documentation](https://stellar.github.io/js-stellar-sdk/)
- [Soroban Docs](https://soroban.stellar.org/docs)
- [Freighter Wallet Docs](https://docs.freighter.app/)
- [Pinata IPFS Docs](https://docs.pinata.cloud/)
- [Contract Source Code](https://github.com/refiup/sc)

---

## 🎯 Quick Start Checklist

- [ ] Instalar dependencias: `npm install @stellar/stellar-sdk axios`
- [ ] Configurar variables de entorno (`.env.local`)
- [ ] Obtener API keys de Pinata o Lighthouse
- [ ] Instalar Freighter Wallet
- [ ] Fondear cuenta con [Friendbot](https://laboratory.stellar.org/#account-creator?network=test)
- [ ] Copiar código de `services/ipfsService.js`
- [ ] Copiar código de `services/stellarService.js`
- [ ] Implementar componente `HumanRegistration.jsx`
- [ ] Probar flujo completo en testnet

---

## 💡 Tips de Optimización

1. **Comprimir imágenes antes de IPFS** → Reduce costos
2. **Cache de datos on-chain** → Evita múltiples llamadas RPC
3. **Batch operations** → Agregar múltiples humanos en una transacción
4. **Error retry logic** → Network issues son comunes
5. **Transaction polling** → No esperar más de 30 segundos

---

## 📞 Soporte

**Preguntas o issues:** [GitHub Issues](https://github.com/refiup/sc/issues)  
**Contract ID:** `CCHFGFX3S52UX46HZEBEGW5N2LDDYMSJDLZF4CQOZ6TSWWKFEG4TFPLS`  
**Network:** Stellar Testnet

---

**¡Listo para integrar! 🚀**
