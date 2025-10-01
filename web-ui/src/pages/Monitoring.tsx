// AION-CR System Monitoring Page
// Maximum Autonomy Level 255 - Complete System Integration

import React, { useState, useEffect } from 'react';
import {
  Box,
  Grid,
  Card,
  CardContent,
  Typography,
  Button,
  Chip,
  Alert,
  CircularProgress,
  LinearProgress,
  Tabs,
  Tab,
  List,
  ListItem,
  ListItemIcon,
  ListItemText,
  IconButton,
  Tooltip,
  Dialog,
  DialogTitle,
  DialogContent,
  DialogActions,
  TextField,
} from '@mui/material';
import {
  MonitorHeart,
  Warning,
  Error,
  CheckCircle,
  Info,
  Speed,
  Memory,
  Storage,
  NetworkCheck,
  Refresh,
  NotificationsActive,
  BugReport,
  Analytics,
  Timeline,
  Visibility,
  Clear,
  Notifications,
} from '@mui/icons-material';
import { SystemStatus, SystemAlert } from '../types';
import { systemApi } from '../services/api';

interface TabPanelProps {
  children?: React.ReactNode;
  index: number;
  value: number;
}

function TabPanel(props: TabPanelProps) {
  const { children, value, index, ...other } = props;

  return (
    <div
      role="tabpanel"
      hidden={value !== index}
      id={`monitoring-tabpanel-${index}`}
      aria-labelledby={`monitoring-tab-${index}`}
      {...other}
    >
      {value === index && <Box sx={{ p: 3 }}>{children}</Box>}
    </div>
  );
}

export default function Monitoring() {
  const [activeTab, setActiveTab] = useState(0);
  const [systemStatus, setSystemStatus] = useState<SystemStatus | null>(null);
  const [alerts, setAlerts] = useState<SystemAlert[]>([]);
  const [logs, setLogs] = useState<any[]>([]);
  const [loading, setLoading] = useState(true);
  const [selectedAlert, setSelectedAlert] = useState<SystemAlert | null>(null);
  const [alertDialogOpen, setAlertDialogOpen] = useState(false);
  const [resolveDialogOpen, setResolveDialogOpen] = useState(false);
  const [resolution, setResolution] = useState('');

  const loadMonitoringData = async () => {
    try {
      setLoading(true);
      const [statusResponse, alertsResponse, logsResponse] = await Promise.all([
        systemApi.getSystemStatus(),
        systemApi.getAlerts({ limit: 50, sortBy: 'timestamp', sortOrder: 'desc' }),
        systemApi.getAuditLogs({ limit: 100, sortBy: 'timestamp', sortOrder: 'desc' }),
      ]);

      if (statusResponse) {
        setSystemStatus(statusResponse);
      }

      if (alertsResponse.success) {
        setAlerts(alertsResponse.data?.items || []);
      }

      if (logsResponse.success) {
        setLogs(logsResponse.data?.items || []);
      }
    } catch (error) {
      console.error('Failed to load monitoring data:', error);
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    loadMonitoringData();

    // Auto-refresh every 30 seconds
    const interval = setInterval(loadMonitoringData, 30000);
    return () => clearInterval(interval);
  }, []);

  const handleAcknowledgeAlert = async (alertId: string) => {
    try {
      await systemApi.acknowledgeAlert(alertId);
      loadMonitoringData();
    } catch (error) {
      console.error('Failed to acknowledge alert:', error);
    }
  };

  const handleResolveAlert = async () => {
    if (!selectedAlert || !resolution.trim()) return;

    try {
      await systemApi.resolveAlert(selectedAlert.id, resolution);
      setResolveDialogOpen(false);
      setResolution('');
      setSelectedAlert(null);
      loadMonitoringData();
    } catch (error) {
      console.error('Failed to resolve alert:', error);
    }
  };

  const getSeverityIcon = (severity: string) => {
    switch (severity) {
      case 'critical': return <Error color="error" />;
      case 'high': return <Warning color="warning" />;
      case 'medium': return <Info color="info" />;
      case 'low': return <CheckCircle color="success" />;
      default: return <Info />;
    }
  };

  const getSeverityColor = (severity: string) => {
    switch (severity) {
      case 'critical': return 'error';
      case 'high': return 'warning';
      case 'medium': return 'info';
      case 'low': return 'success';
      default: return 'default';
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

  const getHealthColor = (value: number) => {
    if (value >= 90) return 'error';
    if (value >= 70) return 'warning';
    return 'success';
  };

  const formatUptime = (uptime: number) => {
    const days = Math.floor(uptime / (24 * 60 * 60 * 1000));
    const hours = Math.floor((uptime % (24 * 60 * 60 * 1000)) / (60 * 60 * 1000));
    const minutes = Math.floor((uptime % (60 * 60 * 1000)) / (60 * 1000));

    if (days > 0) return `${days}d ${hours}h ${minutes}m`;
    if (hours > 0) return `${hours}h ${minutes}m`;
    return `${minutes}m`;
  };

  const criticalAlerts = alerts.filter(a => a.severity === 'critical' && !a.resolved).length;
  const unresolvedAlerts = alerts.filter(a => !a.resolved).length;

  return (
    <Box>
      {/* Header */}
      <Box display="flex" justifyContent="space-between" alignItems="center" mb={3}>
        <Box>
          <Typography variant="h4" fontWeight="bold" gutterBottom>
            System Monitoring
          </Typography>
          <Typography variant="body1" color="text.secondary">
            Real-time system health and performance monitoring
          </Typography>
        </Box>

        <Box display="flex" gap={2}>
          <Button
            variant="outlined"
            startIcon={<Refresh />}
            onClick={loadMonitoringData}
            disabled={loading}
          >
            Refresh
          </Button>
          <Button
            variant="contained"
            startIcon={<NotificationsActive />}
            color={criticalAlerts > 0 ? 'error' : 'primary'}
          >
            {unresolvedAlerts} Alerts
          </Button>
        </Box>
      </Box>

      {/* Critical Alerts Banner */}
      {criticalAlerts > 0 && (
        <Alert severity="error" sx={{ mb: 3 }}>
          <Typography variant="h6" fontWeight="600">
            {criticalAlerts} Critical Alert{criticalAlerts !== 1 ? 's' : ''} Require Immediate Attention
          </Typography>
          <Typography variant="body2">
            System stability may be compromised. Please review and resolve critical alerts immediately.
          </Typography>
        </Alert>
      )}

      {/* System Overview Cards */}
      <Grid container spacing={3} mb={4}>
        <Grid item xs={12} sm={6} md={3}>
          <Card>
            <CardContent>
              <Box display="flex" alignItems="center" justifyContent="space-between">
                <Box>
                  <Typography variant="h6" fontWeight="bold" color={getStatusColor(systemStatus?.status || 'offline')}>
                    {systemStatus?.status?.toUpperCase() || 'OFFLINE'}
                  </Typography>
                  <Typography variant="subtitle2" color="text.secondary">
                    System Status
                  </Typography>
                </Box>
                <MonitorHeart color={getStatusColor(systemStatus?.status || 'offline')} />
              </Box>
              {systemStatus && (
                <Typography variant="caption" color="text.secondary">
                  Uptime: {formatUptime(systemStatus.uptime)}
                </Typography>
              )}
            </CardContent>
          </Card>
        </Grid>

        <Grid item xs={12} sm={6} md={3}>
          <Card>
            <CardContent>
              <Box display="flex" alignItems="center" justifyContent="space-between">
                <Box>
                  <Typography variant="h4" fontWeight="bold" color="warning">
                    {unresolvedAlerts}
                  </Typography>
                  <Typography variant="subtitle2" color="text.secondary">
                    Active Alerts
                  </Typography>
                </Box>
                <Warning color="warning" />
              </Box>
              <Typography variant="caption" color="text.secondary">
                {criticalAlerts} critical
              </Typography>
            </CardContent>
          </Card>
        </Grid>

        <Grid item xs={12} sm={6} md={3}>
          <Card>
            <CardContent>
              <Box display="flex" alignItems="center" justifyContent="space-between">
                <Box>
                  <Typography variant="h4" fontWeight="bold" color="info">
                    {systemStatus?.services?.filter(s => s.status === 'running').length || 0}
                  </Typography>
                  <Typography variant="subtitle2" color="text.secondary">
                    Services Running
                  </Typography>
                </Box>
                <NetworkCheck color="info" />
              </Box>
              <Typography variant="caption" color="text.secondary">
                of {systemStatus?.services?.length || 0} total
              </Typography>
            </CardContent>
          </Card>
        </Grid>

        <Grid item xs={12} sm={6} md={3}>
          <Card>
            <CardContent>
              <Box display="flex" alignItems="center" justifyContent="space-between">
                <Box>
                  <Typography variant="h4" fontWeight="bold" color="primary">
                    {systemStatus?.autonomyLevel || 0}
                  </Typography>
                  <Typography variant="subtitle2" color="text.secondary">
                    Autonomy Level
                  </Typography>
                </Box>
                <Speed color="primary" />
              </Box>
              <Typography variant="caption" color="text.secondary">
                Maximum: 255
              </Typography>
            </CardContent>
          </Card>
        </Grid>
      </Grid>

      {/* System Health */}
      {systemStatus?.health && (
        <Card sx={{ mb: 3 }}>
          <CardContent>
            <Typography variant="h6" fontWeight="600" gutterBottom>
              System Health
            </Typography>

            <Grid container spacing={3}>
              <Grid item xs={12} sm={6} md={3}>
                <Box>
                  <Box display="flex" alignItems="center" gap={1} mb={1}>
                    <Memory />
                    <Typography variant="subtitle2">CPU Usage</Typography>
                  </Box>
                  <LinearProgress
                    variant="determinate"
                    value={systemStatus.health.cpu}
                    color={getHealthColor(systemStatus.health.cpu)}
                    sx={{ height: 8, borderRadius: 1 }}
                  />
                  <Typography variant="caption" color="text.secondary">
                    {systemStatus.health.cpu}%
                  </Typography>
                </Box>
              </Grid>

              <Grid item xs={12} sm={6} md={3}>
                <Box>
                  <Box display="flex" alignItems="center" gap={1} mb={1}>
                    <Memory />
                    <Typography variant="subtitle2">Memory Usage</Typography>
                  </Box>
                  <LinearProgress
                    variant="determinate"
                    value={systemStatus.health.memory}
                    color={getHealthColor(systemStatus.health.memory)}
                    sx={{ height: 8, borderRadius: 1 }}
                  />
                  <Typography variant="caption" color="text.secondary">
                    {systemStatus.health.memory}%
                  </Typography>
                </Box>
              </Grid>

              <Grid item xs={12} sm={6} md={3}>
                <Box>
                  <Box display="flex" alignItems="center" gap={1} mb={1}>
                    <Storage />
                    <Typography variant="subtitle2">Disk Usage</Typography>
                  </Box>
                  <LinearProgress
                    variant="determinate"
                    value={systemStatus.health.disk}
                    color={getHealthColor(systemStatus.health.disk)}
                    sx={{ height: 8, borderRadius: 1 }}
                  />
                  <Typography variant="caption" color="text.secondary">
                    {systemStatus.health.disk}%
                  </Typography>
                </Box>
              </Grid>

              <Grid item xs={12} sm={6} md={3}>
                <Box>
                  <Box display="flex" alignItems="center" gap={1} mb={1}>
                    <NetworkCheck />
                    <Typography variant="subtitle2">Network I/O</Typography>
                  </Box>
                  <LinearProgress
                    variant="determinate"
                    value={systemStatus.health.network}
                    color={getHealthColor(systemStatus.health.network)}
                    sx={{ height: 8, borderRadius: 1 }}
                  />
                  <Typography variant="caption" color="text.secondary">
                    {systemStatus.health.network}%
                  </Typography>
                </Box>
              </Grid>
            </Grid>
          </CardContent>
        </Card>
      )}

      {/* Main Content Tabs */}
      <Card>
        <Box sx={{ borderBottom: 1, borderColor: 'divider' }}>
          <Tabs value={activeTab} onChange={(_, newValue) => setActiveTab(newValue)}>
            <Tab label="Alerts" />
            <Tab label="System Logs" />
            <Tab label="Performance" />
            <Tab label="Services" />
          </Tabs>
        </Box>

        {/* Alerts Tab */}
        <TabPanel value={activeTab} index={0}>
          {loading ? (
            <Box display="flex" justifyContent="center" py={4}>
              <CircularProgress />
            </Box>
          ) : (
            <List>
              {alerts.map((alert) => (
                <ListItem
                  key={alert.id}
                  sx={{
                    border: 1,
                    borderColor: 'divider',
                    borderRadius: 1,
                    mb: 1,
                    backgroundColor: alert.resolved ? 'action.hover' : 'background.paper',
                  }}
                >
                  <ListItemIcon>
                    {getSeverityIcon(alert.severity)}
                  </ListItemIcon>

                  <ListItemText
                    primary={
                      <Box display="flex" alignItems="center" gap={1}>
                        <Typography variant="subtitle2" fontWeight="600">
                          {alert.title}
                        </Typography>
                        <Chip
                          label={alert.severity.toUpperCase()}
                          size="small"
                          color={getSeverityColor(alert.severity)}
                        />
                        {alert.resolved && (
                          <Chip label="RESOLVED" size="small" color="success" variant="outlined" />
                        )}
                      </Box>
                    }
                    secondary={
                      <Box>
                        <Typography variant="body2" color="text.secondary">
                          {alert.message}
                        </Typography>
                        <Typography variant="caption" color="text.secondary">
                          {new Date(alert.timestamp).toLocaleString()} • Source: {alert.source}
                          {alert.assignedTo && ` • Assigned to: ${alert.assignedTo}`}
                        </Typography>
                      </Box>
                    }
                  />

                  <Box display="flex" gap={1}>
                    <Tooltip title="View Details">
                      <IconButton
                        size="small"
                        onClick={() => {
                          setSelectedAlert(alert);
                          setAlertDialogOpen(true);
                        }}
                      >
                        <Visibility />
                      </IconButton>
                    </Tooltip>

                    {!alert.resolved && (
                      <>
                        <Tooltip title="Acknowledge">
                          <IconButton
                            size="small"
                            color="info"
                            onClick={() => handleAcknowledgeAlert(alert.id)}
                          >
                            <Notifications />
                          </IconButton>
                        </Tooltip>

                        <Tooltip title="Resolve">
                          <IconButton
                            size="small"
                            color="success"
                            onClick={() => {
                              setSelectedAlert(alert);
                              setResolveDialogOpen(true);
                            }}
                          >
                            <Clear />
                          </IconButton>
                        </Tooltip>
                      </>
                    )}
                  </Box>
                </ListItem>
              ))}

              {alerts.length === 0 && (
                <Box textAlign="center" py={4}>
                  <CheckCircle color="success" sx={{ fontSize: 48, mb: 2 }} />
                  <Typography variant="h6" color="text.secondary">
                    No alerts found
                  </Typography>
                  <Typography variant="body2" color="text.secondary">
                    System is operating normally
                  </Typography>
                </Box>
              )}
            </List>
          )}
        </TabPanel>

        {/* System Logs Tab */}
        <TabPanel value={activeTab} index={1}>
          <List>
            {logs.slice(0, 20).map((log, index) => (
              <ListItem key={index} divider>
                <ListItemIcon>
                  <BugReport />
                </ListItemIcon>
                <ListItemText
                  primary={log.action}
                  secondary={
                    <Box>
                      <Typography variant="caption" color="text.secondary">
                        {new Date(log.timestamp).toLocaleString()} • {log.user} • {log.outcome}
                      </Typography>
                    </Box>
                  }
                />
              </ListItem>
            ))}
          </List>
        </TabPanel>

        {/* Performance Tab */}
        <TabPanel value={activeTab} index={2}>
          <Alert severity="info">
            Performance analytics dashboard will show historical metrics, trends, and capacity planning.
          </Alert>
        </TabPanel>

        {/* Services Tab */}
        <TabPanel value={activeTab} index={3}>
          <List>
            {systemStatus?.services?.map((service, index) => (
              <ListItem key={index} divider>
                <ListItemIcon>
                  {service.status === 'running' ? (
                    <CheckCircle color="success" />
                  ) : (
                    <Error color="error" />
                  )}
                </ListItemIcon>
                <ListItemText
                  primary={service.name}
                  secondary={
                    <Box>
                      <Typography variant="caption" color="text.secondary">
                        Status: {service.status}
                        {service.port && ` • Port: ${service.port}`}
                        {service.responseTime && ` • Response: ${service.responseTime}ms`}
                      </Typography>
                    </Box>
                  }
                />
              </ListItem>
            ))}
          </List>
        </TabPanel>
      </Card>

      {/* Alert Detail Dialog */}
      <Dialog
        open={alertDialogOpen}
        onClose={() => setAlertDialogOpen(false)}
        maxWidth="md"
        fullWidth
      >
        <DialogTitle>
          Alert Details: {selectedAlert?.title}
        </DialogTitle>
        <DialogContent>
          {selectedAlert && (
            <Box sx={{ pt: 2 }}>
              <Grid container spacing={2}>
                <Grid item xs={6}>
                  <Typography variant="subtitle2">Severity</Typography>
                  <Chip
                    label={selectedAlert.severity.toUpperCase()}
                    color={getSeverityColor(selectedAlert.severity)}
                  />
                </Grid>
                <Grid item xs={6}>
                  <Typography variant="subtitle2">Source</Typography>
                  <Typography variant="body2">{selectedAlert.source}</Typography>
                </Grid>
                <Grid item xs={12}>
                  <Typography variant="subtitle2">Message</Typography>
                  <Typography variant="body2">{selectedAlert.message}</Typography>
                </Grid>
                <Grid item xs={6}>
                  <Typography variant="subtitle2">Created</Typography>
                  <Typography variant="body2">
                    {new Date(selectedAlert.timestamp).toLocaleString()}
                  </Typography>
                </Grid>
                <Grid item xs={6}>
                  <Typography variant="subtitle2">Status</Typography>
                  <Typography variant="body2">
                    {selectedAlert.resolved ? 'Resolved' : 'Active'}
                  </Typography>
                </Grid>
              </Grid>
            </Box>
          )}
        </DialogContent>
        <DialogActions>
          <Button onClick={() => setAlertDialogOpen(false)}>
            Close
          </Button>
        </DialogActions>
      </Dialog>

      {/* Resolve Alert Dialog */}
      <Dialog
        open={resolveDialogOpen}
        onClose={() => setResolveDialogOpen(false)}
        maxWidth="sm"
        fullWidth
      >
        <DialogTitle>
          Resolve Alert: {selectedAlert?.title}
        </DialogTitle>
        <DialogContent>
          <Box sx={{ pt: 2 }}>
            <TextField
              fullWidth
              label="Resolution Details"
              multiline
              rows={4}
              value={resolution}
              onChange={(e) => setResolution(e.target.value)}
              placeholder="Describe how this alert was resolved..."
            />
          </Box>
        </DialogContent>
        <DialogActions>
          <Button onClick={() => setResolveDialogOpen(false)}>
            Cancel
          </Button>
          <Button
            onClick={handleResolveAlert}
            variant="contained"
            color="success"
            disabled={!resolution.trim()}
          >
            Resolve Alert
          </Button>
        </DialogActions>
      </Dialog>
    </Box>
  );
}