# 🚀 Frontend Quick Start - Event Distributor

## TL;DR - Lo Esencial

**Tu frontend solo necesita:**
1. Imagen en Base64
2. Dirección pública del usuario (Stellar Address)

**Entonces hace:**
1. Sube Base64 → IPFS → Obtiene hash `QmXxx...`
2. Llama contrato con: `add_human(address, ipfs_hash)`

---

## 📦 Contract Info

```bash
CONTRACT_ID=CCHFGFX3S52UX46HZEBEGW5N2LDDYMSJDLZF4CQOZ6TSWWKFEG4TFPLS
NETWORK=testnet
RPC_URL=https://soroban-testnet.stellar.org
```

---

## 🎯 3 Pasos Principales

### 1️⃣ Conectar Wallet

```javascript
import { connectWallet } from './services/stellarService';

const address = await connectWallet(); // Retorna "GXXX..."
```

### 2️⃣ Subir Imagen a IPFS

```javascript
import { uploadImageToIPFS } from './services/ipfsService';

// Tu imagen en Base64: "data:image/jpeg;base64,/9j/4AAQ..."
const ipfsHash = await uploadImageToIPFS(imageBase64);
// Retorna: "QmXxx..."
```

### 3️⃣ Llamar al Contrato

```javascript
import { addHumanToContract } from './services/stellarService';

const txHash = await addHumanToContract(userAddress, ipfsHash);
// Retorna hash de transacción
```

---

## 💻 Instalación Rápida

```bash
npm install @stellar/stellar-sdk axios

# Variables de entorno (.env.local)
REACT_APP_CONTRACT_ID=CCHFGFX3S52UX46HZEBEGW5N2LDDYMSJDLZF4CQOZ6TSWWKFEG4TFPLS
REACT_APP_PINATA_API_KEY=tu_api_key
REACT_APP_PINATA_SECRET_KEY=tu_secret_key
```

---

## 📝 Código Mínimo Funcional

```jsx
import React, { useState } from 'react';
import { connectWallet, addHumanToContract } from './services/stellarService';
import { uploadImageToIPFS } from './services/ipfsService';

function App() {
  const [address, setAddress] = useState('');
  const [image, setImage] = useState('');

  const handleConnect = async () => {
    const addr = await connectWallet();
    setAddress(addr);
  };

  const handleUpload = (e) => {
    const file = e.target.files[0];
    const reader = new FileReader();
    reader.onloadend = () => setImage(reader.result);
    reader.readAsDataURL(file);
  };

  const handleSubmit = async () => {
    const ipfsHash = await uploadImageToIPFS(image);
    const tx = await addHumanToContract(address, ipfsHash);
    alert(`✅ Success! TX: ${tx}`);
  };

  return (
    <div>
      <button onClick={handleConnect}>Conectar Wallet</button>
      <input type="file" onChange={handleUpload} />
      <button onClick={handleSubmit}>Enviar</button>
    </div>
  );
}
```

---

## 🔗 Links Importantes

📖 **Documentación Completa:** [`docs/FRONTEND_INTEGRATION.md`](./docs/FRONTEND_INTEGRATION.md)  
🔍 **Explorer:** https://stellar.expert/explorer/testnet/contract/CCHFGFX3S52UX46HZEBEGW5N2LDDYMSJDLZF4CQOZ6TSWWKFEG4TFPLS  
💾 **Código Completo:** Ver `docs/FRONTEND_INTEGRATION.md` para:
- Servicios completos (IPFS + Stellar)
- Componentes React listos
- Manejo de errores
- Testing

---

## 🧪 Testing Rápido

**6 participantes ya registrados en testnet:**
- Alice (validada) ✅
- Bob (validado) ✅  
- Carol ❌
- David (validado) ✅
- Emma ❌

**Ver todos:**
```bash
stellar contract invoke \
  --id CCHFGFX3S52UX46HZEBEGW5N2LDDYMSJDLZF4CQOZ6TSWWKFEG4TFPLS \
  --source admin \
  --network testnet \
  -- get_all_humans --start 0 --limit 10
```

---

## ❓ FAQ Rápido

**Q: ¿Qué formato debe tener la imagen?**  
A: Base64 con prefijo `data:image/jpeg;base64,` o sin prefijo

**Q: ¿Puedo actualizar la imagen después?**  
A: Sí, usa `update_human_image(address, new_ipfs_hash)`

**Q: ¿Necesito fondos en testnet?**  
A: Sí, obtén XLM gratis: https://laboratory.stellar.org/#account-creator?network=test

**Q: ¿Funciona en mobile?**  
A: Sí, usa `<input type="file" capture="environment" />`

---

## 🚨 Errores Comunes

| Error | Solución |
|-------|----------|
| "Freighter not installed" | Instalar [Freighter Wallet](https://www.freighter.app/) |
| "Insufficient funds" | Usar [Friendbot](https://laboratory.stellar.org/#account-creator?network=test) |
| "Human already exists" | Cada address solo se registra 1 vez |
| "IPFS timeout" | Reducir tamaño de imagen (max 800px) |

---

## 📦 Archivos de Servicio

Copia estos 2 archivos a tu proyecto (código completo en `docs/FRONTEND_INTEGRATION.md`):

1. `services/ipfsService.js` - Subir Base64 a IPFS
2. `services/stellarService.js` - Conectar wallet y llamar contrato

---

**✨ ¡Todo listo! Para más detalles ver [`docs/FRONTEND_INTEGRATION.md`](./docs/FRONTEND_INTEGRATION.md)**
