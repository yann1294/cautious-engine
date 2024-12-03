
# ToString Extension for Zed IDE

The **ToString Extension** for Zed IDE automatically generates `toString()` methods for TypeScript classes. This extension processes a TypeScript file and appends `toString()` methods to all classes that lack them.

---

## Features

- Automatically identifies class definitions in TypeScript files.
- Adds properly formatted `toString()` methods.
- Processes the entire file and provides the updated content as output.

---

## Requirements

- **Zed IDE** installed.
- **Rust** installed on your system. You can install Rust via [rustup](https://rustup.rs/).

---

## Installation

### Clone the Repository

```bash
git clone <repository-url>
cd zed-tostring-extension
```

### Build the Extension

#### macOS

1. Ensure the Rust toolchain is installed:

   ```bash
   brew install rust
   rustup install stable
   ```

2. Build the extension as a dynamic library:

   ```bash
   cargo build --release
   ```

3. The compiled file will be located at:
   ```
   target/release/libzed_tostring_extension.dylib
   ```

#### Linux

1. Install Rust using the following command:

   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. Build the extension as a shared object:

   ```bash
   cargo build --release
   ```

3. The compiled file will be located at:
   ```
   target/release/libzed_tostring_extension.so
   ```

---

## Installing the Extension in Zed IDE

1. Create the extension directory:

   ```bash
   mkdir -p ~/.zed/extensions/zed-tostring-extension
   ```

2. Copy the compiled file and `extension.toml` into the directory:

   - For macOS:

     ```bash
     cp target/release/libzed_tostring_extension.dylib ~/.zed/extensions/zed-tostring-extension/
     cp extension.toml ~/.zed/extensions/zed-tostring-extension/
     ```

   - For Linux:

     ```bash
     cp target/release/libzed_tostring_extension.so ~/.zed/extensions/zed-tostring-extension/
     cp extension.toml ~/.zed/extensions/zed-tostring-extension/
     ```

3. Restart Zed IDE to load the extension.

---

## Using the Extension

1. Open a TypeScript file containing class definitions in Zed IDE.
2. Open the command palette (`Ctrl + P` or `Cmd + P`).
3. Run the `/generate_tostring` command.
4. The output will include the updated content with the added `toString()` methods.

---

## Example

### Input File:

```typescript
class Person {
    constructor(public name: string, private age: number) {}
}
```

### Output File (via command output):

```typescript
class Person {
    constructor(public name: string, private age: number) {}

    toString() {
        return `Person { ` + Object.entries(this)
            .map(([k, v]) => `${k}: ${JSON.stringify(v)}`)
            .join(", ") + " }";
    }
}
```

---

## Troubleshooting

- **Extension Not Detected**:
  - Ensure the compiled `.dylib` (macOS) or `.so` (Linux) file and `extension.toml` are in the correct directory (`~/.zed/extensions/zed-tostring-extension/`).
  - Restart Zed IDE after copying the files.

- **Rust Issues**:
  - Ensure Rust is properly installed and updated:
    ```bash
    rustup update
    ```

- **File Not Found Error**:
  - Ensure the file path logic in the extension matches your setup.

---

## License

This extension is licensed under the MIT License.

---

Let me know if you have any additional questions or need further assistance!
