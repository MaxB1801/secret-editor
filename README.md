# Kubernetes Secret Editor CLI

A **Rust-based** CLI tool with **prebuilt binaries** for **Windows** and **Linux** that simplifies editing Kubernetes secrets directly from your terminal. No more manual base64 encoding/decoding hassle!

## Features

- **Direct editing** - Opens secrets in your default text editor
- **Automatic base64 handling** - Encoding and decoding handled seamlessly
- **Cross-platform** - Works on both Windows and Linux
- **Simple usage** - Just provide namespace and secret name
- **No source code required** - Ready-to-use prebuilt binaries

##  Requirements

- `kubectl` installed and configured with cluster access
- Text editor available:
  - **Windows**: notepad (built-in)
  - **Linux**: Any editor (uses `$EDITOR` environment variable, defaults to vim)

## 🛠️ Usage

### Linux
```bash
./cs <namespace> <secret>
```

### Windows (PowerShell or Command Prompt)
```cmd
cs.exe <namespace> <secret>
```

### Example
```bash
cs my-namespace my-secret
```

This will open the secret in your default editor where you can:
1. Edit keys and values directly (no base64 needed)
2. Save and exit
3. Changes are automatically applied to your cluster

## 🔧 Installation & Setup

### Running from Anywhere

To use the `cs` command from any directory, add the binary to your system PATH:

#### Linux (bash/zsh)
1. Add to your `~/.bashrc` or `~/.zshrc`:
   ```bash
   export PATH="$PATH:/path/to/binary"
   ```

2. Reload your shell:
   ```bash
   source ~/.bashrc
   # or
   source ~/.zshrc
   ```

#### Windows
1. Open **System Properties** → **Environment Variables**
2. Add the folder containing `cs.exe` to your system PATH
3. Restart your command prompt or PowerShell

##  What's Included

- **Linux binary**: `cs` (executable)
- **Windows binary**: `cs.exe`
- Both binaries are compiled with `cargo build` (no source code included)

##  How It Works

1. Retrieves the specified Kubernetes secret
2. Automatically decodes base64-encoded values
3. Opens the decoded content in your text editor as `JSON`
4. After you save and exit, re-encodes the content
5. Applies the updated secret back to your cluster

---

**Note**: Make sure your `kubectl` context is set to the correct cluster before using this tool.
