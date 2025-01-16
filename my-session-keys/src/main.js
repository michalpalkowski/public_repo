import './style.css'
import * as sessions from "@argent/x-sessions"
import { ec, constants, Provider } from "starknet"
import { connect } from "get-starknet"

// Sprawdź dostępne eksporty
console.log('Available exports from @argent/x-sessions:', Object.keys(sessions))

const {
  createSession,
  createSessionRequest,
  buildSessionAccount,
  bytesToHexString
} = sessions

const ETHTokenAddress = "0x049d36570d4e46f48e99674bd3fcc84644ddd6b96f7c741b1562b82f9e004dc7"

window.starknetEc = ec;
window.starknetProvider = Provider;
window.argentXSessions = {
    createSession,
    createSessionRequest,
    buildSessionAccount,
    bytesToHexString,
    // SignSessionError,
    // CreateSessionParams
};

console.log('Starknet EC:', window.starknetEc);
console.log('Starknet Provider:', window.starknetProvider);
console.log('Argent Sessions:', window.argentXSessions);

document.querySelector('#app').innerHTML = `
  <div class="container">
    <h1>Session Keys Demo</h1>
    
    <div>
        <button id="createSessionBtn">Create Session Key</button>
        <button id="executeSessionBtn" disabled>Execute With Session</button>
    </div>
    
    <div id="result"></div>
    <div id="debug"></div>
  </div>
`

function debug(message, data) {
    const debugEl = document.getElementById('debug');
    debugEl.innerHTML += `\n[${new Date().toISOString()}] ${message}\n${JSON.stringify(data, null, 2)}\n`;
    console.log(message, data);
}

async function createSessionKey() {
  try {
      const starknet = await connect()
      if (!starknet) {
          throw new Error('Please install ArgentX wallet')
      }
      
      await starknet.enable()
      const wallet = starknet
      
      console.log('Creating session key...')
      debug('Creating session key...', {})

      if (!window.argentXSessions || !window.starknetEc) {
          throw new Error('Modules not loaded yet. Please try again in a moment.');
      }

      const { createSession, createSessionRequest, buildSessionAccount} = window.argentXSessions;
      const ec = window.starknetEc;

      const privateKey = ec.starkCurve.utils.randomPrivateKey();
      const privateKeyHex = window.argentXSessions.bytesToHexString(privateKey);
      const sessionKey = {
          privateKey: privateKeyHex,
          publicKey: ec.starkCurve.getStarkKey(privateKeyHex)
      };
      debug('Session key created', { publicKey: sessionKey.publicKey });

      const sessionParams = {
          allowedMethods: [
              {
                  "Contract Address": "0x2ba5c816a66f69755a17dd1d0daffdd760d5b7375ba6226c4a9bcb784b2d33c",
                  selector: "approve"
              },
              {
                  "Contract Address": "0x6869b5c969538f5112ab2e4568adfd3d8e1922d2af6f8fbabadadeb101d4ffd",
                  selector: "deposit"
              },
          ],
          expiry: Math.floor((Date.now() + 1000 * 60 * 60 * 24) / 1000),
          sessionKey: sessionKey,
          metaData: {
              projectID: "test-dapp",
              txFees: [{
                  tokenAddress: ETHTokenAddress,
                  maxAmount: "0x10000000000000"
              }]
          }
      };
      debug('Session params configured', sessionParams);

      const chainId = await wallet.provider.getChainId();
      const sessionRequest = createSessionRequest({
          sessionParams,
          chainId
      });
      debug('Session request created', sessionRequest);

      const authorisationSignature = await wallet.request({
          type: "wallet_signTypedData",
          params: sessionRequest.sessionTypedData
      });
      debug('Got authorization signature', { authorisationSignature });

      const session = await createSession({
          sessionRequest,
          address: wallet.account.address,
          chainId,
          authorisationSignature
      });
      debug('Session created', session);

      const sessionAccount = await buildSessionAccount({
          useCacheAuthorisation: false,
          session,
          sessionKey,
          provider: new window.starknetProvider({
              rpc: {
                  nodeUrl: "<https://starknet-sepolia.public.blastapi.io/rpc/v0_7>",
                  chainId: "SN_SEPOLIA"
              }
          }),
          argentSessionServiceBaseUrl: "https://cloud.argent-api.com/v1/starknet/sepolia/rpc/v0.7"
      });
      debug('Session account built', { address: sessionAccount.address });

      window.sessionData = { sessionAccount, session, sessionKey };
      document.getElementById('executeSessionBtn').disabled = false;
      
      document.getElementById('result').innerHTML = `
          <div class="success-message">
              Session created successfully!<br>
              Session public key: ${sessionKey.publicKey}
          </div>
      `;
  } catch (error) {
      console.error('Session creation error:', error);
      const resultElement = document.getElementById('result');
      if (resultElement) {
          resultElement.innerHTML = `
              <div class="error-message">
                  Error: ${error instanceof Error ? error.message : String(error)}
              </div>
          `;
      }
  }
}

const executeWithSession = async () => {
  console.log('Executing with session...'); 
  if (!window.sessionData) {
      alert('Please create a session first');
      return;
  }
  try {
      const tx1 = await window.sessionData.sessionAccount.execute({
          contractAddress: "0x2ba5c816a66f69755a17dd1d0daffdd760d5b7375ba6226c4a9bcb784b2d33c",
          entrypoint: "approve",
          calldata: ["0x2", "0x10000000000", "0x0"]
      });
      console.log('Approve transaction executed:', tx1);

      const tx2 = await window.sessionData.sessionAccount.execute({
          contractAddress: "0x6869b5c969538f5112ab2e4568adfd3d8e1922d2af6f8fbabadadeb101d4ffd",
          entrypoint: "deposit",
          calldata: ["0x1", "0x2"]
      });
      console.log('Deposit transaction executed:', tx2);

      document.getElementById('result').innerHTML = `
          <div class="success-message">
              Transactions executed successfully!<br>
              Deposit transaction hash: ${tx1.transaction_hash}<br>
              Approve transaction hash: ${tx2.transaction_hash}
          </div>
      `;
  } catch (error) {
      console.error('Execution error:', error);
      document.getElementById('result').innerHTML = `
          <div class="error-message">
              Error: ${error.message}
          </div>
      `;
  }
};

document.getElementById('createSessionBtn').onclick = createSessionKey;
document.getElementById('executeSessionBtn').onclick = executeWithSession;

debug('Page loaded', { timestamp: Date.now() });
