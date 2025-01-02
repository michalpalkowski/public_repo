# StarkNet Message Hash Testing

This minimal-example demonstrates and compares StarkNet message hashing implementations in two versions:
- JavaScript (using ArgentX wallet)
- Rust (custom implementation)

## Hashing

1. **Start HTTP server:**
   ```bash
   # In the web directory
   cd web
   python -m http.server 8000
   ```

2. **Test ArgentX Implementation:**
   - Open http://localhost:8000 in your browser
   - Ensure ArgentX wallet is installed and unlocked
   - Default test values:
     ```
     Account Address: 0x01b175fe86400121641d32d47490f76cd1ff973a6f090631496c0a08a530ed18
     Message: Hello Argent!
     DApp Name: Example DApp
     Version: 0.0.1
     Chain ID: SN_SEPOLIA
     ```
   - Click "Hash Message" button
   - Note the generated hash value

3. **Test Rust Implementation:**
   ```bash
   # In a new terminal
   cargo run
   ```
   - The program will use the same default values
   - Note the generated hash value

4. **Compare Results:**
   - Both implementations should produce identical hash values
