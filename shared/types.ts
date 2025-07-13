// Shared types between frontend and backend

export interface ApiResponse<T> {
  success: boolean;
  data?: T;
  message?: string;
}

export interface HelloRequest {
  name: string;
}

export interface HelloResponse {
  message: string;
}

export interface HealthResponse {
  status: string;
  timestamp: string;
}

// Example of more complex shared types for a starter template
export interface User {
  id: string;
  name: string;
  email: string;
  created_at: string;
  updated_at: string;
}

export interface CreateUserRequest {
  name: string;
  email: string;
}

export interface UpdateUserRequest {
  name?: string;
  email?: string;
}

// Example project-related types
export interface Project {
  id: string;
  name: string;
  description?: string;
  created_at: string;
  updated_at: string;
}

export interface CreateProjectRequest {
  name: string;
  description?: string;
}

// Status types
export type Status = "active" | "inactive" | "pending" | "completed";

// Error types
export interface ErrorResponse {
  error: string;
  details?: string;
}