// AION-CR Dashboard Page
// Maximum Autonomy Level 255 - Complete System Integration

import React, { useState, useEffect } from 'react';
import {
  Box,
  Grid,
  Card,
  CardContent,
  Typography,
  LinearProgress,
  Chip,
  Alert,
  CircularProgress,
  List,
  ListItem,
  ListItemText,
  ListItemIcon,
  IconButton,
  Tooltip,
} from '@mui/material';
import {
  TrendingUp,
  Security,
  SmartToy,
  Warning,
  CheckCircle,
  Speed,
  Memory,
  Storage,
  NetworkCheck,
  Psychology,
  AutoFixHigh,
  Refresh,
  Timeline,
  Analytics,
} from '@mui/icons-material';
import { SystemStatus } from '../types';
import { systemApi } from '../services/api';

interface DashboardProps {
  systemStatus: SystemStatus | null;
}

interface MetricCard {
  title: string;
  value: string | number;
  change?: number;
  icon: React.ReactNode;
  color: 'primary' | 'secondary' | 'success' | 'warning' | 'error' | 'info';
  description?: string;
}

export default function Dashboard({ systemStatus }: DashboardProps) {
  const [metrics, setMetrics] = useState<any>(null);
  const [recentActivity, setRecentActivity] = useState<any[]>([]);
  const [loading, setLoading] = useState(true);
  const [lastUpdate, setLastUpdate] = useState(new Date());

  const refreshData = async () => {
    try {
      setLoading(true);
      const [metricsResponse, activityResponse] = await Promise.all([
        systemApi.getSystemMetrics('24h'),
        systemApi.getAuditLogs({ limit: 10, sortBy: 'timestamp', sortOrder: 'desc' }),
      ]);

      if (metricsResponse.success) {
        setMetrics(metricsResponse.data);
      }

      if (activityResponse.success) {
        setRecentActivity(activityResponse.data?.items || []);
      }

      setLastUpdate(new Date());
    } catch (error) {
      console.error('Failed to refresh dashboard data:', error);
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    refreshData();

    // Auto-refresh every 30 seconds
    const interval = setInterval(refreshData, 30000);
    return () => clearInterval(interval);
  }, []);

  const metricCards: MetricCard[] = [
    {
      title: 'System Uptime',
      value: systemStatus ? formatUptime(systemStatus.uptime) : '---',
      icon: <Speed />,
      color: 'success',
      description: 'Continuous operation time',
    },
    {
      title: 'Autonomy Level',
      value: `${systemStatus?.autonomyLevel || 0}/255`,
      icon: <AutoFixHigh />,
      color: systemStatus?.autonomyLevel && systemStatus.autonomyLevel >= 200 ? 'error' : 'primary',
      description: 'Current AI autonomy level',
    },
    {
      title: 'Active AI Agents',
      value: systemStatus?.aiModules?.filter(m => m.status === 'active').length || 0,
      icon: <SmartToy />,
      color: 'info',
      description: 'Currently running AI modules',
    },
    {
      title: 'Compliance Score',
      value: `${metrics?.compliance?.score || 0}%`,
      change: metrics?.compliance?.change,
      icon: <Security />,
      color: 'primary',
      description: 'Overall compliance status',
    },
    {
      title: 'Active Conflicts',
      value: metrics?.conflicts?.active || 0,
      icon: <Warning />,
      color: 'warning',
      description: 'Unresolved regulatory conflicts',
    },
    {
      title: 'Processing Rate',
      value: `${metrics?.processing?.rate || 0}/min`,
      change: metrics?.processing?.change,
      icon: <TrendingUp />,
      color: 'success',
      description: 'Documents processed per minute',
    },
  ];

  const formatUptime = (uptime: number) => {
    const days = Math.floor(uptime / (24 * 60 * 60 * 1000));
    const hours = Math.floor((uptime % (24 * 60 * 60 * 1000)) / (60 * 60 * 1000));
    const minutes = Math.floor((uptime % (60 * 60 * 1000)) / (60 * 1000));

    if (days > 0) return `${days}d ${hours}h ${minutes}m`;
    if (hours > 0) return `${hours}h ${minutes}m`;
    return `${minutes}m`;
  };

  const getActivityIcon = (action: string) => {
    if (action.includes('compliance')) return <Security />;
    if (action.includes('agent')) return <SmartToy />;
    if (action.includes('conflict')) return <Warning />;
    if (action.includes('training')) return <Psychology />;
    return <Timeline />;
  };

  const getActivityColor = (outcome: string) => {
    switch (outcome) {
      case 'success': return 'success';
      case 'warning': return 'warning';
      case 'failure': return 'error';
      default: return 'info';
    }
  };

  return (
    <Box>
      {/* Header */}
      <Box display="flex" justifyContent="space-between" alignItems="center" mb={3}>
        <Box>
          <Typography variant="h4" fontWeight="bold" gutterBottom>
            AION-CR Dashboard
          </Typography>
          <Typography variant="body1" color="text.secondary">
            Autonomous Intelligence Operations Network - Compliance Regulation
          </Typography>
        </Box>

        <Box display="flex" alignItems="center" gap={2}>
          <Typography variant="caption" color="text.secondary">
            Last updated: {lastUpdate.toLocaleTimeString()}
          </Typography>
          <Tooltip title="Refresh data">
            <IconButton onClick={refreshData} disabled={loading}>
              <Refresh />
            </IconButton>
          </Tooltip>
        </Box>
      </Box>

      {/* System Status Alert */}
      {systemStatus && systemStatus.status !== 'online' && (
        <Alert
          severity={
            systemStatus.status === 'offline' ? 'error' :
            systemStatus.status === 'degraded' ? 'warning' : 'info'
          }
          sx={{ mb: 3 }}
        >
          System Status: {systemStatus.status.toUpperCase()} - {
            systemStatus.status === 'maintenance' ? 'Scheduled maintenance in progress' :
            systemStatus.status === 'degraded' ? 'Some services may be impacted' :
            'System is currently offline'
          }
        </Alert>
      )}

      {/* Metric Cards */}
      <Grid container spacing={3} mb={4}>
        {metricCards.map((metric, index) => (
          <Grid item xs={12} sm={6} md={4} lg={2} key={index}>
            <Card sx={{ height: '100%' }}>
              <CardContent>
                <Box display="flex" alignItems="center" justifyContent="space-between" mb={2}>
                  <Box sx={{ color: `${metric.color}.main` }}>
                    {metric.icon}
                  </Box>
                  {metric.change !== undefined && (
                    <Chip
                      label={`${metric.change > 0 ? '+' : ''}${metric.change}%`}
                      size="small"
                      color={metric.change > 0 ? 'success' : 'error'}
                      variant="outlined"
                    />
                  )}
                </Box>

                <Typography variant="h4" fontWeight="bold" gutterBottom>
                  {metric.value}
                </Typography>

                <Typography variant="subtitle2" color="text.secondary" gutterBottom>
                  {metric.title}
                </Typography>

                {metric.description && (
                  <Typography variant="caption" color="text.secondary">
                    {metric.description}
                  </Typography>
                )}
              </CardContent>
            </Card>
          </Grid>
        ))}
      </Grid>

      <Grid container spacing={3}>
        {/* System Health */}
        <Grid item xs={12} md={6}>
          <Card>
            <CardContent>
              <Box display="flex" alignItems="center" justifyContent="between" mb={2}>
                <Typography variant="h6" fontWeight="600">
                  System Health
                </Typography>
                <Memory color="primary" />
              </Box>

              {systemStatus?.health ? (
                <Box>
                  <Box mb={2}>
                    <Box display="flex" justifyContent="space-between" mb={1}>
                      <Typography variant="body2">CPU Usage</Typography>
                      <Typography variant="body2" fontWeight="600">
                        {systemStatus.health.cpu}%
                      </Typography>
                    </Box>
                    <LinearProgress
                      variant="determinate"
                      value={systemStatus.health.cpu}
                      color={systemStatus.health.cpu > 80 ? 'error' : systemStatus.health.cpu > 60 ? 'warning' : 'success'}
                    />
                  </Box>

                  <Box mb={2}>
                    <Box display="flex" justifyContent="space-between" mb={1}>
                      <Typography variant="body2">Memory Usage</Typography>
                      <Typography variant="body2" fontWeight="600">
                        {systemStatus.health.memory}%
                      </Typography>
                    </Box>
                    <LinearProgress
                      variant="determinate"
                      value={systemStatus.health.memory}
                      color={systemStatus.health.memory > 80 ? 'error' : systemStatus.health.memory > 60 ? 'warning' : 'success'}
                    />
                  </Box>

                  <Box mb={2}>
                    <Box display="flex" justifyContent="space-between" mb={1}>
                      <Typography variant="body2">Disk Usage</Typography>
                      <Typography variant="body2" fontWeight="600">
                        {systemStatus.health.disk}%
                      </Typography>
                    </Box>
                    <LinearProgress
                      variant="determinate"
                      value={systemStatus.health.disk}
                      color={systemStatus.health.disk > 80 ? 'error' : systemStatus.health.disk > 60 ? 'warning' : 'success'}
                    />
                  </Box>

                  <Box>
                    <Box display="flex" justifyContent="space-between" mb={1}>
                      <Typography variant="body2">Network I/O</Typography>
                      <Typography variant="body2" fontWeight="600">
                        {systemStatus.health.network}%
                      </Typography>
                    </Box>
                    <LinearProgress
                      variant="determinate"
                      value={systemStatus.health.network}
                      color={systemStatus.health.network > 80 ? 'error' : systemStatus.health.network > 60 ? 'warning' : 'success'}
                    />
                  </Box>
                </Box>
              ) : (
                <Box display="flex" alignItems="center" justifyContent="center" py={4}>
                  <CircularProgress size={40} />
                </Box>
              )}
            </CardContent>
          </Card>
        </Grid>

        {/* Services Status */}
        <Grid item xs={12} md={6}>
          <Card>
            <CardContent>
              <Box display="flex" alignItems="center" justify-content="space-between" mb={2}>
                <Typography variant="h6" fontWeight="600">
                  Services Status
                </Typography>
                <NetworkCheck color="primary" />
              </Box>

              {systemStatus?.services ? (
                <List dense>
                  {systemStatus.services.slice(0, 6).map((service, index) => (
                    <ListItem key={index} sx={{ px: 0 }}>
                      <ListItemIcon sx={{ minWidth: 32 }}>
                        {service.status === 'running' ? (
                          <CheckCircle color="success" fontSize="small" />
                        ) : (
                          <Warning color="error" fontSize="small" />
                        )}
                      </ListItemIcon>
                      <ListItemText
                        primary={service.name}
                        secondary={`${service.status} ${service.responseTime ? `(${service.responseTime}ms)` : ''}`}
                        primaryTypographyProps={{ fontSize: '0.875rem' }}
                        secondaryTypographyProps={{ fontSize: '0.75rem' }}
                      />
                    </ListItem>
                  ))}
                </List>
              ) : (
                <Box display="flex" alignItems="center" justifyContent="center" py={4}>
                  <CircularProgress size={40} />
                </Box>
              )}
            </CardContent>
          </Card>
        </Grid>

        {/* Recent Activity */}
        <Grid item xs={12}>
          <Card>
            <CardContent>
              <Box display="flex" alignItems="center" justifyContent="space-between" mb={2}>
                <Typography variant="h6" fontWeight="600">
                  Recent Activity
                </Typography>
                <Analytics color="primary" />
              </Box>

              {recentActivity.length > 0 ? (
                <List>
                  {recentActivity.slice(0, 8).map((activity, index) => (
                    <ListItem key={activity.id || index} divider={index < recentActivity.length - 1}>
                      <ListItemIcon>
                        {getActivityIcon(activity.action)}
                      </ListItemIcon>
                      <ListItemText
                        primary={activity.action}
                        secondary={
                          <Box display="flex" alignItems="center" gap={1}>
                            <Typography variant="caption" color="text.secondary">
                              {new Date(activity.timestamp).toLocaleString()}
                            </Typography>
                            <Chip
                              label={activity.outcome}
                              size="small"
                              color={getActivityColor(activity.outcome)}
                              variant="outlined"
                            />
                            {activity.user && (
                              <Typography variant="caption" color="text.secondary">
                                by {activity.user}
                              </Typography>
                            )}
                          </Box>
                        }
                      />
                    </ListItem>
                  ))}
                </List>
              ) : (
                <Box display="flex" alignItems="center" justifyContent="center" py={4}>
                  {loading ? (
                    <CircularProgress size={40} />
                  ) : (
                    <Typography color="text.secondary">
                      No recent activity
                    </Typography>
                  )}
                </Box>
              )}
            </CardContent>
          </Card>
        </Grid>
      </Grid>
    </Box>
  );
}