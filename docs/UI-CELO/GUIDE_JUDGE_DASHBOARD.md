# 👨‍⚖️ Dashboard del Juez - Celo

Guía de implementación para el dashboard de administración del juez en ReFi Universe (Celo).

---

## 📋 Funcionalidades del Dashboard

El dashboard del juez tiene **2 funciones principales**:

### 1. 📸 Galería de Participantes
- Ver todas las fotos subidas por los usuarios
- Scroll infinito de imágenes (Base64)
- Mostrar dirección de wallet debajo de cada imagen
- Botón para copiar dirección al clipboard

### 2. 💰 Distribución de Fondos
- Conectar wallet del juez (MetaMask/Valora)
- Ingresar monto total a distribuir
- Ingresar lista de direcciones validadas (copiar desde galería)
- Ejecutar distribución en blockchain

---

## 🎯 Arquitectura Simple

```
┌────────────────────────────────────────────────────────┐
│                  DASHBOARD DEL JUEZ                     │
├────────────────────────────────────────────────────────┤
│                                                         │
│  ┌──────────────────────────────────────────────────┐ │
│  │         SECCIÓN 1: GALERÍA DE IMÁGENES           │ │
│  │                                                   │ │
│  │  ┌─────────┐  ┌─────────┐  ┌─────────┐         │ │
│  │  │ [Img 1] │  │ [Img 2] │  │ [Img 3] │ ...     │ │
│  │  │ Base64  │  │ Base64  │  │ Base64  │         │ │
│  │  └─────────┘  └─────────┘  └─────────┘         │ │
│  │                                                   │ │
│  │  0x742d35...    0x8F3a21...    0x9B4c67...      │ │
│  │  [📋 Copiar]    [📋 Copiar]    [📋 Copiar]      │ │
│  │                                                   │ │
│  └──────────────────────────────────────────────────┘ │
│                                                         │
│  ┌──────────────────────────────────────────────────┐ │
│  │       SECCIÓN 2: DISTRIBUCIÓN DE FONDOS          │ │
│  │                                                   │ │
│  │  [🦊 Conectar Wallet]                            │ │
│  │                                                   │ │
│  │  Monto Total:    [______] CELO                   │ │
│  │                                                   │ │
│  │  Direcciones (una por línea):                    │ │
│  │  ┌────────────────────────────────────────────┐  │ │
│  │  │ 0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb │  │ │
│  │  │ 0x8F3a21Ba5B2Cd5A53046d9CB66E57f2F876d... │  │ │
│  │  │ 0x9B4c67Dd8E3Fg7H64157e0DC77F68g3G987e... │  │ │
│  │  └────────────────────────────────────────────┘  │ │
│  │                                                   │ │
│  │  [💸 Distribuir Fondos]                          │ │
│  │                                                   │ │
│  └──────────────────────────────────────────────────┘ │
│                                                         │
└────────────────────────────────────────────────────────┘
```

---

## 🚀 Implementación Frontend

### HTML Completo

```html
<!DOCTYPE html>
<html lang="es">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>Dashboard del Juez - ReFi Universe</title>
  <style>
    * {
      margin: 0;
      padding: 0;
      box-sizing: border-box;
    }

    body {
      font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
      background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
      padding: 20px;
      min-height: 100vh;
    }

    .container {
      max-width: 1200px;
      margin: 0 auto;
      background: white;
      border-radius: 16px;
      padding: 30px;
      box-shadow: 0 20px 60px rgba(0, 0, 0, 0.3);
    }

    h1 {
      color: #333;
      margin-bottom: 10px;
      font-size: 32px;
    }

    .subtitle {
      color: #666;
      margin-bottom: 30px;
      font-size: 16px;
    }

    /* SECCIÓN 1: GALERÍA */
    .gallery-section {
      margin-bottom: 40px;
      border-bottom: 2px solid #eee;
      padding-bottom: 40px;
    }

    .gallery-section h2 {
      color: #667eea;
      margin-bottom: 20px;
      font-size: 24px;
    }

    .gallery-grid {
      display: grid;
      grid-template-columns: repeat(auto-fill, minmax(250px, 1fr));
      gap: 20px;
      max-height: 600px;
      overflow-y: auto;
      padding: 10px;
      background: #f9f9f9;
      border-radius: 8px;
    }

    .participant-card {
      background: white;
      border-radius: 12px;
      padding: 15px;
      box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
      transition: transform 0.2s;
    }

    .participant-card:hover {
      transform: translateY(-5px);
      box-shadow: 0 8px 20px rgba(0, 0, 0, 0.15);
    }

    .participant-image {
      width: 100%;
      height: 200px;
      object-fit: cover;
      border-radius: 8px;
      margin-bottom: 10px;
      background: #eee;
    }

    .participant-address {
      font-family: 'Courier New', monospace;
      font-size: 12px;
      color: #555;
      word-break: break-all;
      margin-bottom: 10px;
      padding: 8px;
      background: #f5f5f5;
      border-radius: 4px;
    }

    .copy-btn {
      width: 100%;
      padding: 8px;
      background: #667eea;
      color: white;
      border: none;
      border-radius: 6px;
      cursor: pointer;
      font-weight: 600;
      transition: background 0.3s;
    }

    .copy-btn:hover {
      background: #5568d3;
    }

    .copy-btn:active {
      background: #4a5bc4;
    }

    .copy-btn.copied {
      background: #48bb78;
    }

    /* SECCIÓN 2: DISTRIBUCIÓN */
    .distribution-section h2 {
      color: #764ba2;
      margin-bottom: 20px;
      font-size: 24px;
    }

    .wallet-connect {
      margin-bottom: 20px;
    }

    .connect-btn {
      padding: 12px 24px;
      background: #764ba2;
      color: white;
      border: none;
      border-radius: 8px;
      cursor: pointer;
      font-size: 16px;
      font-weight: 600;
      transition: background 0.3s;
    }

    .connect-btn:hover {
      background: #6a3f91;
    }

    .wallet-info {
      display: none;
      padding: 12px;
      background: #e6fffa;
      border: 2px solid #48bb78;
      border-radius: 8px;
      margin-top: 10px;
    }

    .wallet-info.connected {
      display: block;
    }

    .form-group {
      margin-bottom: 20px;
    }

    .form-group label {
      display: block;
      color: #333;
      font-weight: 600;
      margin-bottom: 8px;
      font-size: 14px;
    }

    .form-group input {
      width: 100%;
      padding: 12px;
      border: 2px solid #ddd;
      border-radius: 8px;
      font-size: 16px;
      transition: border-color 0.3s;
    }

    .form-group input:focus {
      outline: none;
      border-color: #667eea;
    }

    .form-group textarea {
      width: 100%;
      padding: 12px;
      border: 2px solid #ddd;
      border-radius: 8px;
      font-size: 14px;
      font-family: 'Courier New', monospace;
      min-height: 150px;
      resize: vertical;
      transition: border-color 0.3s;
    }

    .form-group textarea:focus {
      outline: none;
      border-color: #667eea;
    }

    .helper-text {
      font-size: 12px;
      color: #666;
      margin-top: 5px;
    }

    .distribute-btn {
      width: 100%;
      padding: 16px;
      background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
      color: white;
      border: none;
      border-radius: 8px;
      cursor: pointer;
      font-size: 18px;
      font-weight: 700;
      transition: transform 0.2s;
    }

    .distribute-btn:hover {
      transform: scale(1.02);
    }

    .distribute-btn:disabled {
      background: #ccc;
      cursor: not-allowed;
      transform: none;
    }

    .status-message {
      margin-top: 15px;
      padding: 12px;
      border-radius: 8px;
      display: none;
    }

    .status-message.success {
      display: block;
      background: #d4edda;
      color: #155724;
      border: 1px solid #c3e6cb;
    }

    .status-message.error {
      display: block;
      background: #f8d7da;
      color: #721c24;
      border: 1px solid #f5c6cb;
    }

    .loading {
      text-align: center;
      padding: 20px;
      color: #667eea;
    }
  </style>
</head>
<body>
  <div class="container">
    <h1>👨‍⚖️ Dashboard del Juez</h1>
    <p class="subtitle">Revisa participantes y distribuye recompensas</p>

    <!-- SECCIÓN 1: GALERÍA DE PARTICIPANTES -->
    <section class="gallery-section">
      <h2>📸 Galería de Participantes</h2>
      <div id="galleryGrid" class="gallery-grid">
        <div class="loading">Cargando participantes...</div>
      </div>
    </section>

    <!-- SECCIÓN 2: DISTRIBUCIÓN DE FONDOS -->
    <section class="distribution-section">
      <h2>💰 Distribución de Fondos</h2>

      <!-- Conectar Wallet -->
      <div class="wallet-connect">
        <button id="connectBtn" class="connect-btn">🦊 Conectar Wallet</button>
        <div id="walletInfo" class="wallet-info">
          <strong>✅ Wallet Conectada:</strong> <span id="walletAddress"></span>
        </div>
      </div>

      <!-- Formulario de Distribución -->
      <form id="distributionForm">
        <div class="form-group">
          <label for="totalAmount">Monto Total (CELO)</label>
          <input 
            type="number" 
            id="totalAmount" 
            placeholder="10.0" 
            step="0.01"
            min="0"
            required
          />
          <p class="helper-text">Cantidad total a distribuir equitativamente</p>
        </div>

        <div class="form-group">
          <label for="addresses">Direcciones de Wallets (una por línea)</label>
          <textarea 
            id="addresses" 
            placeholder="0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb&#10;0x8F3a21Ba5B2Cd5A53046d9CB66E57f2F876d..."
            required
          ></textarea>
          <p class="helper-text">
            Copia las direcciones desde la galería arriba. 
            Presiona los botones "📋 Copiar" y pégalas aquí.
          </p>
        </div>

        <button type="submit" id="distributeBtn" class="distribute-btn" disabled>
          💸 Distribuir Fondos
        </button>
      </form>

      <div id="statusMessage" class="status-message"></div>
    </section>
  </div>

  <script src="https://cdn.jsdelivr.net/npm/ethers@6.7.0/dist/ethers.umd.min.js"></script>
  <script>
    // ===== CONFIGURACIÓN =====
    const CONFIG = {
      BACKEND_URL: 'http://localhost:3000/api', // Cambiar según tu backend
      CONTRACT_ADDRESS: '0x5FbDB2315678afecb367f032d93F642f64180aa3', // Cambiar al real
      CELO_RPC: 'https://alfajores-forno.celo-testnet.org',
      CHAIN_ID: 44787
    };

    // ABI del contrato (solo las funciones que usamos)
    const CONTRACT_ABI = [
      "function getAllHumans(uint256 start, uint256 limit) view returns (tuple(address walletAddress, string ipfsHash, bool validated, bool exists)[])",
      "function distributeToAddresses(address[] recipients, address token, uint256 totalAmount) payable"
    ];

    // ===== ESTADO GLOBAL =====
    let provider = null;
    let signer = null;
    let contract = null;
    let connectedAddress = null;

    // ===== ELEMENTOS DOM =====
    const galleryGrid = document.getElementById('galleryGrid');
    const connectBtn = document.getElementById('connectBtn');
    const walletInfo = document.getElementById('walletInfo');
    const walletAddressSpan = document.getElementById('walletAddress');
    const distributionForm = document.getElementById('distributionForm');
    const distributeBtn = document.getElementById('distributeBtn');
    const statusMessage = document.getElementById('statusMessage');

    // ===== CARGAR GALERÍA =====
    async function loadGallery() {
      try {
        // Conectar al contrato en modo read-only
        const readProvider = new ethers.JsonRpcProvider(CONFIG.CELO_RPC);
        const readContract = new ethers.Contract(
          CONFIG.CONTRACT_ADDRESS,
          CONTRACT_ABI,
          readProvider
        );

        // Obtener todos los participantes
        const humans = await readContract.getAllHumans(0, 100);
        
        galleryGrid.innerHTML = '';

        if (humans.length === 0) {
          galleryGrid.innerHTML = '<p style="text-align:center; color:#666;">No hay participantes aún</p>';
          return;
        }

        humans.forEach((human, index) => {
          const card = createParticipantCard(human, index);
          galleryGrid.appendChild(card);
        });

      } catch (error) {
        console.error('Error cargando galería:', error);
        galleryGrid.innerHTML = '<p style="text-align:center; color:red;">Error al cargar participantes</p>';
      }
    }

    // ===== CREAR TARJETA DE PARTICIPANTE =====
    function createParticipantCard(human, index) {
      const card = document.createElement('div');
      card.className = 'participant-card';

      const img = document.createElement('img');
      img.className = 'participant-image';
      img.src = human.ipfsHash; // El ipfsHash es el Base64
      img.alt = `Participante ${index + 1}`;
      img.onerror = () => {
        img.src = 'data:image/svg+xml,<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 100 100"><text y="50" font-size="40">👤</text></svg>';
      };

      const addressDiv = document.createElement('div');
      addressDiv.className = 'participant-address';
      addressDiv.textContent = human.walletAddress;

      const copyBtn = document.createElement('button');
      copyBtn.className = 'copy-btn';
      copyBtn.textContent = '📋 Copiar Dirección';
      copyBtn.onclick = () => copyAddress(human.walletAddress, copyBtn);

      card.appendChild(img);
      card.appendChild(addressDiv);
      card.appendChild(copyBtn);

      return card;
    }

    // ===== COPIAR DIRECCIÓN =====
    function copyAddress(address, button) {
      navigator.clipboard.writeText(address).then(() => {
        const originalText = button.textContent;
        button.textContent = '✅ Copiado!';
        button.classList.add('copied');
        
        setTimeout(() => {
          button.textContent = originalText;
          button.classList.remove('copied');
        }, 2000);
      });
    }

    // ===== CONECTAR WALLET =====
    async function connectWallet() {
      try {
        if (!window.ethereum) {
          alert('Por favor instala MetaMask');
          return;
        }

        // Solicitar cuentas
        const accounts = await window.ethereum.request({ 
          method: 'eth_requestAccounts' 
        });

        // Verificar red
        const chainId = await window.ethereum.request({ 
          method: 'eth_chainId' 
        });

        if (parseInt(chainId, 16) !== CONFIG.CHAIN_ID) {
          // Intentar cambiar a Celo Alfajores
          try {
            await window.ethereum.request({
              method: 'wallet_switchEthereumChain',
              params: [{ chainId: `0x${CONFIG.CHAIN_ID.toString(16)}` }],
            });
          } catch (switchError) {
            // Red no existe, agregarla
            if (switchError.code === 4902) {
              await window.ethereum.request({
                method: 'wallet_addEthereumChain',
                params: [{
                  chainId: `0x${CONFIG.CHAIN_ID.toString(16)}`,
                  chainName: 'Celo Alfajores Testnet',
                  nativeCurrency: {
                    name: 'CELO',
                    symbol: 'CELO',
                    decimals: 18
                  },
                  rpcUrls: [CONFIG.CELO_RPC],
                  blockExplorerUrls: ['https://alfajores.celoscan.io']
                }]
              });
            } else {
              throw switchError;
            }
          }
        }

        // Crear provider y signer
        provider = new ethers.BrowserProvider(window.ethereum);
        signer = await provider.getSigner();
        connectedAddress = accounts[0];

        // Crear instancia del contrato
        contract = new ethers.Contract(
          CONFIG.CONTRACT_ADDRESS,
          CONTRACT_ABI,
          signer
        );

        // Actualizar UI
        connectBtn.style.display = 'none';
        walletInfo.classList.add('connected');
        walletAddressSpan.textContent = `${connectedAddress.substring(0, 6)}...${connectedAddress.substring(38)}`;
        distributeBtn.disabled = false;

      } catch (error) {
        console.error('Error conectando wallet:', error);
        alert('Error al conectar wallet: ' + error.message);
      }
    }

    // ===== DISTRIBUIR FONDOS =====
    async function distributeFunds(e) {
      e.preventDefault();

      if (!contract) {
        alert('Por favor conecta tu wallet primero');
        return;
      }

      try {
        distributeBtn.disabled = true;
        distributeBtn.textContent = '⏳ Procesando...';
        statusMessage.className = 'status-message';
        statusMessage.style.display = 'none';

        // Obtener datos del formulario
        const totalAmount = document.getElementById('totalAmount').value;
        const addressesText = document.getElementById('addresses').value;

        // Parsear direcciones
        const addresses = addressesText
          .split('\n')
          .map(addr => addr.trim())
          .filter(addr => addr.length > 0);

        // Validar
        if (addresses.length === 0) {
          throw new Error('Debes ingresar al menos una dirección');
        }

        for (const addr of addresses) {
          if (!ethers.isAddress(addr)) {
            throw new Error(`Dirección inválida: ${addr}`);
          }
        }

        // Convertir monto a Wei
        const amountWei = ethers.parseEther(totalAmount);

        // Ejecutar distribución (CELO nativo = address(0))
        const tx = await contract.distributeToAddresses(
          addresses,
          ethers.ZeroAddress,
          amountWei,
          { value: amountWei }
        );

        statusMessage.textContent = '⏳ Esperando confirmación...';
        statusMessage.className = 'status-message';
        statusMessage.style.display = 'block';

        // Esperar confirmación
        const receipt = await tx.wait();

        // Éxito
        statusMessage.textContent = `✅ ¡Distribución exitosa! TX: ${receipt.hash.substring(0, 10)}...`;
        statusMessage.className = 'status-message success';
        
        // Limpiar formulario
        document.getElementById('totalAmount').value = '';
        document.getElementById('addresses').value = '';

      } catch (error) {
        console.error('Error en distribución:', error);
        statusMessage.textContent = `❌ Error: ${error.message}`;
        statusMessage.className = 'status-message error';
      } finally {
        distributeBtn.disabled = false;
        distributeBtn.textContent = '💸 Distribuir Fondos';
      }
    }

    // ===== EVENT LISTENERS =====
    connectBtn.addEventListener('click', connectWallet);
    distributionForm.addEventListener('submit', distributeFunds);

    // ===== INICIALIZAR =====
    window.addEventListener('load', () => {
      loadGallery();
    });

    // Detectar cambio de cuenta
    if (window.ethereum) {
      window.ethereum.on('accountsChanged', (accounts) => {
        if (accounts.length === 0) {
          location.reload();
        } else {
          connectedAddress = accounts[0];
          walletAddressSpan.textContent = `${connectedAddress.substring(0, 6)}...${connectedAddress.substring(38)}`;
        }
      });
    }
  </script>
</body>
</html>
```

---

## 📦 Características Implementadas

### ✅ Galería de Participantes
- **Grid responsive** con imágenes en Base64
- **Scroll infinito** (max-height con overflow)
- **Dirección debajo** de cada imagen
- **Botón copiar** que cambia a "✅ Copiado!" temporalmente
- **Tarjetas con hover** animado

### ✅ Conexión de Wallet
- **Botón único** para conectar MetaMask/Valora
- **Auto-detección** de red Celo Alfajores
- **Auto-add network** si no existe
- **Mostrar dirección** conectada (truncada)

### ✅ Distribución de Fondos
- **Input de monto** total en CELO
- **Textarea** para pegar múltiples direcciones
- **Validación** de direcciones Ethereum
- **Distribución equitativa** automática
- **Feedback visual** (loading, success, error)
- **Transaction hash** en mensaje de éxito

---

## 🔧 Configuración

### Cambiar Contract Address
```javascript
const CONFIG = {
  CONTRACT_ADDRESS: '0xTU_CONTRATO_AQUI', // ← Cambiar después del deployment
  CELO_RPC: 'https://alfajores-forno.celo-testnet.org',
  CHAIN_ID: 44787
};
```

### Integrar con Backend (Opcional)
Si quieres cargar participantes desde tu backend en lugar del contrato:

```javascript
async function loadGallery() {
  try {
    const response = await fetch(`${CONFIG.BACKEND_URL}/participants`);
    const participants = await response.json();
    
    galleryGrid.innerHTML = '';
    participants.forEach((p, index) => {
      const card = createParticipantCard(p, index);
      galleryGrid.appendChild(card);
    });
  } catch (error) {
    console.error('Error:', error);
  }
}
```

---

## 🎨 Personalización

### Cambiar Colores
```css
/* Gradiente principal */
background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);

/* Botón primario */
.copy-btn {
  background: #667eea;
}

/* Botón secundario */
.connect-btn {
  background: #764ba2;
}
```

### Ajustar Grid
```css
/* Más columnas en pantallas grandes */
.gallery-grid {
  grid-template-columns: repeat(auto-fill, minmax(200px, 1fr));
}

/* Menos columnas */
.gallery-grid {
  grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
}
```

---

## 🚀 Deployment

### Opción 1: Archivo Estático
```bash
# Simplemente abre el HTML en el navegador
open dashboard.html
```

### Opción 2: Servidor Local
```bash
# Con Python
python -m http.server 8000

# Con Node.js (http-server)
npx http-server -p 8000
```

### Opción 3: Deploy en Vercel/Netlify
```bash
# Crear vercel.json
{
  "version": 2,
  "builds": [
    { "src": "dashboard.html", "use": "@vercel/static" }
  ]
}

# Deploy
vercel deploy
```

---

## ✅ Checklist de Uso

### Para el Juez:
1. [ ] Abrir dashboard en navegador
2. [ ] Ver galería de participantes
3. [ ] Revisar imágenes (Base64 renderizadas)
4. [ ] Copiar direcciones válidas con botón 📋
5. [ ] Conectar wallet (MetaMask/Valora)
6. [ ] Ingresar monto total
7. [ ] Pegar direcciones copiadas
8. [ ] Click "Distribuir Fondos"
9. [ ] Confirmar transacción en wallet
10. [ ] Ver confirmación con TX hash

---

## 🔍 Funciones del Contrato

### getAllHumans(start, limit)
Lee participantes desde el contrato:
```javascript
const humans = await contract.getAllHumans(0, 100);
// Returns: Array<{walletAddress, ipfsHash, validated, exists}>
```

### distributeToAddresses(recipients, token, totalAmount)
Distribuye fondos a múltiples direcciones:
```javascript
await contract.distributeToAddresses(
  ['0xAddr1...', '0xAddr2...'],  // Direcciones
  ethers.ZeroAddress,             // CELO nativo
  ethers.parseEther('10'),        // 10 CELO total
  { value: ethers.parseEther('10') } // Enviar CELO
);
```

---

## 🎉 ¡Listo!

El dashboard del juez está completo con:
- ✅ Galería visual de participantes
- ✅ Botones de copiar direcciones
- ✅ Conexión simple de wallet
- ✅ Distribución de fondos CELO
- ✅ Todo en un solo archivo HTML

**Solo necesitas desplegar el contrato en Celo Alfajores y actualizar el `CONTRACT_ADDRESS`!**
