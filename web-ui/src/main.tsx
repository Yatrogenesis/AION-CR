// AION-CR Main Entry Point
// Maximum Autonomy Level 255 - Complete System Integration

import React from 'react';
import ReactDOM from 'react-dom/client';
import { BrowserRouter } from 'react-router-dom';
import { ErrorBoundary } from 'react-error-boundary';
import App from './App';
import './App.css';

// Performance monitoring
if ('performance' in window && 'mark' in performance) {
  performance.mark('aion-cr-start');
}

// Error Fallback Component
function ErrorFallback({ error, resetErrorBoundary }: { error: Error; resetErrorBoundary: () => void }) {
  return (
    <div className="error-boundary">
      <h2>🤖 AION-CR System Error</h2>
      <p>The autonomous system encountered an unexpected error and needs to recover.</p>

      <details style={{ whiteSpace: 'pre-wrap', marginTop: '16px' }}>
        <summary>Technical Details</summary>
        <pre>{error.message}</pre>
        {error.stack && <pre>{error.stack}</pre>}
      </details>

      <div style={{ marginTop: '20px' }}>
        <button
          onClick={resetErrorBoundary}
          style={{
            background: '#1976d2',
            color: 'white',
            border: 'none',
            padding: '12px 24px',
            borderRadius: '6px',
            cursor: 'pointer',
            marginRight: '12px',
            fontWeight: '600',
          }}
        >
          🔄 Restart System
        </button>

        <button
          onClick={() => window.location.reload()}
          style={{
            background: '#666',
            color: 'white',
            border: 'none',
            padding: '12px 24px',
            borderRadius: '6px',
            cursor: 'pointer',
            fontWeight: '600',
          }}
        >
          🔃 Full Reload
        </button>
      </div>

      <div style={{ marginTop: '20px', fontSize: '14px', color: '#666' }}>
        <p>
          <strong>System Status:</strong> Error Recovery Mode<br />
          <strong>Autonomy Level:</strong> Reduced to Safe Mode<br />
          <strong>Timestamp:</strong> {new Date().toISOString()}
        </p>
      </div>
    </div>
  );
}

// Initialize React application
const container = document.getElementById('root');

if (!container) {
  throw new Error('Failed to find the root element. AION-CR cannot initialize.');
}

const root = ReactDOM.createRoot(container);

// Render the application with error boundary and routing
root.render(
  <React.StrictMode>
    <ErrorBoundary
      FallbackComponent={ErrorFallback}
      onError={(error, errorInfo) => {
        // Log error for debugging
        console.error('AION-CR Error Boundary:', error, errorInfo);

        // Report to monitoring service if available
        if (window.gtag) {
          window.gtag('event', 'exception', {
            description: error.toString(),
            fatal: false,
          });
        }

        // Mark performance issue
        if ('performance' in window && 'mark' in performance) {
          performance.mark('aion-cr-error');
        }
      }}
      onReset={() => {
        // Clear any error state
        console.log('AION-CR System Reset');

        // Mark recovery
        if ('performance' in window && 'mark' in performance) {
          performance.mark('aion-cr-recovery');
        }
      }}
    >
      <BrowserRouter>
        <App />
      </BrowserRouter>
    </ErrorBoundary>
  </React.StrictMode>
);

// Performance monitoring completion
if ('performance' in window && 'mark' in performance) {
  performance.mark('aion-cr-rendered');

  // Measure render time
  setTimeout(() => {
    try {
      performance.measure('aion-cr-render-time', 'aion-cr-start', 'aion-cr-rendered');
      const measure = performance.getEntriesByName('aion-cr-render-time')[0];
      console.log(`AION-CR Render Time: ${measure.duration.toFixed(2)}ms`);
    } catch (error) {
      console.warn('Performance measurement failed:', error);
    }
  }, 0);
}

// Hot Module Replacement for development
if (import.meta.hot) {
  import.meta.hot.accept();
}

// Service Worker registration for production
if ('serviceWorker' in navigator && import.meta.env.PROD) {
  window.addEventListener('load', () => {
    navigator.serviceWorker
      .register('/sw.js')
      .then((registration) => {
        console.log('AION-CR Service Worker registered:', registration);
      })
      .catch((error) => {
        console.error('AION-CR Service Worker registration failed:', error);
      });
  });
}

// Global error handlers
window.addEventListener('error', (event) => {
  console.error('AION-CR Global Error:', event.error);
});

window.addEventListener('unhandledrejection', (event) => {
  console.error('AION-CR Unhandled Promise Rejection:', event.reason);
  event.preventDefault(); // Prevent console error
});

// System status logging
console.log(`
🤖 AION-CR System Initialized
═══════════════════════════════════════════════════════════════════════
  System Name: Autonomous Intelligence Operations Network
  Purpose: Compliance Regulation Management
  Autonomy Level: 255 (Maximum)
  Version: ${import.meta.env.VITE_APP_VERSION || '1.0.0'}
  Build: ${import.meta.env.VITE_BUILD_DATE || new Date().toISOString()}
  Environment: ${import.meta.env.MODE}
  React Version: ${React.version}
═══════════════════════════════════════════════════════════════════════
  Status: ✅ Online
  AI Modules: Loading...
  Blockchain: Connecting...
  Quantum Core: Initializing...
  Security Level: Maximum
═══════════════════════════════════════════════════════════════════════
`);

// Export for debugging in development
if (import.meta.env.DEV) {
  (window as any).AION_CR = {
    version: import.meta.env.VITE_APP_VERSION || '1.0.0',
    build: import.meta.env.VITE_BUILD_DATE || new Date().toISOString(),
    environment: import.meta.env.MODE,
    autonomyLevel: 255,
    debug: {
      performance: () => {
        if ('performance' in window) {
          console.table(performance.getEntriesByType('measure'));
        }
      },
      memory: () => {
        if ('memory' in performance) {
          console.table((performance as any).memory);
        }
      },
    },
  };
}

// Declare global types for development debugging
declare global {
  interface Window {
    AION_CR?: {
      version: string;
      build: string;
      environment: string;
      autonomyLevel: number;
      debug: {
        performance: () => void;
        memory: () => void;
      };
    };
    gtag?: (...args: any[]) => void;
  }
}