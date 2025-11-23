# 🎨 UI Integration Guides - Celo

Guías completas para integrar el frontend con los smart contracts de ReFi Universe desplegados en **Celo**.

---

## 📚 Guías Disponibles

### 📱 [GUIDE_INTEGRATION_APP.md](GUIDE_INTEGRATION_APP.md)
**Para: App de Usuario (Participantes)**

Flujo simplificado para que los usuarios:
- 📸 Capturen su foto (cámara) o suban archivo
- 📋 Ingresen su dirección de wallet Celo
- ✅ Envíen datos al backend
- 🔍 Verifiquen su estado de validación

**El frontend solo maneja:**
- Captura/upload de imagen
- Conversión de imagen a Base64
- Formulario con dirección de wallet
- Envío al backend

---

### 👨‍⚖️ [GUIDE_INTEGRATION_JUDGE.md](GUIDE_INTEGRATION_JUDGE.md)
**Para: App del Juez (Administrador)**

Dashboard para que el juez:
- 👥 Vea galería de participantes (imagen Base64 + dirección)
- ✅ Valide o rechace fotos
- 🎪 Cree eventos
- 💰 Distribuya fondos a direcciones validadas
- 📊 Monitoree estadísticas

**El backend maneja:**
- Conexión con el contrato Celo
- Registro de usuarios en blockchain
- Validación de participantes
- Distribución de recompensas

---

## 🌐 Arquitectura Simplificada

### Frontend (Usuario)
```
1. Usuario captura foto o sube archivo
2. Frontend convierte imagen a Base64
3. Usuario ingresa su dirección de wallet Celo
4. Frontend envía {imageBase64, walletAddress} al backend
5. Backend procesa y registra en blockchain
```

### Backend (Admin)
```
1. Recibe {imageBase64, walletAddress}
2. Conecta con contrato Celo vía ethers.js
3. Ejecuta: addHuman(walletAddress, imageBase64)
4. Juez valida: updateHumanValidation(walletAddress, true/false)
5. Distribución: distributeToEvent(eventId, token, amount)
```

---

## 🚀 Quick Start

### Frontend (Usuario)

**1. Captura de Imagen:**
```html
<!-- Opción 1: Tomar foto con cámara -->
<input type="file" accept="image/*" capture="camera" id="camera-input" />

<!-- Opción 2: Subir archivo -->
<input type="file" accept="image/*" id="file-input" />
```

**2. Convertir a Base64:**
```javascript
function convertToBase64(file) {
  return new Promise((resolve, reject) => {
    const reader = new FileReader();
    reader.onload = () => resolve(reader.result);
    reader.onerror = reject;
    reader.readAsDataURL(file);
  });
}

// Uso
const file = event.target.files[0];
const imageBase64 = await convertToBase64(file);
// Resultado: "data:image/jpeg;base64,/9j/4AAQSkZJRg..."
```

**3. Formulario de Dirección:**
```html
<input 
  type="text" 
  placeholder="0x..." 
  pattern="0x[a-fA-F0-9]{40}"
  required 
/>
```

**4. Envío al Backend:**
```javascript
const response = await fetch('/api/register', {
  method: 'POST',
  headers: { 'Content-Type': 'application/json' },
  body: JSON.stringify({
    imageBase64: imageBase64,
    walletAddress: walletAddress
  })
});
```

### Backend (Node.js + ethers.js)

**Variables de Entorno:**
```env
# .env
CELO_RPC_URL=https://alfajores-forno.celo-testnet.org
CELO_CHAIN_ID=44787
CONTRACT_ADDRESS=0x...
ADMIN_PRIVATE_KEY=0x...
```

**Registro en Blockchain:**
```javascript
const ethers = require('ethers');

// Conectar al contrato
const provider = new ethers.JsonRpcProvider(process.env.CELO_RPC_URL);
const wallet = new ethers.Wallet(process.env.ADMIN_PRIVATE_KEY, provider);
const contract = new ethers.Contract(CONTRACT_ADDRESS, ABI, wallet);

// Registrar usuario
async function registerUser(walletAddress, imageBase64) {
  const tx = await contract.addHuman(walletAddress, imageBase64);
  await tx.wait();
  return tx.hash;
}
```

---

## 📊 Contract Functions

### Para Usuarios
| Función | Descripción | Gas Estimado |
|---------|-------------|--------------|
| `getHuman(address)` | Ver info | Read-only |
| `getAllHumans(start, limit)` | Lista paginada | Read-only |

### Para Admin
| Función | Descripción | Gas Estimado |
|---------|-------------|--------------|
| `addHuman(addr, ipfs)` | Agregar participante | ~100k gas |
| `updateHumanValidation(addr, bool)` | Validar | ~50k gas |
| `updateHumanImage(addr, ipfs)` | Cambiar imagen | ~45k gas |
| `createEvent(name, addrs[])` | Crear evento | ~150k + 20k per addr |
| `distributeToEvent(id, token, amount)` | Distribuir | ~50k per recipient |

**Gas Price:** ~0.5 Gwei (muy bajo en Celo)  
**Costos:** ~$0.001 - $0.01 USD por transacción

---

## 🎯 Flujos de Usuario

### Usuario (Participante)
```
1. Abrir App
2. Capturar foto o subir imagen
3. Frontend convierte imagen a Base64
4. Ingresar dirección de wallet Celo (0x...)
5. Enviar formulario
6. Backend registra en blockchain
7. Esperar validación del juez
8. Recibir recompensas en la dirección proporcionada
```

### Juez (Admin)
```
1. Ver lista de participantes (imagen + dirección)
2. Revisar fotos
3. Validar (✅) o rechazar (❌) cada participante
4. Crear evento con participantes validados
5. Distribuir fondos a direcciones validadas
6. Los usuarios reciben CELO/tokens en sus wallets
```

---

## 💰 Tokens Soportados

### Celo Native
```javascript
// Distribuir CELO nativo
const token = '0x0000000000000000000000000000000000000000'; // address(0)
await distributeToEvent(eventId, token, amountInWei);
```

### cUSD (Celo Dollar)
```javascript
// Alfajores cUSD
const cUSD = '0x874069Fa1Eb16D44d622F2e0Ca25eeA172369bC1';
await distributeToEvent(eventId, cUSD, amountInWei);
```

### cEUR (Celo Euro)
```javascript
// Alfajores cEUR
const cEUR = '0x10c892A6EC43a53E45D0B916B4b7D383B1b78C0F';
await distributeToEvent(eventId, cEUR, amountInWei);
```

### cREAL (Celo Real)
```javascript
// Alfajores cREAL
const cREAL = '0xE4D517785D091D3c54818832dB6094bcc2744545';
await distributeToEvent(eventId, cREAL, amountInWei);
```

---

## 📋 Datos que Maneja el Frontend

### Input del Usuario:
```javascript
{
  imageBase64: "data:image/jpeg;base64,/9j/4AAQSkZJRg...",  // Imagen convertida
  walletAddress: "0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb"  // Dirección Celo
}
```

### Validaciones Frontend:
- ✅ Imagen no mayor a 5MB
- ✅ Formato: JPG, PNG, WEBP
- ✅ Dirección válida (0x + 40 caracteres hexadecimales)
- ✅ No campos vacíos

### Respuesta del Backend:
```javascript
{
  success: true,
  txHash: "0xabc123...",  // Hash de transacción en Celo
  message: "Usuario registrado exitosamente"
}
```

---

## 🔧 Ejemplo Completo Frontend

### HTML Simple
```html
<!DOCTYPE html>
<html>
<head>
  <title>ReFi Universe - Registro</title>
</head>
<body>
  <h1>Registro de Participante</h1>
  
  <form id="registerForm">
    <!-- Captura de foto -->
    <label>Tomar foto o subir imagen:</label>
    <input type="file" accept="image/*" id="imageInput" required />
    
    <img id="preview" style="max-width: 300px; display: none;" />
    
    <!-- Dirección de wallet -->
    <label>Dirección de wallet Celo:</label>
    <input 
      type="text" 
      id="walletAddress" 
      placeholder="0x..." 
      pattern="0x[a-fA-F0-9]{40}"
      required 
    />
    
    <button type="submit">Registrar</button>
  </form>

  <script>
    const form = document.getElementById('registerForm');
    const imageInput = document.getElementById('imageInput');
    const preview = document.getElementById('preview');
    
    // Previsualizar imagen
    imageInput.addEventListener('change', (e) => {
      const file = e.target.files[0];
      if (file) {
        preview.src = URL.createObjectURL(file);
        preview.style.display = 'block';
      }
    });
    
    // Enviar formulario
    form.addEventListener('submit', async (e) => {
      e.preventDefault();
      
      const file = imageInput.files[0];
      const walletAddress = document.getElementById('walletAddress').value;
      
      // Convertir a Base64
      const imageBase64 = await convertToBase64(file);
      
      // Enviar al backend
      const response = await fetch('/api/register', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ imageBase64, walletAddress })
      });
      
      const result = await response.json();
      alert(result.message);
    });
    
    function convertToBase64(file) {
      return new Promise((resolve, reject) => {
        const reader = new FileReader();
        reader.onload = () => resolve(reader.result);
        reader.onerror = reject;
        reader.readAsDataURL(file);
      });
    }
  </script>
</body>
</html>
```

### Backend API (Node.js + Express)
```javascript
const express = require('express');
const ethers = require('ethers');

const app = express();
app.use(express.json({ limit: '10mb' }));

// Configuración
const provider = new ethers.JsonRpcProvider(process.env.CELO_RPC_URL);
const wallet = new ethers.Wallet(process.env.ADMIN_PRIVATE_KEY, provider);
const contract = new ethers.Contract(
  process.env.CONTRACT_ADDRESS,
  ABI,
  wallet
);

// Endpoint de registro
app.post('/api/register', async (req, res) => {
  try {
    const { imageBase64, walletAddress } = req.body;
    
    // Validar
    if (!imageBase64 || !walletAddress) {
      return res.status(400).json({ error: 'Faltan datos' });
    }
    
    // Registrar en blockchain
    const tx = await contract.addHuman(walletAddress, imageBase64);
    await tx.wait();
    
    res.json({
      success: true,
      txHash: tx.hash,
      message: 'Usuario registrado exitosamente'
    });
    
  } catch (error) {
    res.status(500).json({ error: error.message });
  }
});

app.listen(3000);
```

---

## 🐛 Troubleshooting

### "Imagen muy grande"
→ Limitar tamaño a 5MB máximo
→ Comprimir imagen antes de convertir a Base64

### "Dirección inválida"
→ Verificar formato: 0x + 40 caracteres hexadecimales
→ Ejemplo válido: `0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb`

### "Error al subir imagen"
→ Verificar que el formato sea JPG, PNG o WEBP
→ Revisar que FileReader esté soportado

### "Backend no responde"
→ Verificar que el backend esté corriendo
→ Revisar CORS si frontend y backend están en dominios diferentes

### "Transaction reverted"
→ Verificar que el backend tenga fondos CELO para gas
→ Verificar que la dirección del contrato sea correcta

---

## 🔗 Recursos

**Celo Alfajores:**
- 🔍 [Explorer](https://alfajores.celoscan.io)
- 💰 [Faucet](https://faucet.celo.org/alfajores)
- 📚 [Docs](https://docs.celo.org/)
- 🎮 [Playground](https://celo.org/developers)

**Wallets:**
- 🦊 [MetaMask](https://metamask.io/)
- 📱 [Valora](https://valoraapp.com/)

**Development:**
- 📖 [Hardhat](https://hardhat.org/)
- ⚡ [Wagmi](https://wagmi.sh/)
- 🔷 [ethers.js](https://docs.ethers.org/)

---

## ✅ Checklist de Implementación

### Frontend (Usuario)
- [ ] Input para captura/upload de imagen
- [ ] Función para convertir imagen a Base64
- [ ] Input para dirección de wallet (validación 0x...)
- [ ] Previsualización de imagen
- [ ] Validación de tamaño (< 5MB)
- [ ] Formulario de envío
- [ ] Llamada POST al backend con {imageBase64, walletAddress}
- [ ] Mensaje de confirmación/error

### Backend (Admin)
- [ ] Instalar ethers.js: `npm install ethers`
- [ ] Configurar variables de entorno (.env)
- [ ] Obtener CELO de prueba para cuenta admin
- [ ] Endpoint POST /api/register
- [ ] Conectar con contrato Celo
- [ ] Función registerUser(walletAddress, imageBase64)
- [ ] Manejo de errores
- [ ] Logs de transacciones

### Juez Dashboard
- [ ] Obtener lista de participantes: getAllHumans()
- [ ] Mostrar galería (imagen Base64 + dirección)
- [ ] Botón validar/rechazar por participante
- [ ] Función updateHumanValidation(address, true/false)
- [ ] Crear eventos con participantes validados
- [ ] Distribuir fondos: distributeToEvent()
- [ ] Historial de transacciones

---

## 🎯 Resumen del Flujo

```
┌─────────────────────────────────────────────────────────┐
│                    FRONTEND (Usuario)                    │
├─────────────────────────────────────────────────────────┤
│                                                          │
│  1. Usuario toma foto o sube imagen                     │
│  2. JavaScript convierte imagen a Base64                │
│  3. Usuario ingresa dirección wallet: 0x...             │
│  4. Click "Enviar"                                       │
│                                                          │
│  POST /api/register                                      │
│  {                                                       │
│    imageBase64: "data:image/jpeg;base64,...",           │
│    walletAddress: "0x742d35Cc6634C0532..."             │
│  }                                                       │
│                                                          │
└─────────────────────────────────────────────────────────┘
                           │
                           ▼
┌─────────────────────────────────────────────────────────┐
│                 BACKEND (Node.js + ethers)               │
├─────────────────────────────────────────────────────────┤
│                                                          │
│  1. Recibe datos del frontend                           │
│  2. Conecta con contrato Celo (ethers.js)               │
│  3. Ejecuta: addHuman(walletAddress, imageBase64)       │
│  4. Espera confirmación de transacción                  │
│  5. Retorna txHash al frontend                          │
│                                                          │
└─────────────────────────────────────────────────────────┘
                           │
                           ▼
┌─────────────────────────────────────────────────────────┐
│              BLOCKCHAIN CELO (Alfajores)                 │
├─────────────────────────────────────────────────────────┤
│                                                          │
│  ✅ Participante registrado en el contrato              │
│  📍 Dirección: 0x742d35Cc6634C0532...                   │
│  📸 Imagen: stored as Base64 string                     │
│  ⏳ Estado: Pendiente de validación                     │
│                                                          │
└─────────────────────────────────────────────────────────┘
                           │
                           ▼
┌─────────────────────────────────────────────────────────┐
│                  DASHBOARD JUEZ                          │
├─────────────────────────────────────────────────────────┤
│                                                          │
│  1. Juez ve galería de participantes                    │
│  2. Revisa imagen (mostrar Base64 como <img>)           │
│  3. Valida: updateHumanValidation(address, true)        │
│  4. Crea evento con validados                           │
│  5. Distribuye: distributeToEvent(eventId, token, amt)  │
│                                                          │
└─────────────────────────────────────────────────────────┘
                           │
                           ▼
┌─────────────────────────────────────────────────────────┐
│                 USUARIO RECIBE FONDOS                    │
├─────────────────────────────────────────────────────────┤
│                                                          │
│  💰 CELO/cUSD enviados a la dirección proporcionada     │
│  📱 Usuario puede ver fondos en cualquier wallet        │
│  🔍 Transacción visible en Celoscan                     │
│                                                          │
└─────────────────────────────────────────────────────────┘
```

---

## 🎉 Ventajas de este Enfoque

✅ **Sin wallet connect** - Usuario solo proporciona dirección  
✅ **Sin IPFS** - Imágenes en Base64 directamente en blockchain  
✅ **Flujo simple** - 2 campos: imagen + dirección  
✅ **Backend controla todo** - Seguridad y gas manejados centralmente  
✅ **Costo bajo** - ~$0.001 por registro en Celo  
✅ **Distribución directa** - Fondos van directo a la dirección del usuario  

---

**🚀 ¡Listo para implementar!**

**Para desplegar en Celo Alfajores:**
1. Obtén CELO de prueba: https://faucet.celo.org/alfajores
2. Ejecuta: `npm run deploy:alfajores`
3. Copia el contract address al backend
4. Implementa el frontend simple
5. ¡Prueba el flujo completo!
