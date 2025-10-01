// AION-CR Login Page
// Maximum Autonomy Level 255 - Complete System Integration

import React, { useState } from 'react';
import {
  Box,
  Card,
  CardContent,
  TextField,
  Button,
  Typography,
  Alert,
  CircularProgress,
  Divider,
  InputAdornment,
  IconButton,
  FormControlLabel,
  Checkbox,
  Link,
} from '@mui/material';
import {
  Visibility,
  VisibilityOff,
  Security,
  SmartToy,
  Memory,
  AccountCircle,
  Lock,
} from '@mui/icons-material';
import { useAuth } from '../hooks/useAuth';

export default function Login() {
  const { login, isLoading, error, clearError } = useAuth();
  const [credentials, setCredentials] = useState({
    username: '',
    password: '',
  });
  const [showPassword, setShowPassword] = useState(false);
  const [rememberMe, setRememberMe] = useState(false);
  const [validationErrors, setValidationErrors] = useState<Record<string, string>>({});

  const handleChange = (field: string) => (event: React.ChangeEvent<HTMLInputElement>) => {
    setCredentials(prev => ({
      ...prev,
      [field]: event.target.value,
    }));

    // Clear validation error when user starts typing
    if (validationErrors[field]) {
      setValidationErrors(prev => ({
        ...prev,
        [field]: '',
      }));
    }

    // Clear auth error
    if (error) {
      clearError();
    }
  };

  const handleSubmit = async (event: React.FormEvent) => {
    event.preventDefault();

    // Validation
    const errors: Record<string, string> = {};
    if (!credentials.username.trim()) {
      errors.username = 'Username is required';
    }
    if (!credentials.password) {
      errors.password = 'Password is required';
    }

    if (Object.keys(errors).length > 0) {
      setValidationErrors(errors);
      return;
    }

    const result = await login(credentials);

    if (result.success && rememberMe) {
      localStorage.setItem('aion_remember_user', credentials.username);
    }
  };

  const handleTogglePasswordVisibility = () => {
    setShowPassword(!showPassword);
  };

  // System status indicators for visual appeal
  const systemIndicators = [
    { label: 'AI Engine', status: 'active', icon: <SmartToy /> },
    { label: 'Quantum Core', status: 'active', icon: <Memory /> },
    { label: 'Security Matrix', status: 'active', icon: <Security /> },
  ];

  return (
    <Box
      sx={{
        minHeight: '100vh',
        background: 'linear-gradient(135deg, #0f1419 0%, #1a237e 50%, #000051 100%)',
        display: 'flex',
        alignItems: 'center',
        justifyContent: 'center',
        padding: 2,
        position: 'relative',
        overflow: 'hidden',
      }}
    >
      {/* Animated background elements */}
      <Box
        sx={{
          position: 'absolute',
          top: 0,
          left: 0,
          right: 0,
          bottom: 0,
          opacity: 0.1,
          background: `
            radial-gradient(circle at 20% 80%, #00bcd4 0%, transparent 50%),
            radial-gradient(circle at 80% 20%, #3f51b5 0%, transparent 50%),
            radial-gradient(circle at 40% 40%, #9c27b0 0%, transparent 50%)
          `,
          animation: 'pulse 4s ease-in-out infinite',
        }}
      />

      <Box sx={{ width: '100%', maxWidth: 400, zIndex: 1 }}>
        {/* Header */}
        <Box textAlign="center" mb={4}>
          <Box
            sx={{
              display: 'inline-flex',
              alignItems: 'center',
              justifyContent: 'center',
              width: 80,
              height: 80,
              borderRadius: 2,
              background: 'linear-gradient(45deg, #1976d2, #42a5f5)',
              mb: 2,
              boxShadow: '0 8px 32px rgba(25, 118, 210, 0.3)',
            }}
          >
            <SmartToy sx={{ fontSize: 40, color: 'white' }} />
          </Box>

          <Typography variant="h3" fontWeight="bold" color="white" gutterBottom>
            AION-CR
          </Typography>
          <Typography variant="h6" color="rgba(255, 255, 255, 0.8)" gutterBottom>
            Autonomous Intelligence Operations Network
          </Typography>
          <Typography variant="body2" color="rgba(255, 255, 255, 0.6)">
            Compliance Regulation System
          </Typography>
        </Box>

        {/* System Status Indicators */}
        <Box display="flex" justifyContent="center" gap={2} mb={4}>
          {systemIndicators.map((indicator, index) => (
            <Box
              key={index}
              sx={{
                display: 'flex',
                flexDirection: 'column',
                alignItems: 'center',
                gap: 1,
                p: 2,
                borderRadius: 2,
                backgroundColor: 'rgba(255, 255, 255, 0.1)',
                backdropFilter: 'blur(10px)',
                border: '1px solid rgba(255, 255, 255, 0.2)',
              }}
            >
              <Box sx={{ color: '#4caf50' }}>
                {indicator.icon}
              </Box>
              <Typography variant="caption" color="white" textAlign="center">
                {indicator.label}
              </Typography>
              <Box
                sx={{
                  width: 8,
                  height: 8,
                  borderRadius: '50%',
                  backgroundColor: '#4caf50',
                  animation: 'pulse 2s infinite',
                }}
              />
            </Box>
          ))}
        </Box>

        {/* Login Form */}
        <Card
          sx={{
            backgroundColor: 'rgba(255, 255, 255, 0.95)',
            backdropFilter: 'blur(20px)',
            border: '1px solid rgba(255, 255, 255, 0.3)',
            boxShadow: '0 8px 32px rgba(0, 0, 0, 0.3)',
          }}
        >
          <CardContent sx={{ p: 4 }}>
            <Typography variant="h5" fontWeight="600" gutterBottom>
              System Access
            </Typography>
            <Typography variant="body2" color="text.secondary" mb={3}>
              Enter your credentials to access the AION-CR system
            </Typography>

            {error && (
              <Alert severity="error" sx={{ mb: 3 }}>
                {error}
              </Alert>
            )}

            <Box component="form" onSubmit={handleSubmit}>
              <TextField
                fullWidth
                label="Username"
                variant="outlined"
                value={credentials.username}
                onChange={handleChange('username')}
                error={!!validationErrors.username}
                helperText={validationErrors.username}
                disabled={isLoading}
                InputProps={{
                  startAdornment: (
                    <InputAdornment position="start">
                      <AccountCircle />
                    </InputAdornment>
                  ),
                }}
                sx={{ mb: 3 }}
              />

              <TextField
                fullWidth
                label="Password"
                type={showPassword ? 'text' : 'password'}
                variant="outlined"
                value={credentials.password}
                onChange={handleChange('password')}
                error={!!validationErrors.password}
                helperText={validationErrors.password}
                disabled={isLoading}
                InputProps={{
                  startAdornment: (
                    <InputAdornment position="start">
                      <Lock />
                    </InputAdornment>
                  ),
                  endAdornment: (
                    <InputAdornment position="end">
                      <IconButton
                        onClick={handleTogglePasswordVisibility}
                        edge="end"
                        disabled={isLoading}
                      >
                        {showPassword ? <VisibilityOff /> : <Visibility />}
                      </IconButton>
                    </InputAdornment>
                  ),
                }}
                sx={{ mb: 3 }}
              />

              <FormControlLabel
                control={
                  <Checkbox
                    checked={rememberMe}
                    onChange={(e) => setRememberMe(e.target.checked)}
                    disabled={isLoading}
                  />
                }
                label="Remember me"
                sx={{ mb: 3 }}
              />

              <Button
                type="submit"
                fullWidth
                variant="contained"
                size="large"
                disabled={isLoading}
                sx={{
                  py: 1.5,
                  background: 'linear-gradient(45deg, #1976d2, #42a5f5)',
                  '&:hover': {
                    background: 'linear-gradient(45deg, #1565c0, #1976d2)',
                  },
                }}
              >
                {isLoading ? (
                  <CircularProgress size={24} color="inherit" />
                ) : (
                  'Access System'
                )}
              </Button>

              <Divider sx={{ my: 3 }}>
                <Typography variant="caption" color="text.secondary">
                  Security Level: Maximum
                </Typography>
              </Divider>

              <Box textAlign="center">
                <Link
                  href="#"
                  variant="body2"
                  color="primary"
                  sx={{ textDecoration: 'none' }}
                >
                  Forgot your credentials?
                </Link>
              </Box>
            </Box>
          </CardContent>
        </Card>

        {/* Footer */}
        <Box textAlign="center" mt={4}>
          <Typography variant="caption" color="rgba(255, 255, 255, 0.6)">
            AION-CR v1.0.0 | Autonomy Level 255 | Secure Access Portal
          </Typography>
        </Box>
      </Box>

      <style>
        {`
          @keyframes pulse {
            0%, 100% { opacity: 0.1; }
            50% { opacity: 0.3; }
          }
        `}
      </style>
    </Box>
  );
}