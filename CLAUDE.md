# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Development Commands

### Starting Development Servers
- `pnpm dev` - Start both backend and frontend servers concurrently
- `pnpm backend:dev` - Start only the Rust backend (port 3001)
- `pnpm frontend:dev` - Start only the React frontend (port 5173)
- `pnpm backend:dev:watch` - Start backend with cargo-watch for auto-reload

### Building
- `pnpm build` - Build both frontend and backend for production
- `pnpm frontend:build` - Build frontend only (outputs to frontend/dist/)
- `pnpm backend:build` - Build backend only (release binary to backend/target/release/)

### Frontend-Specific Commands
- `cd frontend && pnpm lint` - Lint frontend TypeScript/React code
- `cd frontend && pnpm lint:fix` - Auto-fix linting issues

### Backend-Specific Commands
- `cd backend && cargo run` - Run backend in development mode
- `cd backend && cargo build --release` - Build optimized production binary
- `cd backend && cargo test` - Run Rust tests

## Architecture Overview

### Full-Stack Monorepo Structure
This is a Rust + React starter template organized as a monorepo with separate frontend and backend projects:

- **Backend**: Rust with Axum web framework, SQLx ORM, and SQLite database
- **Frontend**: React 18 + TypeScript + Vite + Tailwind CSS
- **Shared**: Common TypeScript types in `/shared/types.ts`
- **Database**: SQLite with automatic migrations via SQLx

### Key Backend Architecture (backend/src/)
- `main.rs`: Axum server setup with API routes and CORS configuration
- `database.rs`: SQLite connection pool management and migration runner
- `models/`: Database models (User, Project) with SQLx derive macros
- `handlers/`: API route handlers organized by entity (users.rs, projects.rs)
- `migrations/`: SQLx migration files for database schema changes

### Key Frontend Architecture (frontend/src/)
- `App.tsx`: Main React component
- `lib/api.ts`: Type-safe API client with error handling
- Vite configuration with API proxy to backend (`/api` → `localhost:3001`)
- Tailwind CSS for styling with PostCSS configuration

### Database Design
- SQLite database stored in `backend/data/app.db`
- UUID-based primary keys for all entities
- Automatic timestamp triggers for `updated_at` fields
- Foreign key relationships with cascade deletes

### API Structure
All backend endpoints are prefixed with `/api`:
- Health/utility: `/api/health`, `/api/hello`
- Users: `/api/users` (CRUD operations)
- Projects: `/api/projects` (with user relationships)

### Type Safety
- Shared TypeScript types in `/shared/types.ts` ensure consistency between frontend and backend
- Rust backend uses serde for JSON serialization/deserialization
- Frontend API client is fully typed with TypeScript generics

### Development Workflow
1. The project uses pnpm workspaces with frontend as a workspace member
2. SQLx migrations run automatically on backend startup
3. Frontend development server proxies API calls to backend
4. CORS is configured permissively for local development