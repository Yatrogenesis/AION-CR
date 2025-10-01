// AION-CR Autonomous Agents Page
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
  LinearProgress,
  Table,
  TableBody,
  TableCell,
  TableContainer,
  TableHead,
  TableRow,
  Paper,
  IconButton,
  Tooltip,
  Dialog,
  DialogTitle,
  DialogContent,
  DialogActions,
  TextField,
  FormControl,
  InputLabel,
  Select,
  MenuItem,
  Switch,
  FormControlLabel,
  Alert,
  CircularProgress,
} from '@mui/material';
import {
  PlayArrow,
  Stop,
  Settings,
  Add,
  Psychology,
  SmartToy,
  TrendingUp,
  Warning,
  CheckCircle,
  Speed,
  Memory,
  AutoFixHigh,
  Refresh,
  Edit,
  Delete,
  Visibility,
} from '@mui/icons-material';
import { AutonomousAgent } from '../types';
import { systemApi } from '../services/api';

export default function Agents() {
  const [agents, setAgents] = useState<AutonomousAgent[]>([]);
  const [loading, setLoading] = useState(true);
  const [selectedAgent, setSelectedAgent] = useState<AutonomousAgent | null>(null);
  const [createDialogOpen, setCreateDialogOpen] = useState(false);
  const [detailDialogOpen, setDetailDialogOpen] = useState(false);
  const [newAgent, setNewAgent] = useState({
    name: '',
    type: 'compliance' as const,
    autonomyLevel: 100,
    learningEnabled: true,
  });

  const loadAgents = async () => {
    try {
      setLoading(true);
      const response = await systemApi.getAgents();
      if (response.success) {
        setAgents(response.data?.items || []);
      }
    } catch (error) {
      console.error('Failed to load agents:', error);
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    loadAgents();
  }, []);

  const handleStartAgent = async (agentId: string) => {
    try {
      await systemApi.startAgent(agentId);
      loadAgents();
    } catch (error) {
      console.error('Failed to start agent:', error);
    }
  };

  const handleStopAgent = async (agentId: string) => {
    try {
      await systemApi.stopAgent(agentId);
      loadAgents();
    } catch (error) {
      console.error('Failed to stop agent:', error);
    }
  };

  const handleCreateAgent = async () => {
    try {
      const response = await systemApi.createAgent({
        ...newAgent,
        capabilities: getAgentCapabilities(newAgent.type),
        configuration: {
          maxAutonomy: newAgent.autonomyLevel,
          allowedActions: getDefaultActions(newAgent.type),
          escalationRules: ['high_risk_decision', 'compliance_violation'],
          learningEnabled: newAgent.learningEnabled,
          parameters: {},
        },
      });

      if (response.success) {
        setCreateDialogOpen(false);
        setNewAgent({
          name: '',
          type: 'compliance',
          autonomyLevel: 100,
          learningEnabled: true,
        });
        loadAgents();
      }
    } catch (error) {
      console.error('Failed to create agent:', error);
    }
  };

  const getAgentCapabilities = (type: string) => {
    switch (type) {
      case 'compliance': return ['rule_validation', 'document_analysis', 'risk_assessment'];
      case 'monitoring': return ['system_health', 'anomaly_detection', 'performance_optimization'];
      case 'analysis': return ['data_processing', 'pattern_recognition', 'reporting'];
      case 'optimization': return ['resource_allocation', 'workflow_optimization', 'cost_reduction'];
      case 'security': return ['threat_detection', 'access_control', 'incident_response'];
      default: return ['basic_operations'];
    }
  };

  const getDefaultActions = (type: string) => {
    const base = ['read_data', 'generate_reports', 'send_notifications'];
    switch (type) {
      case 'compliance': return [...base, 'create_rules', 'validate_documents', 'escalate_violations'];
      case 'monitoring': return [...base, 'restart_services', 'adjust_resources', 'create_alerts'];
      case 'analysis': return [...base, 'process_data', 'create_insights', 'update_models'];
      case 'optimization': return [...base, 'modify_workflows', 'reallocate_resources', 'update_configs'];
      case 'security': return [...base, 'block_access', 'quarantine_threats', 'update_policies'];
      default: return base;
    }
  };

  const getStatusColor = (status: string) => {
    switch (status) {
      case 'active': return 'success';
      case 'training': return 'info';
      case 'error': return 'error';
      default: return 'default';
    }
  };

  const getAutonomyColor = (level: number) => {
    if (level >= 200) return 'error';
    if (level >= 150) return 'warning';
    if (level >= 100) return 'info';
    return 'success';
  };

  const activeAgents = agents.filter(agent => agent.status === 'active').length;
  const totalTasks = agents.reduce((sum, agent) => sum + agent.performance.tasksCompleted, 0);
  const avgSuccessRate = agents.length > 0
    ? agents.reduce((sum, agent) => sum + agent.performance.successRate, 0) / agents.length
    : 0;

  return (
    <Box>
      {/* Header */}
      <Box display="flex" justifyContent="space-between" alignItems="center" mb={3}>
        <Box>
          <Typography variant="h4" fontWeight="bold" gutterBottom>
            Autonomous Agents
          </Typography>
          <Typography variant="body1" color="text.secondary">
            AI agents operating at maximum autonomy level 255
          </Typography>
        </Box>

        <Box display="flex" gap={2}>
          <Button
            variant="outlined"
            startIcon={<Refresh />}
            onClick={loadAgents}
            disabled={loading}
          >
            Refresh
          </Button>
          <Button
            variant="contained"
            startIcon={<Add />}
            onClick={() => setCreateDialogOpen(true)}
          >
            Create Agent
          </Button>
        </Box>
      </Box>

      {/* Overview Cards */}
      <Grid container spacing={3} mb={4}>
        <Grid item xs={12} sm={6} md={3}>
          <Card>
            <CardContent>
              <Box display="flex" alignItems="center" justifyContent="space-between">
                <Box>
                  <Typography variant="h4" fontWeight="bold" color="primary">
                    {activeAgents}
                  </Typography>
                  <Typography variant="subtitle2" color="text.secondary">
                    Active Agents
                  </Typography>
                </Box>
                <SmartToy color="primary" />
              </Box>
            </CardContent>
          </Card>
        </Grid>

        <Grid item xs={12} sm={6} md={3}>
          <Card>
            <CardContent>
              <Box display="flex" alignItems="center" justifyContent="space-between">
                <Box>
                  <Typography variant="h4" fontWeight="bold" color="success">
                    {totalTasks.toLocaleString()}
                  </Typography>
                  <Typography variant="subtitle2" color="text.secondary">
                    Tasks Completed
                  </Typography>
                </Box>
                <CheckCircle color="success" />
              </Box>
            </CardContent>
          </Card>
        </Grid>

        <Grid item xs={12} sm={6} md={3}>
          <Card>
            <CardContent>
              <Box display="flex" alignItems="center" justifyContent="space-between">
                <Box>
                  <Typography variant="h4" fontWeight="bold" color="info">
                    {avgSuccessRate.toFixed(1)}%
                  </Typography>
                  <Typography variant="subtitle2" color="text.secondary">
                    Avg Success Rate
                  </Typography>
                </Box>
                <TrendingUp color="info" />
              </Box>
            </CardContent>
          </Card>
        </Grid>

        <Grid item xs={12} sm={6} md={3}>
          <Card>
            <CardContent>
              <Box display="flex" alignItems="center" justifyContent="space-between">
                <Box>
                  <Typography variant="h4" fontWeight="bold" color="warning">
                    255
                  </Typography>
                  <Typography variant="subtitle2" color="text.secondary">
                    Max Autonomy Level
                  </Typography>
                </Box>
                <AutoFixHigh color="warning" />
              </Box>
            </CardContent>
          </Card>
        </Grid>
      </Grid>

      {/* Agents Table */}
      <Card>
        <CardContent>
          <Typography variant="h6" fontWeight="600" gutterBottom>
            Agent Management
          </Typography>

          {loading ? (
            <Box display="flex" justifyContent="center" py={4}>
              <CircularProgress />
            </Box>
          ) : (
            <TableContainer>
              <Table>
                <TableHead>
                  <TableRow>
                    <TableCell>Agent</TableCell>
                    <TableCell>Type</TableCell>
                    <TableCell>Status</TableCell>
                    <TableCell>Autonomy</TableCell>
                    <TableCell>Performance</TableCell>
                    <TableCell>Last Action</TableCell>
                    <TableCell>Actions</TableCell>
                  </TableRow>
                </TableHead>
                <TableBody>
                  {agents.map((agent) => (
                    <TableRow key={agent.id}>
                      <TableCell>
                        <Box>
                          <Typography variant="subtitle2" fontWeight="600">
                            {agent.name}
                          </Typography>
                          <Typography variant="caption" color="text.secondary">
                            {agent.id}
                          </Typography>
                        </Box>
                      </TableCell>

                      <TableCell>
                        <Chip
                          label={agent.type.replace('_', ' ').toUpperCase()}
                          size="small"
                          variant="outlined"
                        />
                      </TableCell>

                      <TableCell>
                        <Chip
                          label={agent.status.toUpperCase()}
                          size="small"
                          color={getStatusColor(agent.status)}
                        />
                      </TableCell>

                      <TableCell>
                        <Box>
                          <Chip
                            label={`L${agent.autonomyLevel}`}
                            size="small"
                            color={getAutonomyColor(agent.autonomyLevel)}
                          />
                          <LinearProgress
                            variant="determinate"
                            value={(agent.autonomyLevel / 255) * 100}
                            sx={{ mt: 1, width: 80 }}
                            color={getAutonomyColor(agent.autonomyLevel)}
                          />
                        </Box>
                      </TableCell>

                      <TableCell>
                        <Box>
                          <Typography variant="caption" display="block">
                            {agent.performance.successRate.toFixed(1)}% success
                          </Typography>
                          <Typography variant="caption" color="text.secondary">
                            {agent.performance.tasksCompleted} tasks
                          </Typography>
                        </Box>
                      </TableCell>

                      <TableCell>
                        <Typography variant="caption">
                          {new Date(agent.lastAction).toLocaleString()}
                        </Typography>
                      </TableCell>

                      <TableCell>
                        <Box display="flex" gap={1}>
                          {agent.status === 'active' ? (
                            <Tooltip title="Stop Agent">
                              <IconButton
                                size="small"
                                color="error"
                                onClick={() => handleStopAgent(agent.id)}
                              >
                                <Stop />
                              </IconButton>
                            </Tooltip>
                          ) : (
                            <Tooltip title="Start Agent">
                              <IconButton
                                size="small"
                                color="success"
                                onClick={() => handleStartAgent(agent.id)}
                              >
                                <PlayArrow />
                              </IconButton>
                            </Tooltip>
                          )}

                          <Tooltip title="View Details">
                            <IconButton
                              size="small"
                              onClick={() => {
                                setSelectedAgent(agent);
                                setDetailDialogOpen(true);
                              }}
                            >
                              <Visibility />
                            </IconButton>
                          </Tooltip>

                          <Tooltip title="Settings">
                            <IconButton size="small">
                              <Settings />
                            </IconButton>
                          </Tooltip>
                        </Box>
                      </TableCell>
                    </TableRow>
                  ))}
                </TableBody>
              </Table>
            </TableContainer>
          )}
        </CardContent>
      </Card>

      {/* Create Agent Dialog */}
      <Dialog
        open={createDialogOpen}
        onClose={() => setCreateDialogOpen(false)}
        maxWidth="sm"
        fullWidth
      >
        <DialogTitle>Create New Autonomous Agent</DialogTitle>
        <DialogContent>
          <Box sx={{ pt: 2 }}>
            <TextField
              fullWidth
              label="Agent Name"
              value={newAgent.name}
              onChange={(e) => setNewAgent(prev => ({ ...prev, name: e.target.value }))}
              sx={{ mb: 3 }}
            />

            <FormControl fullWidth sx={{ mb: 3 }}>
              <InputLabel>Agent Type</InputLabel>
              <Select
                value={newAgent.type}
                onChange={(e) => setNewAgent(prev => ({ ...prev, type: e.target.value as any }))}
              >
                <MenuItem value="compliance">Compliance</MenuItem>
                <MenuItem value="monitoring">Monitoring</MenuItem>
                <MenuItem value="analysis">Analysis</MenuItem>
                <MenuItem value="optimization">Optimization</MenuItem>
                <MenuItem value="security">Security</MenuItem>
              </Select>
            </FormControl>

            <Box sx={{ mb: 3 }}>
              <Typography variant="subtitle2" gutterBottom>
                Autonomy Level: {newAgent.autonomyLevel}
              </Typography>
              <LinearProgress
                variant="determinate"
                value={(newAgent.autonomyLevel / 255) * 100}
                color={getAutonomyColor(newAgent.autonomyLevel)}
                sx={{ mb: 1 }}
              />
              <TextField
                type="range"
                min="1"
                max="255"
                value={newAgent.autonomyLevel}
                onChange={(e) => setNewAgent(prev => ({ ...prev, autonomyLevel: parseInt(e.target.value) }))}
                fullWidth
              />
            </Box>

            <FormControlLabel
              control={
                <Switch
                  checked={newAgent.learningEnabled}
                  onChange={(e) => setNewAgent(prev => ({ ...prev, learningEnabled: e.target.checked }))}
                />
              }
              label="Enable Machine Learning"
            />

            {newAgent.autonomyLevel >= 200 && (
              <Alert severity="warning" sx={{ mt: 2 }}>
                High autonomy level (200+) grants extensive system access. Use with caution.
              </Alert>
            )}
          </Box>
        </DialogContent>
        <DialogActions>
          <Button onClick={() => setCreateDialogOpen(false)}>
            Cancel
          </Button>
          <Button
            onClick={handleCreateAgent}
            variant="contained"
            disabled={!newAgent.name.trim()}
          >
            Create Agent
          </Button>
        </DialogActions>
      </Dialog>

      {/* Agent Detail Dialog */}
      <Dialog
        open={detailDialogOpen}
        onClose={() => setDetailDialogOpen(false)}
        maxWidth="md"
        fullWidth
      >
        <DialogTitle>
          Agent Details: {selectedAgent?.name}
        </DialogTitle>
        <DialogContent>
          {selectedAgent && (
            <Box sx={{ pt: 2 }}>
              <Grid container spacing={3}>
                <Grid item xs={12} md={6}>
                  <Typography variant="h6" gutterBottom>Performance Metrics</Typography>
                  <Box>
                    <Typography variant="body2">
                      Tasks Completed: {selectedAgent.performance.tasksCompleted}
                    </Typography>
                    <Typography variant="body2">
                      Success Rate: {selectedAgent.performance.successRate.toFixed(1)}%
                    </Typography>
                    <Typography variant="body2">
                      Avg Response Time: {selectedAgent.performance.averageResponseTime}ms
                    </Typography>
                    <Typography variant="body2">
                      Uptime: {selectedAgent.performance.uptime.toFixed(1)}%
                    </Typography>
                  </Box>
                </Grid>

                <Grid item xs={12} md={6}>
                  <Typography variant="h6" gutterBottom>Capabilities</Typography>
                  <Box display="flex" flexWrap="wrap" gap={1}>
                    {selectedAgent.capabilities.map((capability, index) => (
                      <Chip
                        key={index}
                        label={capability.replace('_', ' ')}
                        size="small"
                        variant="outlined"
                      />
                    ))}
                  </Box>
                </Grid>

                <Grid item xs={12}>
                  <Typography variant="h6" gutterBottom>Configuration</Typography>
                  <Typography variant="body2">
                    Max Autonomy: {selectedAgent.configuration.maxAutonomy}
                  </Typography>
                  <Typography variant="body2">
                    Learning Enabled: {selectedAgent.configuration.learningEnabled ? 'Yes' : 'No'}
                  </Typography>
                  <Typography variant="body2">
                    Allowed Actions: {selectedAgent.configuration.allowedActions.length}
                  </Typography>
                </Grid>
              </Grid>
            </Box>
          )}
        </DialogContent>
        <DialogActions>
          <Button onClick={() => setDetailDialogOpen(false)}>
            Close
          </Button>
        </DialogActions>
      </Dialog>
    </Box>
  );
}