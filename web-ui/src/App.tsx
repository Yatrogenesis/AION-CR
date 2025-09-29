import React, { useState, useEffect } from 'react';
import { Routes, Route, Navigate } from 'react-router-dom';
import { ThemeProvider, createTheme } from '@mui/material/styles';
import { CssBaseline, Box, CircularProgress, Alert, Snackbar } from '@mui/material';
import { QueryClient, QueryClientProvider } from '@tanstack/react-query';
import { ReactQueryDevtools } from '@tanstack/react-query-devtools';

// Components
import Sidebar from './components/Layout/Sidebar';
import TopBar from './components/Layout/TopBar';
import Dashboard from './pages/Dashboard';
import Agents from './pages/Agents';
import Compliance from './pages/Compliance';
import Conflicts from './pages/Conflicts';
import MachineLearning from './pages/MachineLearning';
import Monitoring from './pages/Monitoring';
import Deployment from './pages/Deployment';
import Settings from './pages/Settings';
import Login from './pages/Login';

// Hooks
import { useAuth } from './hooks/useAuth';
import { useTheme } from './hooks/useTheme';
import { useNotifications } from './hooks/useNotifications';

// Types
import { SystemStatus, User } from './types';

// Services
import { systemApi } from './services/api';

// Styles
import './App.css';

const queryClient = new QueryClient({
  defaultOptions: {
    queries: {
      refetchOnWindowFocus: false,
      retry: 3,
      staleTime: 5 * 60 * 1000, // 5 minutes
    },
  },
});

interface AppState {
  isLoading: boolean;
  systemStatus: SystemStatus | null;
  error: string | null;
}

function App() {
  const { user, isAuthenticated, isLoading: authLoading } = useAuth();
  const { theme, toggleTheme } = useTheme();
  const { notifications, addNotification, removeNotification } = useNotifications();

  const [appState, setAppState] = useState<AppState>({
    isLoading: true,
    systemStatus: null,
    error: null,
  });

  const [sidebarOpen, setSidebarOpen] = useState(true);

  const muiTheme = createTheme({
    palette: {
      mode: theme,
      primary: {
        main: '#1976d2',
        dark: '#115293',
        light: '#42a5f5',
      },
      secondary: {
        main: '#dc004e',
        dark: '#9a0036',
        light: '#e5005a',
      },
      background: {
        default: theme === 'dark' ? '#0a0a0a' : '#f5f5f5',
        paper: theme === 'dark' ? '#1a1a1a' : '#ffffff',
      },
      text: {
        primary: theme === 'dark' ? '#ffffff' : '#000000',
        secondary: theme === 'dark' ? '#b0b0b0' : '#666666',
      },
    },
    typography: {
      fontFamily: '"Inter", "Roboto", "Helvetica", "Arial", sans-serif',
      h1: {
        fontSize: '2.5rem',
        fontWeight: 700,
        letterSpacing: '-0.01562em',
      },
      h2: {
        fontSize: '2rem',
        fontWeight: 600,
        letterSpacing: '-0.00833em',
      },
      h3: {
        fontSize: '1.75rem',
        fontWeight: 600,
        letterSpacing: '0em',
      },
      h4: {
        fontSize: '1.5rem',
        fontWeight: 600,
        letterSpacing: '0.00735em',
      },
      h5: {
        fontSize: '1.25rem',
        fontWeight: 600,
        letterSpacing: '0em',
      },
      h6: {
        fontSize: '1.125rem',
        fontWeight: 600,
        letterSpacing: '0.0075em',
      },
      body1: {
        fontSize: '1rem',
        lineHeight: 1.5,
      },
      body2: {
        fontSize: '0.875rem',
        lineHeight: 1.43,
      },
    },
    components: {
      MuiButton: {
        styleOverrides: {
          root: {
            textTransform: 'none',
            fontWeight: 600,
            borderRadius: 8,
            padding: '8px 16px',
          },
        },
      },
      MuiCard: {
        styleOverrides: {
          root: {
            borderRadius: 12,
            boxShadow: theme === 'dark'
              ? '0 4px 20px rgba(0, 0, 0, 0.5)'
              : '0 4px 20px rgba(0, 0, 0, 0.1)',
          },
        },
      },
      MuiPaper: {
        styleOverrides: {
          root: {
            borderRadius: 12,
          },
        },
      },
      MuiChip: {
        styleOverrides: {
          root: {
            borderRadius: 6,
            fontWeight: 500,
          },
        },
      },
    },
  });

  // Initialize system status
  useEffect(() => {
    const initializeApp = async () => {
      try {
        setAppState(prev => ({ ...prev, isLoading: true, error: null }));

        const status = await systemApi.getSystemStatus();

        setAppState({
          isLoading: false,
          systemStatus: status,
          error: null,
        });

        // Add welcome notification
        addNotification({
          id: Date.now().toString(),
          type: 'success',
          title: 'AION-CR System Ready',
          message: 'All systems operational with maximum autonomy enabled',
          timestamp: new Date(),
          autoHide: true,
        });

      } catch (error) {
        console.error('Failed to initialize app:', error);
        setAppState({
          isLoading: false,
          systemStatus: null,
          error: 'Failed to connect to AION-CR system',
        });

        addNotification({
          id: Date.now().toString(),
          type: 'error',
          title: 'System Error',
          message: 'Failed to connect to AION-CR system',
          timestamp: new Date(),
          autoHide: false,
        });
      }
    };

    if (isAuthenticated && !authLoading) {
      initializeApp();
    }
  }, [isAuthenticated, authLoading, addNotification]);

  // Real-time system monitoring
  useEffect(() => {
    if (!isAuthenticated || !appState.systemStatus) return;

    const interval = setInterval(async () => {
      try {
        const status = await systemApi.getSystemStatus();
        setAppState(prev => ({ ...prev, systemStatus: status }));

        // Check for system alerts
        if (status.alerts && status.alerts.length > 0) {
          status.alerts.forEach(alert => {
            addNotification({
              id: `alert-${alert.id}`,
              type: alert.severity === 'high' ? 'error' : alert.severity === 'medium' ? 'warning' : 'info',
              title: alert.title,
              message: alert.message,
              timestamp: new Date(),
              autoHide: alert.severity === 'low',
            });
          });
        }
      } catch (error) {
        console.error('System monitoring error:', error);
      }
    }, 30000); // Update every 30 seconds

    return () => clearInterval(interval);
  }, [isAuthenticated, appState.systemStatus, addNotification]);

  // Show loading screen during authentication
  if (authLoading) {
    return (
      <ThemeProvider theme={muiTheme}>
        <CssBaseline />
        <Box
          display="flex"
          justifyContent="center"
          alignItems="center"
          minHeight="100vh"
          flexDirection="column"
        >
          <CircularProgress size={60} thickness={4} />
          <Box mt={2} color="text.secondary">
            Initializing AION-CR System...
          </Box>
        </Box>
      </ThemeProvider>
    );
  }

  // Show login page if not authenticated
  if (!isAuthenticated) {
    return (
      <ThemeProvider theme={muiTheme}>
        <CssBaseline />
        <Login />
      </ThemeProvider>
    );
  }

  // Show loading screen during app initialization
  if (appState.isLoading) {
    return (
      <ThemeProvider theme={muiTheme}>
        <CssBaseline />
        <Box
          display="flex"
          justifyContent="center"
          alignItems="center"
          minHeight="100vh"
          flexDirection="column"
        >
          <CircularProgress size={60} thickness={4} />
          <Box mt={2} color="text.secondary">
            Loading AION-CR Dashboard...
          </Box>
        </Box>
      </ThemeProvider>
    );
  }

  // Show error state
  if (appState.error) {
    return (
      <ThemeProvider theme={muiTheme}>
        <CssBaseline />
        <Box
          display="flex"
          justifyContent="center"
          alignItems="center"
          minHeight="100vh"
          flexDirection="column"
          p={3}
        >
          <Alert severity="error" sx={{ mb: 2, maxWidth: 600 }}>
            {appState.error}
          </Alert>
          <Box color="text.secondary" textAlign="center">
            Please check your connection and try again
          </Box>
        </Box>
      </ThemeProvider>
    );
  }

  return (
    <QueryClientProvider client={queryClient}>
      <ThemeProvider theme={muiTheme}>
        <CssBaseline />
        <Box sx={{ display: 'flex', minHeight: '100vh' }}>
          {/* Sidebar */}
          <Sidebar
            open={sidebarOpen}
            onToggle={() => setSidebarOpen(!sidebarOpen)}
            systemStatus={appState.systemStatus}
          />

          {/* Main Content */}
          <Box
            component="main"
            sx={{
              flexGrow: 1,
              display: 'flex',
              flexDirection: 'column',
              transition: 'margin-left 0.3s ease',
              marginLeft: sidebarOpen ? '280px' : '80px',
            }}
          >
            {/* Top Bar */}
            <TopBar
              user={user}
              systemStatus={appState.systemStatus}
              onThemeToggle={toggleTheme}
              currentTheme={theme}
              onSidebarToggle={() => setSidebarOpen(!sidebarOpen)}
            />

            {/* Page Content */}
            <Box
              sx={{
                flexGrow: 1,
                p: 3,
                backgroundColor: 'background.default',
                minHeight: 'calc(100vh - 64px)',
              }}
            >
              <Routes>
                <Route path="/" element={<Navigate to="/dashboard" replace />} />
                <Route path="/dashboard" element={<Dashboard systemStatus={appState.systemStatus} />} />
                <Route path="/agents" element={<Agents />} />
                <Route path="/compliance" element={<Compliance />} />
                <Route path="/conflicts" element={<Conflicts />} />
                <Route path="/ml" element={<MachineLearning />} />
                <Route path="/monitoring" element={<Monitoring />} />
                <Route path="/deployment" element={<Deployment />} />
                <Route path="/settings" element={<Settings />} />
              </Routes>
            </Box>
          </Box>
        </Box>

        {/* Notifications */}
        {notifications.map((notification) => (
          <Snackbar
            key={notification.id}
            open={true}
            autoHideDuration={notification.autoHide ? 6000 : null}
            onClose={() => removeNotification(notification.id)}
            anchorOrigin={{ vertical: 'top', horizontal: 'right' }}
            sx={{ mt: 8 }}
          >
            <Alert
              onClose={() => removeNotification(notification.id)}
              severity={notification.type}
              variant="filled"
              sx={{ minWidth: 300 }}
            >
              <Box>
                <Box fontWeight="bold">{notification.title}</Box>
                <Box>{notification.message}</Box>
              </Box>
            </Alert>
          </Snackbar>
        ))}

        {/* React Query DevTools */}
        <ReactQueryDevtools initialIsOpen={false} />
      </ThemeProvider>
    </QueryClientProvider>
  );
}

export default App;