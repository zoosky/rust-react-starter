export interface ApiResponse<T> {
  success: boolean;
  data?: T;
  message?: string;
}

export interface HelloRequest {
  name: string;
}

export interface HelloResponse {
  greeting: string;
}

class ApiClient {
  private baseUrl: string;

  constructor(baseUrl: string = '/api') {
    this.baseUrl = baseUrl;
  }

  private async request<T>(
    endpoint: string,
    options: RequestInit = {}
  ): Promise<ApiResponse<T>> {
    const url = `${this.baseUrl}${endpoint}`;
    
    const config: RequestInit = {
      headers: {
        'Content-Type': 'application/json',
        ...options.headers,
      },
      ...options,
    };

    try {
      const response = await fetch(url, config);
      
      if (!response.ok) {
        throw new Error(`HTTP error! status: ${response.status}`);
      }
      
      return await response.json();
    } catch (error) {
      console.error('API request failed:', error);
      throw error;
    }
  }

  async healthCheck(): Promise<ApiResponse<string>> {
    return this.request<string>('/health');
  }

  async hello(request: HelloRequest): Promise<ApiResponse<HelloResponse>> {
    return this.request<HelloResponse>('/hello', {
      method: 'POST',
      body: JSON.stringify(request),
    });
  }
}

export const apiClient = new ApiClient();