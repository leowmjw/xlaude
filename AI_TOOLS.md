# xlaude - Multi-AI Coding Tool Management

*English | [Bahasa Malaysia](#xlaude---pengurusan-alat-pengekodan-berbilang-ai)*

xlaude now supports multiple AI coding tools, with the following priority order:

1. **OpenCode** - A powerful terminal AI coding tool
2. **Qwen Code** - Qwen's code assistant CLI
3. **Claude** - AI assistant developed by Anthropic

## AI Tool Priority and Fallback Mechanism

xlaude will try to launch AI coding tools according to the above priority. When you run the `xlaude open` command:

1. First, it tries to use OpenCode
2. If OpenCode is not available, it will try Qwen Code
3. If Qwen Code is also not available, it falls back to using Claude

## Environment Variable Configuration

You can customize the command for each AI tool through environment variables:

- `XLAUDE_OPENCODE_CMD`: Override the OpenCode command (default: "opencode")
- `XLAUDE_QWEN_CMD`: Override the Qwen Code command (default: "qwen")
- `XLAUDE_CLAUDE_CMD`: Override the Claude command (default: "claude")

For example:
```bash
# Use a custom OpenCode path
export XLAUDE_OPENCODE_CMD="/usr/local/bin/my-opencode"

# Completely disable a tool (set to empty string)
export XLAUDE_QWEN_CMD=""
```

## Tool Installation Guide

### OpenCode

```bash
# Using installation script
curl -fsSL https://raw.githubusercontent.com/opencode-ai/opencode/refs/heads/main/install | bash

# Using Homebrew (macOS and Linux)
brew install opencode-ai/tap/opencode
```

More information: [OpenCode Documentation](https://opencode.ai/docs/cli/)

### Qwen Code

```bash
# Install with npm
npm install -g @qwen-code/qwen-code@latest

# Check version
qwen --version
```

More information: [Qwen Code Repository](https://github.com/QwenLM/qwen-code)

### Claude

Please refer to Anthropic's official installation guide.

## Typical Workflow

The workflow remains the same, but now xlaude will automatically select the available AI coding tool:

```bash
# Create a new workspace
xlaude create feature-auth

# Open and automatically launch the highest priority available AI coding tool
xlaude open feature-auth
```

---

# xlaude - Pengurusan Alat Pengekodan Berbilang AI

*[English](#xlaude---multi-ai-coding-tool-management) | Bahasa Malaysia*

xlaude kini menyokong pelbagai alat pengekodan AI, dengan urutan keutamaan berikut:

1. **OpenCode** - Alat pengekodan AI terminal yang berkuasa
2. **Qwen Code** - CLI pembantu kod Qwen
3. **Claude** - Pembantu AI yang dibangunkan oleh Anthropic

## Keutamaan Alat AI dan Mekanisme Sandaran

xlaude akan cuba melancarkan alat pengekodan AI mengikut keutamaan di atas. Apabila anda menjalankan perintah `xlaude open`:

1. Pertama, ia mencuba menggunakan OpenCode
2. Jika OpenCode tidak tersedia, ia akan mencuba Qwen Code
3. Jika Qwen Code juga tidak tersedia, ia akan menggunakan Claude sebagai pilihan terakhir

## Konfigurasi Pembolehubah Persekitaran

Anda boleh menyesuaikan perintah untuk setiap alat AI melalui pembolehubah persekitaran:

- `XLAUDE_OPENCODE_CMD`: Ganti perintah OpenCode (lalai: "opencode")
- `XLAUDE_QWEN_CMD`: Ganti perintah Qwen Code (lalai: "qwen")
- `XLAUDE_CLAUDE_CMD`: Ganti perintah Claude (lalai: "claude")

Contoh:
```bash
# Gunakan laluan OpenCode yang disesuaikan
export XLAUDE_OPENCODE_CMD="/usr/local/bin/my-opencode"

# Lumpuhkan alat sepenuhnya (tetapkan kepada rentetan kosong)
export XLAUDE_QWEN_CMD=""
```

## Panduan Pemasangan Alat

### OpenCode

```bash
# Menggunakan skrip pemasangan
curl -fsSL https://raw.githubusercontent.com/opencode-ai/opencode/refs/heads/main/install | bash

# Menggunakan Homebrew (macOS dan Linux)
brew install opencode-ai/tap/opencode
```

Maklumat lanjut: [Dokumentasi OpenCode](https://opencode.ai/docs/cli/)

### Qwen Code

```bash
# Pasang dengan npm
npm install -g @qwen-code/qwen-code@latest

# Semak versi
qwen --version
```

Maklumat lanjut: [Repositori Qwen Code](https://github.com/QwenLM/qwen-code)

### Claude

Sila rujuk panduan pemasangan rasmi Anthropic.

## Aliran Kerja Tipikal

Aliran kerja kekal sama, tetapi kini xlaude akan memilih alat pengekodan AI yang tersedia secara automatik:

```bash
# Cipta ruang kerja baru
xlaude create feature-auth

# Buka dan lancarkan secara automatik alat pengekodan AI tersedia dengan keutamaan tertinggi
xlaude open feature-auth
```
