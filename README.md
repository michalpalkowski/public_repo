
## Testing

1. **Test ArgentX Implementation**:
   - Open `http://127.0.0.1:5500/src/index.html` in your browser
   - Make sure ArgentX wallet is installed and unlocked
   - The default values should be:
     - Account Address: `0x01b175fe86400121641d32d47490f76cd1ff973a6f090631496c0a08a530ed18`
     - Message: `Hello Argent!`
     - DApp Name: `Example DApp`
     - Version: `0.0.1`
     - Chain ID: `SN_SEPOLIA`
   - Click "Hash Message" button
   - The hashed message will be displayed on the page
   - Note the hash value

2. **Test Rust Implementation**:
   - Run the Rust program:
     ```bash
     cargo run
     ```
   - The program uses the same default values
   - Note the final hash value

3. **Compare Results**:
   - Both implementations should produce identical hash values
   - The ArgentX hash should match the Rust implementation's final hash
