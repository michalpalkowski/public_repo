# This repo provides short examples to test ArgentX features

 **How to start HTTP server:**
   ```bash
   cd web
   python -m http.server 8000
   ```

 **Test ArgentX issue with loading transactionDetails using HTTP server:**
   - Open http://localhost:8000 in your browser
   - Ensure ArgentX wallet is installed and unlocked
   - Click "Verify Proof" button to see the error

 **Test session keys Implementation using HTML example and Live server extension:**
   ```npm init -y```

   ```npm install starknet```

   ```npm install @argent/x-sessions```

   ```cd session_keys```

   ```open html filewith live server```

 **Test session keys Implementation using vite:**
   ```cd my_session_keys```
   ```npm install```
   ```npm run dev```

   Open http://localhost:5173 in your browser