# Rust + React Starter Template

A full-stack starter template with Rust backend (using Axum) and React frontend (using Vite + TypeScript).

## 🚀 Features

- **Backend**: Rust with Axum web framework + SQLx + SQLite
- **Frontend**: React 18 + TypeScript + Vite + Tailwind CSS
- **Database**: SQLite with SQLx and automatic migrations
- **Package Management**: pnpm with workspace support
- **CORS**: Configured for local development
- **API Client**: Pre-configured TypeScript API client
- **Hot Reload**: Both backend and frontend support hot reloading

## 📁 Project Structure

```
rust-react-starter/
├── backend/                 # Rust backend
│   ├── src/
│   │   ├── main.rs         # Main server file with API endpoints
│   │   ├── database.rs     # Database connection and setup
│   │   ├── models/         # Database models
│   │   └── handlers/       # API route handlers
│   ├── migrations/         # SQLx database migrations
│   ├── Cargo.toml          # Rust dependencies
│   └── build.rs           # Build script
├── frontend/               # React frontend
│   ├── src/
│   │   ├── App.tsx         # Main React component
│   │   ├── main.tsx        # React entry point
│   │   └── lib/
│   │       └── api.ts      # API client
│   ├── package.json        # Frontend dependencies
│   ├── tailwind.config.js  # Tailwind CSS configuration
│   └── vite.config.ts      # Vite configuration
├── shared/                 # Shared TypeScript types
│   └── types.ts           # Common types between frontend and backend
├── Cargo.toml              # Workspace configuration
├── package.json            # Root package.json with scripts
└── pnpm-workspace.yaml     # pnpm workspace config
```

## 🛠️ Prerequisites

Make sure you have the following installed:

- [Rust](https://rustup.rs/) (latest stable)
- [Node.js](https://nodejs.org/) (v18 or higher)
- [pnpm](https://pnpm.io/installation)

## 🏃‍♂️ Quick Start

1. **Clone or copy this template**
   ```bash
   # If you downloaded as a template
   cd rust-react-starter
   ```

2. **Install frontend dependencies**
   ```bash
   pnpm install
   ```

3. **Start the development servers**
   
   **Option 1: Start both servers with one command**
   ```bash
   pnpm dev
   ```
   
   **Option 2: Start servers individually**
   ```bash
   # Terminal 1 - Backend (runs on http://localhost:3001)
   pnpm backend:dev
   
   # Terminal 2 - Frontend (runs on http://localhost:5173)
   pnpm frontend:dev
   ```

4. **Open your browser**
   - Frontend: http://localhost:5173
   - Backend API: http://localhost:3001

## 🔍 API Endpoints

The backend provides the following endpoints:

### Basic Endpoints
- `GET /api/health` - Health check endpoint
- `POST /api/hello` - Echo endpoint that returns a greeting

### User Management
- `GET /api/users` - Get all users
- `POST /api/users` - Create a new user
- `GET /api/users/:id` - Get a specific user
- `PUT /api/users/:id` - Update a user
- `DELETE /api/users/:id` - Delete a user

### Project Management
- `GET /api/projects` - Get all projects
- `POST /api/projects` - Create a new project

### Example API Usage

```bash
# Health check
curl http://localhost:3001/api/health

# Hello endpoint
curl -X POST http://localhost:3001/api/hello \
  -H "Content-Type: application/json" \
  -d '{"name": "World"}'

# Create a user
curl -X POST http://localhost:3001/api/users \
  -H "Content-Type: application/json" \
  -d '{"name": "John Doe", "email": "john@example.com"}'

# Get all users
curl http://localhost:3001/api/users

# Create a project
curl -X POST http://localhost:3001/api/projects \
  -H "Content-Type: application/json" \
  -d '{"name": "My Project", "description": "A sample project", "user_id": "USER_ID_HERE"}'
```

## 🎯 What's Included

### Backend (Rust + Axum + SQLx)
- Axum web framework with routing
- SQLite database with SQLx for async operations
- Automatic database migrations on startup
- CRUD operations for Users and Projects
- CORS middleware for frontend communication
- JSON serialization with serde
- Structured logging with tracing
- Type-safe database queries
- UUID-based primary keys

### Frontend (React + TypeScript + Tailwind)
- React 18 with TypeScript
- Tailwind CSS for styling with custom configuration
- Vite for fast development and building
- Pre-configured API client with error handling
- Example component that calls the backend API
- PostCSS configuration for Tailwind
- ESLint configuration

### Database (SQLite + SQLx)
- SQLite database for local development
- SQLx migrations with automatic execution
- User and Project models with relationships
- Timestamp tracking with automatic updates
- Type-safe database operations

## 📦 Available Scripts

From the root directory:

```bash
# Development
pnpm dev              # Start both backend and frontend
pnpm backend:dev      # Start only backend
pnpm frontend:dev     # Start only frontend

# Building
pnpm build            # Build both backend and frontend
pnpm backend:build    # Build only backend
pnpm frontend:build   # Build only frontend

# Testing & Linting
pnpm frontend:lint    # Lint frontend code
pnpm frontend:test    # Run frontend tests
```

## 🔧 Configuration

### Backend Configuration
- **Port**: 3001 (configurable via environment variable `PORT`)
- **Database**: SQLite database stored in `backend/data/app.db`
- **CORS**: Configured to allow requests from frontend (localhost:5173)
- **Migrations**: Automatically run on startup

### Frontend Configuration
- **Development Server**: Vite dev server on port 5173
- **API Proxy**: Configured in `vite.config.ts` to proxy `/api` requests to backend
- **TypeScript**: Strict mode enabled with modern ES features
- **Styling**: Tailwind CSS with custom configuration

## 🚀 Deployment

### Backend Deployment
1. Build the Rust binary:
   ```bash
   cd backend && cargo build --release
   ```
2. The binary will be available at `backend/target/release/rust-react-starter`

### Frontend Deployment
1. Build the frontend:
   ```bash
   cd frontend && pnpm build
   ```
2. The built files will be in `frontend/dist/`

## 📝 Next Steps

After setting up the template, you might want to:

1. **Authentication**
   - Implement JWT authentication
   - Add login/register endpoints
   - Add password hashing with bcrypt

2. **Enhanced Database**
   - Add more complex relationships
   - Implement database indexing
   - Add database connection pooling configuration

3. **State Management**
   - Add Zustand, Redux, or Context API for complex state
   - Implement optimistic updates

4. **Enhanced Styling**
   - Add custom Tailwind components
   - Implement dark mode support
   - Add animation libraries

5. **Testing**
   - Add backend tests with `tokio-test`
   - Add frontend tests with Vitest or Jest
   - Add integration tests

6. **API Improvements**
   - Add input validation with validator
   - Implement API versioning
   - Add rate limiting
   - Add API documentation with OpenAPI/Swagger

## 🤝 Contributing

Feel free to submit issues and enhancement requests!

## 📄 License

This project is open source and available under the [MIT License](LICENSE).