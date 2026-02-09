# 🐾 Mofumofu UI

<div align="center">

<img src="../assets/mofumofu_kawaii_mini.svg" alt="もふもふ" width="300"/>

The frontend component library for Mofumofu - a soft and simple blogging platform.

[![Svelte](https://img.shields.io/badge/Built%20with-Svelte%205-ff3e00.svg)](https://svelte.dev)
[![TypeScript](https://img.shields.io/badge/TypeScript-Ready-blue.svg)](https://www.typescriptlang.org/)
[![TailwindCSS](https://img.shields.io/badge/Styled%20with-TailwindCSS%204-38bdf8.svg)](https://tailwindcss.com)

</div>

## ✨ Overview

**mofumofu-ui** is the frontend component library for [Mofumofu](../README.md), an open-source minimalist blogging platform. Built with Svelte 5, it provides the complete user interface for the blogging experience including post creation, user profiles, authentication, and responsive design components.

### 🎯 Key Features

- **🔥 Svelte 5 Native** - Built from the ground up with Svelte 5 runes (`$state`, `$props`, `$derived`)
- **📱 Responsive Design** - Mobile-first components with seamless desktop experience
- **🎨 Modern UI** - Beautiful components built with TailwindCSS 4 and accessible primitives
- **🌍 Internationalization** - Multi-language support with Paraglide.js
- **📝 Rich Content** - Advanced markdown processing with math, syntax highlighting, and more
- **🔐 Authentication Ready** - Complete auth system with OAuth integration
- **⚡ Performance Optimized** - Infinite scroll, skeleton states, and optimized rendering
- **📦 Type Safe** - Full TypeScript support with Valibot schema validation

## 🚀 Getting Started

This is the frontend application for the Mofumofu platform. To run the full stack locally, you'll need both the backend API and this frontend application.

### Prerequisites

- **Node.js** 18+
- **pnpm** (recommended package manager)
- **Mofumofu Backend** running on your system

### Development Setup

```bash
# Clone the entire Mofumofu repository
git clone <repository-url>
cd mofu/mofumofu-ui

# Install dependencies
pnpm install

# Set up environment variables
cp .env.example .env.local
# Edit .env.local with your API URL (usually http://localhost:3000)

# Start development server
pnpm dev
```

### Configuration

Create a `.env.local` file with:

```env
PUBLIC_API_URL=http://localhost:3000  # Your backend API URL
```

## 🏗️ Architecture

### 📁 Project Structure

```
src/lib/
├── api/           # HTTP client with organized endpoints
│   ├── auth/      # Authentication APIs
│   ├── user/      # User management
│   ├── post/      # Content APIs
│   └── follow/    # Social features
├── components/    # Reusable Svelte 5 components
│   ├── ui/        # Low-level UI primitives
│   ├── post/      # Post-related components
│   ├── profile/   # User profile components
│   └── settings/  # Settings forms
├── hooks/         # Custom Svelte 5 runes
├── stores/        # Global state management
├── utils/         # Utility functions
└── schemas/       # Validation schemas
```

### 🧩 Component Categories

#### Core UI Components

- **Button, Input, Textarea** - Form controls with variants
- **Dialog, Drawer, Popover** - Overlay components
- **Badge, Card, Separator** - Content display
- **Calendar, Select, Command** - Advanced inputs

#### Domain Components

- **PostCard, PostList** - Content display with skeleton states
- **Navbar** - Responsive navigation with scroll behavior
- **ProfileHeader, ProfileInfo** - User profile components
- **WriteEditor, WritePreview** - Content creation interface

#### Settings Components

- **PersonalInfoSettings** - Profile management with image upload
- **AccountSettings** - Authentication and security
- **DisplaySettings** - Theme and language preferences

## 🔧 Development

### Prerequisites

- **Node.js** 18+
- **pnpm** (recommended package manager)

### Setup

```bash
# Clone the repository
git clone <repository-url>
cd mofumofu-ui

# Install dependencies
pnpm install

# Start development server
pnpm dev
```

### Available Scripts

| Command            | Description                               |
| ------------------ | ----------------------------------------- |
| `pnpm dev`         | Start development server for showcase app |
| `pnpm build`       | Build library and production app          |
| `pnpm preview`     | Preview production build                  |
| `pnpm check`       | Run Svelte type checking                  |
| `pnpm check:watch` | Type checking in watch mode               |
| `pnpm fmt`         | Format code with Prettier                 |
| `pnpm lint`        | Check code formatting                     |
| `pnpm prepack`     | Build library package                     |

## 🛠️ Technology Stack

### Core Technologies

- **[Svelte 5](https://svelte.dev/)** - Reactive UI framework with runes
- **[SvelteKit](https://kit.svelte.dev/)** - Full-stack framework
- **[TypeScript](https://www.typescriptlang.org/)** - Type safety
- **[TailwindCSS 4](https://tailwindcss.com/)** - Utility-first styling
- **[Vite](https://vitejs.dev/)** - Build tool and dev server

### Key Libraries

- **[bits-ui](https://www.bits-ui.com/)** - Accessible component primitives
- **[ky](https://github.com/sindresorhus/ky)** - HTTP client
- **[valibot](https://valibot.dev/)** - Schema validation
- **[mode-watcher](https://github.com/svecosystem/mode-watcher)** - Theme management
- **[unified](https://unifiedjs.com/)** - Markdown processing pipeline

### Markdown Features

- **GitHub Flavored Markdown** (GFM)
- **Math rendering** (KaTeX)
- **Syntax highlighting** (highlight.js)
- **Emoji support** 🎉
- **Table of contents generation**
- **GitHub-style blockquote alerts**

## 🎨 Styling & Theming

Mofumofu UI uses TailwindCSS 4 with a custom design system:

- **Responsive breakpoints** - Mobile-first approach
- **Dark/light themes** - Automatic system preference detection
- **Custom animations** - Powered by `tw-animate-css`
- **Consistent spacing** - Design token system
- **Accessible colors** - WCAG compliant color palette

## 🌍 Internationalization

Built-in support for multiple languages:

- **English** (default)
- **Korean** (한국어)
- **Japanese** (日本語)
- **German** (Deutsch)
- **French** (Français)
- **Spanish** (Español)
- **Russian** (Русский)

## 📊 State Management

### Stores

- **`auth.svelte.ts`** - Authentication state with persistence
- **`posts.svelte.ts`** - Post data and filtering
- **`settings.svelte.ts`** - User preferences
- **`user.svelte.ts`** - User profile data

### Hooks

- **`useInfiniteScroll`** - Pagination and loading
- **`useNavbarScroll`** - Navigation visibility
- **`useWriteEditor`** - Content creation state

## 🔐 Authentication

Complete authentication system including:

- **Email/password** authentication
- **OAuth integration** (GitHub, Google)
- **Password reset** flow
- **Email verification**
- **Session management**

## 📱 Responsive Design

- **Mobile-first** approach
- **Adaptive layouts** for different screen sizes
- **Touch-friendly** interactions
- **Optimized performance** on mobile devices

## 🚢 Deployment

This frontend is built for **Cloudflare Workers** with:

- **Edge computing** capabilities
- **Global CDN** distribution
- **Serverless** architecture
- **Zero cold start** deployments

### Building for Production

```bash
pnpm build    # Builds both app and library package
pnpm preview  # Preview the production build
```

## 🔗 Related Projects

- **[Mofumofu Backend](../mofumofu-backend)** - Rust/Axum API server
- **[Mofumofu Desktop](../mofumofu-desktop)** - Tauri desktop application
- **[Main Repository](../)** - Complete platform overview

## 🤝 Contributing

We welcome contributions! Please see the main repository's contributing guidelines for more details.

## 📄 License

This project is part of the Mofumofu platform and is licensed under the Apache License 2.0.  
See the [LICENSE](../LICENSE) file for details.

## 🐾 About Mofumofu

"Mofumofu" (もふもふ) is a Japanese onomatopoeia describing something soft and fluffy. We believe the internet should be a space for genuine thought and expression, not optimized for engagement metrics. Mofumofu offers a quieter place to write, built for clarity, ownership, and simplicity.

---

<div align="center">
Made with 💜 by the Mofumofu team
</div>
