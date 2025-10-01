// AION-CR TopBar Component
// Maximum Autonomy Level 255 - Complete System Integration

import React, { useState } from 'react';
import {
  AppBar,
  Toolbar,
  IconButton,
  Typography,
  Box,
  Avatar,
  Menu,
  MenuItem,
  Badge,
  Chip,
  Tooltip,
  Button,
  Divider,
  ListItemIcon,
  ListItemText,
} from '@mui/material';
import {
  Menu as MenuIcon,
  Notifications as NotificationsIcon,
  AccountCircle,
  Settings,
  Logout,
  DarkMode,
  LightMode,
  Security,
  Speed,
  Memory,
  NetworkCheck,
  Warning,
  CheckCircle,
  Error,
  Info,
} from '@mui/icons-material';
import { SystemStatus, User } from '../../types';

interface TopBarProps {
  user: User | null;
  systemStatus: SystemStatus | null;
  onThemeToggle: () => void;
  currentTheme: 'light' | 'dark';
  onSidebarToggle: () => void;
}

export default function TopBar({
  user,
  systemStatus,
  onThemeToggle,
  currentTheme,
  onSidebarToggle,
}: TopBarProps) {
  const [userMenuAnchor, setUserMenuAnchor] = useState<null | HTMLElement>(null);
  const [notificationsAnchor, setNotificationsAnchor] = useState<null | HTMLElement>(null);

  const handleUserMenuOpen = (event: React.MouseEvent<HTMLElement>) => {
    setUserMenuAnchor(event.currentTarget);
  };

  const handleUserMenuClose = () => {
    setUserMenuAnchor(null);
  };

  const handleNotificationsOpen = (event: React.MouseEvent<HTMLElement>) => {
    setNotificationsAnchor(event.currentTarget);
  };

  const handleNotificationsClose = () => {
    setNotificationsAnchor(null);
  };

  const handleLogout = () => {
    handleUserMenuClose();
    // Logout logic would be handled by parent component
    window.location.href = '/login';
  };

  const getStatusIcon = (status: string) => {
    switch (status) {
      case 'online': return <CheckCircle color="success" />;
      case 'degraded': return <Warning color="warning" />;
      case 'offline': return <Error color="error" />;
      case 'maintenance': return <Info color="info" />;
      default: return <Info />;
    }
  };

  const getStatusColor = (status: string) => {
    switch (status) {
      case 'online': return 'success';
      case 'degraded': return 'warning';
      case 'offline': return 'error';
      case 'maintenance': return 'info';
      default: return 'default';
    }
  };

  const formatUptime = (uptime: number) => {
    const days = Math.floor(uptime / (24 * 60 * 60 * 1000));
    const hours = Math.floor((uptime % (24 * 60 * 60 * 1000)) / (60 * 60 * 1000));
    const minutes = Math.floor((uptime % (60 * 60 * 1000)) / (60 * 1000));

    if (days > 0) return `${days}d ${hours}h`;
    if (hours > 0) return `${hours}h ${minutes}m`;
    return `${minutes}m`;
  };

  const activeAlerts = systemStatus?.alerts?.filter(alert => !alert.resolved).length || 0;

  return (
    <AppBar
      position="static"
      elevation={1}
      sx={{
        backgroundColor: 'background.paper',
        borderBottom: '1px solid',
        borderColor: 'divider',
        color: 'text.primary',
      }}
    >
      <Toolbar sx={{ justifyContent: 'space-between', px: 3 }}>
        {/* Left Section */}
        <Box display="flex" alignItems="center">
          <IconButton
            edge="start"
            color="inherit"
            onClick={onSidebarToggle}
            sx={{ mr: 2 }}
          >
            <MenuIcon />
          </IconButton>

          {/* System Status Indicators */}
          {systemStatus && (
            <Box display="flex" alignItems="center" gap={2}>
              <Tooltip title={`System Status: ${systemStatus.status}`}>
                <Chip
                  icon={getStatusIcon(systemStatus.status)}
                  label={systemStatus.status.toUpperCase()}
                  size="small"
                  color={getStatusColor(systemStatus.status) as any}
                  variant="outlined"
                />
              </Tooltip>

              <Tooltip title={`Autonomy Level: ${systemStatus.autonomyLevel}/255`}>
                <Chip
                  icon={<Security />}
                  label={`L${systemStatus.autonomyLevel}`}
                  size="small"
                  color={systemStatus.autonomyLevel >= 200 ? 'error' : 'primary'}
                  variant="filled"
                />
              </Tooltip>

              <Tooltip title={`Uptime: ${formatUptime(systemStatus.uptime)}`}>
                <Chip
                  icon={<Speed />}
                  label={formatUptime(systemStatus.uptime)}
                  size="small"
                  variant="outlined"
                />
              </Tooltip>

              {systemStatus.health && (
                <Tooltip title={`CPU: ${systemStatus.health.cpu}% | Memory: ${systemStatus.health.memory}%`}>
                  <Chip
                    icon={<Memory />}
                    label={`${systemStatus.health.cpu}%`}
                    size="small"
                    color={systemStatus.health.cpu > 80 ? 'warning' : 'default'}
                    variant="outlined"
                  />
                </Tooltip>
              )}

              {systemStatus.services && (
                <Tooltip title={`${systemStatus.services.filter(s => s.status === 'running').length}/${systemStatus.services.length} services running`}>
                  <Chip
                    icon={<NetworkCheck />}
                    label={`${systemStatus.services.filter(s => s.status === 'running').length}/${systemStatus.services.length}`}
                    size="small"
                    color="info"
                    variant="outlined"
                  />
                </Tooltip>
              )}
            </Box>
          )}
        </Box>

        {/* Center Section - Current Time and Version */}
        <Box display="flex" alignItems="center" gap={2}>
          <Typography variant="body2" color="text.secondary">
            {new Date().toLocaleTimeString()}
          </Typography>
          {systemStatus && (
            <Typography variant="caption" color="text.secondary">
              v{systemStatus.version}
            </Typography>
          )}
        </Box>

        {/* Right Section */}
        <Box display="flex" alignItems="center" gap={1}>
          {/* Theme Toggle */}
          <Tooltip title={`Switch to ${currentTheme === 'light' ? 'dark' : 'light'} mode`}>
            <IconButton color="inherit" onClick={onThemeToggle}>
              {currentTheme === 'light' ? <DarkMode /> : <LightMode />}
            </IconButton>
          </Tooltip>

          {/* Notifications */}
          <Tooltip title="Notifications">
            <IconButton color="inherit" onClick={handleNotificationsOpen}>
              <Badge badgeContent={activeAlerts} color="error">
                <NotificationsIcon />
              </Badge>
            </IconButton>
          </Tooltip>

          {/* User Menu */}
          <Button
            onClick={handleUserMenuOpen}
            startIcon={
              <Avatar
                sx={{ width: 32, height: 32 }}
                src={user?.preferences?.avatar}
              >
                {user?.username?.[0]?.toUpperCase() || 'U'}
              </Avatar>
            }
            color="inherit"
            sx={{ textTransform: 'none' }}
          >
            <Box display="flex" flexDirection="column" alignItems="flex-start" ml={1}>
              <Typography variant="body2" fontWeight="600">
                {user?.username || 'User'}
              </Typography>
              <Typography variant="caption" color="text.secondary">
                {user?.role || 'operator'}
              </Typography>
            </Box>
          </Button>
        </Box>

        {/* Notifications Menu */}
        <Menu
          anchorEl={notificationsAnchor}
          open={Boolean(notificationsAnchor)}
          onClose={handleNotificationsClose}
          PaperProps={{
            sx: { width: 320, maxHeight: 400 }
          }}
        >
          <Box sx={{ p: 2, borderBottom: '1px solid', borderColor: 'divider' }}>
            <Typography variant="h6">Notifications</Typography>
            <Typography variant="body2" color="text.secondary">
              {activeAlerts} active alerts
            </Typography>
          </Box>

          {systemStatus?.alerts?.slice(0, 5).map((alert, index) => (
            <MenuItem key={alert.id} dense>
              <ListItemIcon>
                {alert.severity === 'critical' ? <Error color="error" /> :
                 alert.severity === 'high' ? <Warning color="warning" /> :
                 <Info color="info" />}
              </ListItemIcon>
              <ListItemText
                primary={alert.title}
                secondary={alert.message}
                primaryTypographyProps={{ fontSize: '0.875rem' }}
                secondaryTypographyProps={{ fontSize: '0.75rem' }}
              />
            </MenuItem>
          )) || (
            <MenuItem disabled>
              <ListItemText primary="No notifications" />
            </MenuItem>
          )}

          {activeAlerts > 5 && (
            <MenuItem onClick={handleNotificationsClose}>
              <ListItemText
                primary={`View ${activeAlerts - 5} more alerts...`}
                primaryTypographyProps={{ color: 'primary.main' }}
              />
            </MenuItem>
          )}
        </Menu>

        {/* User Menu */}
        <Menu
          anchorEl={userMenuAnchor}
          open={Boolean(userMenuAnchor)}
          onClose={handleUserMenuClose}
          PaperProps={{
            sx: { width: 240 }
          }}
        >
          <Box sx={{ p: 2, borderBottom: '1px solid', borderColor: 'divider' }}>
            <Typography variant="subtitle1" fontWeight="600">
              {user?.username}
            </Typography>
            <Typography variant="body2" color="text.secondary">
              {user?.email}
            </Typography>
            <Chip
              label={user?.role?.toUpperCase() || 'USER'}
              size="small"
              color="primary"
              variant="outlined"
              sx={{ mt: 1 }}
            />
          </Box>

          <MenuItem onClick={handleUserMenuClose}>
            <ListItemIcon>
              <AccountCircle />
            </ListItemIcon>
            <ListItemText primary="Profile" />
          </MenuItem>

          <MenuItem onClick={handleUserMenuClose}>
            <ListItemIcon>
              <Settings />
            </ListItemIcon>
            <ListItemText primary="Settings" />
          </MenuItem>

          <Divider />

          <MenuItem onClick={handleLogout}>
            <ListItemIcon>
              <Logout />
            </ListItemIcon>
            <ListItemText primary="Logout" />
          </MenuItem>
        </Menu>
      </Toolbar>
    </AppBar>
  );
}