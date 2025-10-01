// AION-CR Sidebar Component
// Maximum Autonomy Level 255 - Complete System Integration

import React, { useState } from 'react';
import { useNavigate, useLocation } from 'react-router-dom';
import {
  Drawer,
  List,
  ListItem,
  ListItemIcon,
  ListItemText,
  ListItemButton,
  Box,
  Typography,
  Divider,
  Chip,
  Tooltip,
  Badge,
  Collapse,
  IconButton,
} from '@mui/material';
import {
  Dashboard as DashboardIcon,
  SmartToy as AgentsIcon,
  Security as ComplianceIcon,
  Warning as ConflictsIcon,
  Psychology as MLIcon,
  MonitorHeart as MonitoringIcon,
  CloudUpload as DeploymentIcon,
  Settings as SettingsIcon,
  ExpandLess,
  ExpandMore,
  ChevronLeft,
  ChevronRight,
  AccountTree,
  Analytics,
  BugReport,
  DataObject,
  Lock,
  Timeline,
  AutoFixHigh,
  Memory,
  Hub,
} from '@mui/icons-material';
import { SystemStatus } from '../../types';

interface SidebarProps {
  open: boolean;
  onToggle: () => void;
  systemStatus: SystemStatus | null;
}

interface MenuItem {
  id: string;
  label: string;
  path: string;
  icon: React.ReactNode;
  badge?: number;
  children?: MenuItem[];
}

const menuItems: MenuItem[] = [
  {
    id: 'dashboard',
    label: 'Dashboard',
    path: '/dashboard',
    icon: <DashboardIcon />,
  },
  {
    id: 'agents',
    label: 'Autonomous Agents',
    path: '/agents',
    icon: <AgentsIcon />,
    children: [
      { id: 'agents-overview', label: 'Overview', path: '/agents', icon: <AccountTree /> },
      { id: 'agents-create', label: 'Create Agent', path: '/agents/create', icon: <AutoFixHigh /> },
      { id: 'agents-training', label: 'Training', path: '/agents/training', icon: <Psychology /> },
    ],
  },
  {
    id: 'compliance',
    label: 'Compliance',
    path: '/compliance',
    icon: <ComplianceIcon />,
    children: [
      { id: 'compliance-rules', label: 'Rules Engine', path: '/compliance/rules', icon: <DataObject /> },
      { id: 'compliance-frameworks', label: 'Frameworks', path: '/compliance/frameworks', icon: <Security /> },
      { id: 'compliance-audit', label: 'Audit Trails', path: '/compliance/audit', icon: <Timeline /> },
    ],
  },
  {
    id: 'conflicts',
    label: 'Conflict Resolution',
    path: '/conflicts',
    icon: <ConflictsIcon />,
  },
  {
    id: 'ml',
    label: 'Machine Learning',
    path: '/ml',
    icon: <MLIcon />,
    children: [
      { id: 'ml-models', label: 'Models', path: '/ml/models', icon: <Memory /> },
      { id: 'ml-training', label: 'Training', path: '/ml/training', icon: <Psychology /> },
      { id: 'ml-quantum', label: 'Quantum ML', path: '/ml/quantum', icon: <Hub /> },
    ],
  },
  {
    id: 'monitoring',
    label: 'System Monitoring',
    path: '/monitoring',
    icon: <MonitoringIcon />,
    children: [
      { id: 'monitoring-alerts', label: 'Alerts', path: '/monitoring/alerts', icon: <Warning /> },
      { id: 'monitoring-logs', label: 'Logs', path: '/monitoring/logs', icon: <BugReport /> },
      { id: 'monitoring-analytics', label: 'Analytics', path: '/monitoring/analytics', icon: <Analytics /> },
    ],
  },
  {
    id: 'deployment',
    label: 'Deployment',
    path: '/deployment',
    icon: <DeploymentIcon />,
    children: [
      { id: 'deployment-blockchain', label: 'Blockchain', path: '/deployment/blockchain', icon: <Lock /> },
      { id: 'deployment-cloud', label: 'Cloud Services', path: '/deployment/cloud', icon: <CloudUpload /> },
    ],
  },
  {
    id: 'settings',
    label: 'Settings',
    path: '/settings',
    icon: <SettingsIcon />,
  },
];

export default function Sidebar({ open, onToggle, systemStatus }: SidebarProps) {
  const navigate = useNavigate();
  const location = useLocation();
  const [expandedItems, setExpandedItems] = useState<string[]>(['agents', 'compliance', 'ml', 'monitoring']);

  const handleItemClick = (item: MenuItem) => {
    if (item.children) {
      toggleExpanded(item.id);
    } else {
      navigate(item.path);
    }
  };

  const toggleExpanded = (itemId: string) => {
    setExpandedItems(prev =>
      prev.includes(itemId)
        ? prev.filter(id => id !== itemId)
        : [...prev, itemId]
    );
  };

  const isActive = (path: string) => {
    return location.pathname === path || location.pathname.startsWith(path + '/');
  };

  const getSystemStatusColor = () => {
    if (!systemStatus) return 'default';
    switch (systemStatus.status) {
      case 'online': return 'success';
      case 'degraded': return 'warning';
      case 'offline': return 'error';
      case 'maintenance': return 'info';
      default: return 'default';
    }
  };

  const getAutonomyLevelColor = (level: number) => {
    if (level >= 200) return 'error';
    if (level >= 150) return 'warning';
    if (level >= 100) return 'info';
    return 'success';
  };

  const renderMenuItem = (item: MenuItem, depth = 0) => {
    const hasChildren = item.children && item.children.length > 0;
    const isExpanded = expandedItems.includes(item.id);
    const active = isActive(item.path);

    return (
      <React.Fragment key={item.id}>
        <ListItem disablePadding sx={{ display: 'block' }}>
          <ListItemButton
            onClick={() => handleItemClick(item)}
            selected={active && !hasChildren}
            sx={{
              minHeight: 48,
              justifyContent: open ? 'initial' : 'center',
              px: 2.5,
              pl: depth > 0 ? 4 + depth * 2 : 2.5,
              backgroundColor: active && !hasChildren ? 'action.selected' : 'transparent',
              '&:hover': {
                backgroundColor: 'action.hover',
              },
            }}
          >
            <ListItemIcon
              sx={{
                minWidth: 0,
                mr: open ? 3 : 'auto',
                justifyContent: 'center',
                color: active ? 'primary.main' : 'inherit',
              }}
            >
              {item.badge ? (
                <Badge badgeContent={item.badge} color="error">
                  {item.icon}
                </Badge>
              ) : (
                item.icon
              )}
            </ListItemIcon>
            <ListItemText
              primary={item.label}
              sx={{
                opacity: open ? 1 : 0,
                color: active ? 'primary.main' : 'inherit',
                fontWeight: active ? 600 : 400,
              }}
            />
            {hasChildren && open && (
              <IconButton size="small" sx={{ ml: 1 }}>
                {isExpanded ? <ExpandLess /> : <ExpandMore />}
              </IconButton>
            )}
          </ListItemButton>
        </ListItem>

        {hasChildren && (
          <Collapse in={isExpanded && open} timeout="auto" unmountOnExit>
            <List component="div" disablePadding>
              {item.children!.map(child => renderMenuItem(child, depth + 1))}
            </List>
          </Collapse>
        )}
      </React.Fragment>
    );
  };

  return (
    <Drawer
      variant="permanent"
      sx={{
        width: open ? 280 : 80,
        flexShrink: 0,
        '& .MuiDrawer-paper': {
          width: open ? 280 : 80,
          boxSizing: 'border-box',
          borderRight: '1px solid',
          borderColor: 'divider',
          backgroundColor: 'background.paper',
          transition: 'width 0.3s ease',
          overflowX: 'hidden',
        },
      }}
    >
      {/* Header */}
      <Box
        sx={{
          display: 'flex',
          alignItems: 'center',
          justifyContent: 'space-between',
          p: 2,
          minHeight: 64,
          borderBottom: '1px solid',
          borderColor: 'divider',
        }}
      >
        {open && (
          <Box>
            <Typography variant="h6" fontWeight="bold" color="primary">
              AION-CR
            </Typography>
            <Typography variant="caption" color="text.secondary">
              Autonomous Compliance
            </Typography>
          </Box>
        )}
        <IconButton onClick={onToggle} size="small">
          {open ? <ChevronLeft /> : <ChevronRight />}
        </IconButton>
      </Box>

      {/* System Status */}
      {systemStatus && (
        <Box sx={{ p: open ? 2 : 1, borderBottom: '1px solid', borderColor: 'divider' }}>
          {open ? (
            <Box>
              <Box display="flex" alignItems="center" justifyContent="space-between" mb={1}>
                <Typography variant="subtitle2" fontWeight="600">
                  System Status
                </Typography>
                <Chip
                  label={systemStatus.status.toUpperCase()}
                  size="small"
                  color={getSystemStatusColor()}
                  variant="filled"
                />
              </Box>
              <Box display="flex" alignItems="center" justifyContent="space-between" mb={1}>
                <Typography variant="caption" color="text.secondary">
                  Autonomy Level
                </Typography>
                <Chip
                  label={`${systemStatus.autonomyLevel}/255`}
                  size="small"
                  color={getAutonomyLevelColor(systemStatus.autonomyLevel)}
                  variant="outlined"
                />
              </Box>
              <Box display="flex" alignItems="center" justifyContent="space-between">
                <Typography variant="caption" color="text.secondary">
                  Active AI Modules
                </Typography>
                <Typography variant="caption" fontWeight="600">
                  {systemStatus.aiModules?.filter(m => m.status === 'active').length || 0}
                </Typography>
              </Box>
            </Box>
          ) : (
            <Tooltip title={`System: ${systemStatus.status} | Autonomy: ${systemStatus.autonomyLevel}/255`}>
              <Box display="flex" justifyContent="center">
                <Chip
                  size="small"
                  color={getSystemStatusColor()}
                  variant="filled"
                  sx={{ width: 8, height: 8, borderRadius: '50%' }}
                />
              </Box>
            </Tooltip>
          )}
        </Box>
      )}

      {/* Navigation Menu */}
      <List sx={{ flexGrow: 1, py: 1 }}>
        {menuItems.map(item => renderMenuItem(item))}
      </List>

      {/* Footer */}
      <Box
        sx={{
          p: open ? 2 : 1,
          borderTop: '1px solid',
          borderColor: 'divider',
          textAlign: 'center',
        }}
      >
        {open ? (
          <Box>
            <Typography variant="caption" color="text.secondary">
              AION-CR v1.0.0
            </Typography>
            <br />
            <Typography variant="caption" color="text.secondary">
              Maximum Autonomy
            </Typography>
          </Box>
        ) : (
          <Tooltip title="AION-CR v1.0.0 - Maximum Autonomy">
            <Typography variant="caption" color="primary" fontWeight="bold">
              A
            </Typography>
          </Tooltip>
        )}
      </Box>
    </Drawer>
  );
}