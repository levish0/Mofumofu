# Mofumofu Markdown Service

A fast markdown rendering service built with Bun and Elysia.

## Features

- ✅ **Fast**: Built with Bun for maximum performance
- ✅ **Rich Markdown**: Supports GFM, math, emoji, code highlighting
- ✅ **Secure**: HTML sanitization with rehype-sanitize
- ✅ **TOC Generation**: Automatic table of contents extraction
- ✅ **Internal Service**: Designed for internal use by Rust backend

## Development

### Prerequisites

- [Bun](https://bun.sh) installed

### Install Dependencies

```bash
bun install
```

### Development Mode

```bash
bun dev
```

The service will run at `http://localhost:6700` with hot reload.

### Production

```bash
bun start
```

## API Endpoints

### `GET /`

Returns service information.

### `GET /health`

Health check endpoint.

**Response:**
```json
{
  "status": "ok",
  "service": "mofumofu-markdown-service",
  "timestamp": "2025-08-13T10:00:00.000Z"
}
```

### `POST /render`

Renders markdown to HTML.

**Request:**
```json
{
  "markdown": "# Hello World\n\nThis is **bold** text."
}
```

**Response:**
```json
{
  "success": true,
  "data": {
    "htmlContent": "<h1 id=\"h-hello-world\">Hello World</h1>\n<p>This is <strong>bold</strong> text.</p>",
    "tocItems": [
      {
        "level": 1,
        "text": "Hello World",
        "id": "h-hello-world"
      }
    ]
  }
}
```

## Integration with Rust Backend

The Rust backend calls this service internally:

```rust
// Example Rust code
let response = reqwest::Client::new()
    .post("http://localhost:6700/render")
    .json(&json!({ "markdown": content }))
    .send()
    .await?;

let result: MarkdownResponse = response.json().await?;
```

## Supported Markdown Features

- **GitHub Flavored Markdown** (tables, strikethrough, task lists)
- **Math expressions** with KaTeX
- **Code highlighting** with highlight.js
- **Emoji support** (:smile: → 😄)
- **Table of contents** generation
- **Custom blockquotes** (GitHub alerts)
- **HTML sanitization** for security