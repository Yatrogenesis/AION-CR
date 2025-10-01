// AION-CR Settings Page
// Maximum Autonomy Level 255 - Complete System Integration

import React, { useState, useEffect } from 'react';
import {
  Box,
  Grid,
  Card,
  CardContent,
  Typography,
  Button,
  TextField,
  Switch,
  FormControlLabel,
  FormControl,
  InputLabel,
  Select,
  MenuItem,
  Tabs,
  Tab,
  Alert,
  Divider,
  List,
  ListItem,
  ListItemIcon,
  ListItemText,
  ListItemSecondaryAction,
  IconButton,
  Dialog,
  DialogTitle,
  DialogContent,
  DialogActions,
  Chip,
  Accordion,
  AccordionSummary,
  AccordionDetails,
  LinearProgress,
} from '@mui/material';
import {
  Settings as SettingsIcon,
  Security,
  Notifications,
  Palette,
  Language,
  Storage,
  NetworkCheck,
  Person,
  Shield,
  Key,
  Backup,
  Update,
  ExpandMore,
  Edit,
  Delete,
  Add,
  Save,
  Restore,
  Download,
  Upload,
  Warning,
  CheckCircle,
} from '@mui/icons-material';
import { useAuth } from '../hooks/useAuth';
import { useTheme } from '../hooks/useTheme';
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
      id={`settings-tabpanel-${index}`}
      aria-labelledby={`settings-tab-${index}`}
      {...other}
    >
      {value === index && <Box sx={{ p: 3 }}>{children}</Box>}
    </div>
  );
}

export default function Settings() {
  const { user, updateUser } = useAuth();
  const { theme, toggleTheme } = useTheme();
  const [activeTab, setActiveTab] = useState(0);
  const [loading, setLoading] = useState(false);
  const [saveSuccess, setSaveSuccess] = useState(false);
  const [backupDialogOpen, setBackupDialogOpen] = useState(false);
  const [apiKeyDialogOpen, setApiKeyDialogOpen] = useState(false);

  const [userSettings, setUserSettings] = useState({
    username: user?.username || '',
    email: user?.email || '',
    language: 'en',
    timezone: 'UTC',
    notifications: true,
    emailAlerts: true,
    darkMode: theme === 'dark',
    autoSave: true,
    maxAutonomy: 255,
  });

  const [systemSettings, setSystemSettings] = useState({
    systemName: 'AION-CR',
    maxConcurrentJobs: 100,
    logRetentionDays: 90,
    backupFrequency: 'daily',
    maintenanceMode: false,
    debugMode: false,
    auditingEnabled: true,
    encryptionEnabled: true,
    dataRetentionPolicy: '7years',
    complianceFrameworks: ['SOX', 'GDPR', 'HIPAA'],
  });

  const [apiKeys, setApiKeys] = useState([
    { id: '1', name: 'Production API', key: 'aion_****_prod', created: '2024-01-15', lastUsed: '2024-01-20' },
    { id: '2', name: 'Development API', key: 'aion_****_dev', created: '2024-01-10', lastUsed: '2024-01-19' },
    { id: '3', name: 'Analytics API', key: 'aion_****_analytics', created: '2024-01-05', lastUsed: '2024-01-18' },
  ]);

  const [newApiKey, setNewApiKey] = useState({
    name: '',
    permissions: 'read',
    expiryDays: 365,
  });

  useEffect(() => {
    if (user) {
      setUserSettings(prev => ({
        ...prev,
        username: user.username,
        email: user.email,
        darkMode: theme === 'dark',
      }));
    }
  }, [user, theme]);

  const handleSaveUserSettings = async () => {
    try {
      setLoading(true);
      // Update user preferences
      if (user) {
        const updatedUser = {
          ...user,
          username: userSettings.username,
          email: userSettings.email,
          preferences: {
            ...user.preferences,
            language: userSettings.language,
            notifications: userSettings.notifications,
            theme: userSettings.darkMode ? 'dark' : 'light',
          },
        };
        updateUser(updatedUser);
      }

      // Toggle theme if changed
      if ((theme === 'dark') !== userSettings.darkMode) {
        toggleTheme();
      }

      setSaveSuccess(true);
      setTimeout(() => setSaveSuccess(false), 3000);
    } catch (error) {
      console.error('Failed to save user settings:', error);
    } finally {
      setLoading(false);
    }
  };

  const handleSaveSystemSettings = async () => {
    try {
      setLoading(true);
      await systemApi.updateConfiguration(systemSettings);
      setSaveSuccess(true);
      setTimeout(() => setSaveSuccess(false), 3000);
    } catch (error) {
      console.error('Failed to save system settings:', error);
    } finally {
      setLoading(false);
    }
  };

  const handleCreateApiKey = async () => {
    try {
      const newKey = {
        id: Date.now().toString(),
        name: newApiKey.name,
        key: `aion_${Math.random().toString(36).substring(2, 15)}_${newApiKey.permissions}`,
        created: new Date().toISOString().split('T')[0],
        lastUsed: 'Never',
        permissions: newApiKey.permissions,
        expiryDays: newApiKey.expiryDays,
      };

      setApiKeys(prev => [...prev, newKey]);
      setApiKeyDialogOpen(false);
      setNewApiKey({ name: '', permissions: 'read', expiryDays: 365 });
    } catch (error) {
      console.error('Failed to create API key:', error);
    }
  };

  const handleDeleteApiKey = (keyId: string) => {
    setApiKeys(prev => prev.filter(key => key.id !== keyId));
  };

  const handleBackup = async () => {
    try {
      setLoading(true);
      // Simulate backup process
      await new Promise(resolve => setTimeout(resolve, 2000));
      setBackupDialogOpen(false);
      setSaveSuccess(true);
      setTimeout(() => setSaveSuccess(false), 3000);
    } catch (error) {
      console.error('Failed to create backup:', error);
    } finally {
      setLoading(false);
    }
  };

  return (
    <Box>
      {/* Header */}
      <Box display="flex" justifyContent="space-between" alignItems="center" mb={3}>
        <Box>
          <Typography variant="h4" fontWeight="bold" gutterBottom>
            System Settings
          </Typography>
          <Typography variant="body1" color="text.secondary">
            Configure AION-CR system preferences and security settings
          </Typography>
        </Box>

        {saveSuccess && (
          <Alert severity="success" sx={{ ml: 2 }}>
            Settings saved successfully!
          </Alert>
        )}
      </Box>

      {/* Main Content Tabs */}
      <Card>
        <Box sx={{ borderBottom: 1, borderColor: 'divider' }}>
          <Tabs value={activeTab} onChange={(_, newValue) => setActiveTab(newValue)}>
            <Tab icon={<Person />} label="Profile" />
            <Tab icon={<SettingsIcon />} label="System" />
            <Tab icon={<Security />} label="Security" />
            <Tab icon={<Key />} label="API Keys" />
            <Tab icon={<Backup />} label="Backup" />
          </Tabs>
        </Box>

        {/* Profile Tab */}
        <TabPanel value={activeTab} index={0}>
          <Grid container spacing={4}>
            <Grid item xs={12} md={6}>
              <Typography variant="h6" fontWeight="600" gutterBottom>
                Personal Information
              </Typography>

              <Box component="form" sx={{ mt: 2 }}>
                <TextField
                  fullWidth
                  label="Username"
                  value={userSettings.username}
                  onChange={(e) => setUserSettings(prev => ({ ...prev, username: e.target.value }))}
                  sx={{ mb: 3 }}
                />

                <TextField
                  fullWidth
                  label="Email"
                  type="email"
                  value={userSettings.email}
                  onChange={(e) => setUserSettings(prev => ({ ...prev, email: e.target.value }))}
                  sx={{ mb: 3 }}
                />

                <FormControl fullWidth sx={{ mb: 3 }}>
                  <InputLabel>Language</InputLabel>
                  <Select
                    value={userSettings.language}
                    onChange={(e) => setUserSettings(prev => ({ ...prev, language: e.target.value }))}
                  >
                    <MenuItem value="en">English</MenuItem>
                    <MenuItem value="es">Español</MenuItem>
                    <MenuItem value="fr">Français</MenuItem>
                    <MenuItem value="de">Deutsch</MenuItem>
                    <MenuItem value="zh">中文</MenuItem>
                  </Select>
                </FormControl>

                <FormControl fullWidth sx={{ mb: 3 }}>
                  <InputLabel>Timezone</InputLabel>
                  <Select
                    value={userSettings.timezone}
                    onChange={(e) => setUserSettings(prev => ({ ...prev, timezone: e.target.value }))}
                  >
                    <MenuItem value="UTC">UTC</MenuItem>
                    <MenuItem value="America/New_York">Eastern Time</MenuItem>
                    <MenuItem value="America/Los_Angeles">Pacific Time</MenuItem>
                    <MenuItem value="Europe/London">London</MenuItem>
                    <MenuItem value="Asia/Tokyo">Tokyo</MenuItem>
                  </Select>
                </FormControl>

                <Button
                  variant="contained"
                  startIcon={<Save />}
                  onClick={handleSaveUserSettings}
                  disabled={loading}
                >
                  Save Profile
                </Button>
              </Box>
            </Grid>

            <Grid item xs={12} md={6}>
              <Typography variant="h6" fontWeight="600" gutterBottom>
                Preferences
              </Typography>

              <List>
                <ListItem>
                  <ListItemIcon>
                    <Palette />
                  </ListItemIcon>
                  <ListItemText
                    primary="Dark Mode"
                    secondary="Enable dark theme interface"
                  />
                  <ListItemSecondaryAction>
                    <Switch
                      checked={userSettings.darkMode}
                      onChange={(e) => setUserSettings(prev => ({ ...prev, darkMode: e.target.checked }))}
                    />
                  </ListItemSecondaryAction>
                </ListItem>

                <ListItem>
                  <ListItemIcon>
                    <Notifications />
                  </ListItemIcon>
                  <ListItemText
                    primary="Notifications"
                    secondary="Receive system notifications"
                  />
                  <ListItemSecondaryAction>
                    <Switch
                      checked={userSettings.notifications}
                      onChange={(e) => setUserSettings(prev => ({ ...prev, notifications: e.target.checked }))}
                    />
                  </ListItemSecondaryAction>
                </ListItem>

                <ListItem>
                  <ListItemIcon>
                    <Notifications />
                  </ListItemIcon>
                  <ListItemText
                    primary="Email Alerts"
                    secondary="Receive critical alerts via email"
                  />
                  <ListItemSecondaryAction>
                    <Switch
                      checked={userSettings.emailAlerts}
                      onChange={(e) => setUserSettings(prev => ({ ...prev, emailAlerts: e.target.checked }))}
                    />
                  </ListItemSecondaryAction>
                </ListItem>

                <ListItem>
                  <ListItemIcon>
                    <Save />
                  </ListItemIcon>
                  <ListItemText
                    primary="Auto-save"
                    secondary="Automatically save changes"
                  />
                  <ListItemSecondaryAction>
                    <Switch
                      checked={userSettings.autoSave}
                      onChange={(e) => setUserSettings(prev => ({ ...prev, autoSave: e.target.checked }))}
                    />
                  </ListItemSecondaryAction>
                </ListItem>
              </List>
            </Grid>
          </Grid>
        </TabPanel>

        {/* System Tab */}
        <TabPanel value={activeTab} index={1}>
          <Grid container spacing={4}>
            <Grid item xs={12} md={6}>
              <Typography variant="h6" fontWeight="600" gutterBottom>
                System Configuration
              </Typography>

              <TextField
                fullWidth
                label="System Name"
                value={systemSettings.systemName}
                onChange={(e) => setSystemSettings(prev => ({ ...prev, systemName: e.target.value }))}
                sx={{ mb: 3 }}
              />

              <TextField
                fullWidth
                label="Max Concurrent Jobs"
                type="number"
                value={systemSettings.maxConcurrentJobs}
                onChange={(e) => setSystemSettings(prev => ({ ...prev, maxConcurrentJobs: parseInt(e.target.value) }))}
                sx={{ mb: 3 }}
              />

              <TextField
                fullWidth
                label="Log Retention (Days)"
                type="number"
                value={systemSettings.logRetentionDays}
                onChange={(e) => setSystemSettings(prev => ({ ...prev, logRetentionDays: parseInt(e.target.value) }))}
                sx={{ mb: 3 }}
              />

              <FormControl fullWidth sx={{ mb: 3 }}>
                <InputLabel>Backup Frequency</InputLabel>
                <Select
                  value={systemSettings.backupFrequency}
                  onChange={(e) => setSystemSettings(prev => ({ ...prev, backupFrequency: e.target.value }))}
                >
                  <MenuItem value="hourly">Hourly</MenuItem>
                  <MenuItem value="daily">Daily</MenuItem>
                  <MenuItem value="weekly">Weekly</MenuItem>
                  <MenuItem value="monthly">Monthly</MenuItem>
                </Select>
              </FormControl>

              <FormControl fullWidth sx={{ mb: 3 }}>
                <InputLabel>Data Retention Policy</InputLabel>
                <Select
                  value={systemSettings.dataRetentionPolicy}
                  onChange={(e) => setSystemSettings(prev => ({ ...prev, dataRetentionPolicy: e.target.value }))}
                >
                  <MenuItem value="1year">1 Year</MenuItem>
                  <MenuItem value="3years">3 Years</MenuItem>
                  <MenuItem value="7years">7 Years</MenuItem>
                  <MenuItem value="indefinite">Indefinite</MenuItem>
                </Select>
              </FormControl>
            </Grid>

            <Grid item xs={12} md={6}>
              <Typography variant="h6" fontWeight="600" gutterBottom>
                System Features
              </Typography>

              <List>
                <ListItem>
                  <ListItemIcon>
                    <Warning />
                  </ListItemIcon>
                  <ListItemText
                    primary="Maintenance Mode"
                    secondary="Enable system maintenance mode"
                  />
                  <ListItemSecondaryAction>
                    <Switch
                      checked={systemSettings.maintenanceMode}
                      onChange={(e) => setSystemSettings(prev => ({ ...prev, maintenanceMode: e.target.checked }))}
                    />
                  </ListItemSecondaryAction>
                </ListItem>

                <ListItem>
                  <ListItemIcon>
                    <Settings />
                  </ListItemIcon>
                  <ListItemText
                    primary="Debug Mode"
                    secondary="Enable detailed system logging"
                  />
                  <ListItemSecondaryAction>
                    <Switch
                      checked={systemSettings.debugMode}
                      onChange={(e) => setSystemSettings(prev => ({ ...prev, debugMode: e.target.checked }))}
                    />
                  </ListItemSecondaryAction>
                </ListItem>

                <ListItem>
                  <ListItemIcon>
                    <Security />
                  </ListItemIcon>
                  <ListItemText
                    primary="Auditing"
                    secondary="Enable comprehensive audit logging"
                  />
                  <ListItemSecondaryAction>
                    <Switch
                      checked={systemSettings.auditingEnabled}
                      onChange={(e) => setSystemSettings(prev => ({ ...prev, auditingEnabled: e.target.checked }))}
                    />
                  </ListItemSecondaryAction>
                </ListItem>

                <ListItem>
                  <ListItemIcon>
                    <Shield />
                  </ListItemIcon>
                  <ListItemText
                    primary="Encryption"
                    secondary="Enable data encryption at rest"
                  />
                  <ListItemSecondaryAction>
                    <Switch
                      checked={systemSettings.encryptionEnabled}
                      onChange={(e) => setSystemSettings(prev => ({ ...prev, encryptionEnabled: e.target.checked }))}
                    />
                  </ListItemSecondaryAction>
                </ListItem>
              </List>

              <Button
                variant="contained"
                startIcon={<Save />}
                onClick={handleSaveSystemSettings}
                disabled={loading}
                sx={{ mt: 2 }}
              >
                Save System Settings
              </Button>
            </Grid>
          </Grid>
        </TabPanel>

        {/* Security Tab */}
        <TabPanel value={activeTab} index={2}>
          <Grid container spacing={4}>
            <Grid item xs={12} md={6}>
              <Typography variant="h6" fontWeight="600" gutterBottom>
                Autonomy Settings
              </Typography>

              <Alert severity="warning" sx={{ mb: 3 }}>
                <Typography variant="body2">
                  <strong>Current Autonomy Level: 255 (Maximum)</strong><br />
                  This grants the AI system unrestricted access to all operations.
                  Modify with extreme caution.
                </Typography>
              </Alert>

              <Box sx={{ mb: 3 }}>
                <Typography variant="subtitle2" gutterBottom>
                  Maximum Autonomy Level: {userSettings.maxAutonomy}
                </Typography>
                <LinearProgress
                  variant="determinate"
                  value={(userSettings.maxAutonomy / 255) * 100}
                  color={userSettings.maxAutonomy >= 200 ? 'error' : userSettings.maxAutonomy >= 150 ? 'warning' : 'success'}
                  sx={{ mb: 1 }}
                />
                <TextField
                  type="range"
                  min="1"
                  max="255"
                  value={userSettings.maxAutonomy}
                  onChange={(e) => setUserSettings(prev => ({ ...prev, maxAutonomy: parseInt(e.target.value) }))}
                  fullWidth
                />
              </Box>

              <Typography variant="h6" fontWeight="600" gutterBottom>
                Password & Authentication
              </Typography>

              <Button variant="outlined" sx={{ mb: 2, mr: 2 }}>
                Change Password
              </Button>
              <Button variant="outlined" sx={{ mb: 2 }}>
                Setup 2FA
              </Button>

              <Typography variant="h6" fontWeight="600" gutterBottom sx={{ mt: 3 }}>
                Session Management
              </Typography>

              <FormControlLabel
                control={<Switch defaultChecked />}
                label="Require re-authentication for sensitive operations"
              />
              <FormControlLabel
                control={<Switch defaultChecked />}
                label="Auto-logout after inactivity"
              />
              <FormControlLabel
                control={<Switch />}
                label="Remember login on this device"
              />
            </Grid>

            <Grid item xs={12} md={6}>
              <Typography variant="h6" fontWeight="600" gutterBottom>
                Compliance Frameworks
              </Typography>

              <Box sx={{ mb: 3 }}>
                {systemSettings.complianceFrameworks.map((framework, index) => (
                  <Chip
                    key={index}
                    label={framework}
                    onDelete={() => {
                      setSystemSettings(prev => ({
                        ...prev,
                        complianceFrameworks: prev.complianceFrameworks.filter((_, i) => i !== index)
                      }));
                    }}
                    sx={{ mr: 1, mb: 1 }}
                  />
                ))}
                <Button size="small" startIcon={<Add />} sx={{ ml: 1 }}>
                  Add Framework
                </Button>
              </Box>

              <Typography variant="h6" fontWeight="600" gutterBottom>
                Security Monitoring
              </Typography>

              <List dense>
                <ListItem>
                  <ListItemIcon>
                    <CheckCircle color="success" />
                  </ListItemIcon>
                  <ListItemText
                    primary="Intrusion Detection"
                    secondary="Active - Last scan: 5 minutes ago"
                  />
                </ListItem>

                <ListItem>
                  <ListItemIcon>
                    <CheckCircle color="success" />
                  </ListItemIcon>
                  <ListItemText
                    primary="Vulnerability Assessment"
                    secondary="Active - Last scan: 1 hour ago"
                  />
                </ListItem>

                <ListItem>
                  <ListItemIcon>
                    <CheckCircle color="success" />
                  </ListItemIcon>
                  <ListItemText
                    primary="Behavioral Analysis"
                    secondary="Active - Monitoring user patterns"
                  />
                </ListItem>

                <ListItem>
                  <ListItemIcon>
                    <Warning color="warning" />
                  </ListItemIcon>
                  <ListItemText
                    primary="Threat Intelligence"
                    secondary="Warning - Update available"
                  />
                </ListItem>
              </List>
            </Grid>
          </Grid>
        </TabPanel>

        {/* API Keys Tab */}
        <TabPanel value={activeTab} index={3}>
          <Box display="flex" justifyContent="space-between" alignItems="center" mb={3}>
            <Typography variant="h6" fontWeight="600">
              API Keys Management
            </Typography>
            <Button
              variant="contained"
              startIcon={<Add />}
              onClick={() => setApiKeyDialogOpen(true)}
            >
              Create API Key
            </Button>
          </Box>

          <Grid container spacing={3}>
            {apiKeys.map((apiKey) => (
              <Grid item xs={12} md={6} lg={4} key={apiKey.id}>
                <Card>
                  <CardContent>
                    <Box display="flex" justifyContent="space-between" alignItems="start" mb={2}>
                      <Typography variant="h6" fontWeight="600">
                        {apiKey.name}
                      </Typography>
                      <IconButton
                        size="small"
                        color="error"
                        onClick={() => handleDeleteApiKey(apiKey.id)}
                      >
                        <Delete />
                      </IconButton>
                    </Box>

                    <Typography variant="body2" color="text.secondary" gutterBottom>
                      Key: {apiKey.key}
                    </Typography>

                    <Typography variant="caption" display="block" color="text.secondary">
                      Created: {apiKey.created}
                    </Typography>
                    <Typography variant="caption" display="block" color="text.secondary">
                      Last used: {apiKey.lastUsed}
                    </Typography>

                    <Box mt={2}>
                      <Button size="small" startIcon={<Edit />}>
                        Edit
                      </Button>
                      <Button size="small" startIcon={<Download />} sx={{ ml: 1 }}>
                        Download
                      </Button>
                    </Box>
                  </CardContent>
                </Card>
              </Grid>
            ))}
          </Grid>
        </TabPanel>

        {/* Backup Tab */}
        <TabPanel value={activeTab} index={4}>
          <Grid container spacing={4}>
            <Grid item xs={12} md={6}>
              <Typography variant="h6" fontWeight="600" gutterBottom>
                System Backup
              </Typography>

              <Alert severity="info" sx={{ mb: 3 }}>
                Regular backups ensure data recovery in case of system failure.
                Current backup frequency: {systemSettings.backupFrequency}
              </Alert>

              <Button
                variant="contained"
                startIcon={<Backup />}
                onClick={() => setBackupDialogOpen(true)}
                sx={{ mb: 2, mr: 2 }}
              >
                Create Backup
              </Button>

              <Button
                variant="outlined"
                startIcon={<Restore />}
                sx={{ mb: 2 }}
              >
                Restore from Backup
              </Button>

              <Typography variant="h6" fontWeight="600" gutterBottom sx={{ mt: 3 }}>
                Export/Import
              </Typography>

              <Button
                variant="outlined"
                startIcon={<Download />}
                sx={{ mb: 2, mr: 2 }}
              >
                Export Configuration
              </Button>

              <Button
                variant="outlined"
                startIcon={<Upload />}
                sx={{ mb: 2 }}
              >
                Import Configuration
              </Button>
            </Grid>

            <Grid item xs={12} md={6}>
              <Typography variant="h6" fontWeight="600" gutterBottom>
                Recent Backups
              </Typography>

              <List>
                <ListItem>
                  <ListItemIcon>
                    <CheckCircle color="success" />
                  </ListItemIcon>
                  <ListItemText
                    primary="Full System Backup"
                    secondary="2024-01-20 03:00 AM (2.3 GB)"
                  />
                  <ListItemSecondaryAction>
                    <IconButton>
                      <Download />
                    </IconButton>
                  </ListItemSecondaryAction>
                </ListItem>

                <ListItem>
                  <ListItemIcon>
                    <CheckCircle color="success" />
                  </ListItemIcon>
                  <ListItemText
                    primary="Configuration Backup"
                    secondary="2024-01-19 03:00 AM (45 MB)"
                  />
                  <ListItemSecondaryAction>
                    <IconButton>
                      <Download />
                    </IconButton>
                  </ListItemSecondaryAction>
                </ListItem>

                <ListItem>
                  <ListItemIcon>
                    <CheckCircle color="success" />
                  </ListItemIcon>
                  <ListItemText
                    primary="Database Backup"
                    secondary="2024-01-18 03:00 AM (1.8 GB)"
                  />
                  <ListItemSecondaryAction>
                    <IconButton>
                      <Download />
                    </IconButton>
                  </ListItemSecondaryAction>
                </ListItem>
              </List>
            </Grid>
          </Grid>
        </TabPanel>
      </Card>

      {/* Create API Key Dialog */}
      <Dialog
        open={apiKeyDialogOpen}
        onClose={() => setApiKeyDialogOpen(false)}
        maxWidth="sm"
        fullWidth
      >
        <DialogTitle>Create New API Key</DialogTitle>
        <DialogContent>
          <Box sx={{ pt: 2 }}>
            <TextField
              fullWidth
              label="API Key Name"
              value={newApiKey.name}
              onChange={(e) => setNewApiKey(prev => ({ ...prev, name: e.target.value }))}
              sx={{ mb: 3 }}
            />

            <FormControl fullWidth sx={{ mb: 3 }}>
              <InputLabel>Permissions</InputLabel>
              <Select
                value={newApiKey.permissions}
                onChange={(e) => setNewApiKey(prev => ({ ...prev, permissions: e.target.value }))}
              >
                <MenuItem value="read">Read Only</MenuItem>
                <MenuItem value="write">Read/Write</MenuItem>
                <MenuItem value="admin">Administrator</MenuItem>
              </Select>
            </FormControl>

            <TextField
              fullWidth
              label="Expiry (Days)"
              type="number"
              value={newApiKey.expiryDays}
              onChange={(e) => setNewApiKey(prev => ({ ...prev, expiryDays: parseInt(e.target.value) }))}
            />
          </Box>
        </DialogContent>
        <DialogActions>
          <Button onClick={() => setApiKeyDialogOpen(false)}>
            Cancel
          </Button>
          <Button
            onClick={handleCreateApiKey}
            variant="contained"
            disabled={!newApiKey.name.trim()}
          >
            Create Key
          </Button>
        </DialogActions>
      </Dialog>

      {/* Backup Dialog */}
      <Dialog
        open={backupDialogOpen}
        onClose={() => setBackupDialogOpen(false)}
        maxWidth="sm"
        fullWidth
      >
        <DialogTitle>Create System Backup</DialogTitle>
        <DialogContent>
          <Alert severity="info" sx={{ mb: 3 }}>
            This will create a complete backup of the AION-CR system including
            configuration, data, and AI models. The process may take several minutes.
          </Alert>

          <FormControlLabel
            control={<Switch defaultChecked />}
            label="Include user data"
          />
          <FormControlLabel
            control={<Switch defaultChecked />}
            label="Include AI models"
          />
          <FormControlLabel
            control={<Switch defaultChecked />}
            label="Include system logs"
          />
          <FormControlLabel
            control={<Switch />}
            label="Compress backup"
          />
        </DialogContent>
        <DialogActions>
          <Button onClick={() => setBackupDialogOpen(false)}>
            Cancel
          </Button>
          <Button
            onClick={handleBackup}
            variant="contained"
            disabled={loading}
          >
            {loading ? 'Creating Backup...' : 'Create Backup'}
          </Button>
        </DialogActions>
      </Dialog>
    </Box>
  );
}