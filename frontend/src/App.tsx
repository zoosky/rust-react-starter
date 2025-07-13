import { useState, useEffect } from 'react';
import { apiClient, ApiResponse, HelloResponse } from './lib/api';
import './App.css';

function App() {
  const [name, setName] = useState('');
  const [greeting, setGreeting] = useState('');
  const [loading, setLoading] = useState(false);
  const [healthStatus, setHealthStatus] = useState<string>('');

  useEffect(() => {
    // Check health on component mount
    const checkHealth = async () => {
      try {
        const response = await apiClient.healthCheck();
        if (response.success && response.data) {
          setHealthStatus(`Backend status: ${response.data}`);
        }
      } catch (error) {
        setHealthStatus('Backend is not responding');
        console.error('Health check failed:', error);
      }
    };

    checkHealth();
  }, []);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    
    if (!name.trim()) {
      alert('Please enter your name');
      return;
    }

    setLoading(true);
    try {
      const response: ApiResponse<HelloResponse> = await apiClient.hello({ name });
      
      if (response.success && response.data) {
        setGreeting(response.data.greeting);
      } else {
        throw new Error(response.message || 'Failed to get greeting');
      }
    } catch (error) {
      console.error('Failed to get greeting:', error);
      alert('Failed to get greeting from server');
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="App">
      <header className="App-header">
        <h1>🦀 Rust + React Starter</h1>
        <p className="status">{healthStatus}</p>
        
        <div className="greeting-form">
          <h2>Test the API</h2>
          <form onSubmit={handleSubmit}>
            <div className="input-group">
              <label htmlFor="name">Enter your name:</label>
              <input
                id="name"
                type="text"
                value={name}
                onChange={(e) => setName(e.target.value)}
                placeholder="Your name"
                disabled={loading}
              />
            </div>
            <button type="submit" disabled={loading}>
              {loading ? 'Getting greeting...' : 'Get Greeting'}
            </button>
          </form>
          
          {greeting && (
            <div className="greeting-result">
              <h3>Response from Rust backend:</h3>
              <p>{greeting}</p>
            </div>
          )}
        </div>
      </header>
    </div>
  );
}

export default App;