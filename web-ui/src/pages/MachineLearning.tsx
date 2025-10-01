// AION-CR Machine Learning Page
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
  Table,
  TableBody,
  TableCell,
  TableContainer,
  TableHead,
  TableRow,
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
  LinearProgress,
  CircularProgress,
  Tabs,
  Tab,
  Alert,
  Paper,
} from '@mui/material';
import {
  Psychology,
  Memory,
  Speed,
  TrendingUp,
  Visibility,
  PlayArrow,
  Stop,
  CloudUpload,
  GetApp,
  Refresh,
  Add,
  Settings,
  Analytics,
  Hub,
  ModelTraining,
  Biotech,
  Computer,
} from '@mui/icons-material';
import { MLModel, QuantumJob } from '../types';
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
      id={`ml-tabpanel-${index}`}
      aria-labelledby={`ml-tab-${index}`}
      {...other}
    >
      {value === index && <Box sx={{ p: 3 }}>{children}</Box>}
    </div>
  );
}

export default function MachineLearning() {
  const [activeTab, setActiveTab] = useState(0);
  const [models, setModels] = useState<MLModel[]>([]);
  const [quantumJobs, setQuantumJobs] = useState<QuantumJob[]>([]);
  const [loading, setLoading] = useState(true);
  const [selectedModel, setSelectedModel] = useState<MLModel | null>(null);
  const [createDialogOpen, setCreateDialogOpen] = useState(false);
  const [detailDialogOpen, setDetailDialogOpen] = useState(false);

  const [newModel, setNewModel] = useState({
    name: '',
    type: 'classification' as const,
    framework: 'tensorflow',
    trainingData: '',
  });

  const [metrics, setMetrics] = useState({
    totalModels: 0,
    activeModels: 0,
    avgAccuracy: 0,
    quantumJobs: 0,
  });

  const loadModels = async () => {
    try {
      setLoading(true);
      const [modelsResponse, quantumResponse] = await Promise.all([
        systemApi.getMLModels(),
        systemApi.getQuantumJobs(),
      ]);

      if (modelsResponse.success) {
        const modelsData = modelsResponse.data?.items || [];
        setModels(modelsData);

        // Calculate metrics
        const activeModels = modelsData.filter(m => m.status === 'deployed').length;
        const avgAccuracy = modelsData.length > 0
          ? modelsData.reduce((sum, m) => sum + m.accuracy, 0) / modelsData.length
          : 0;

        setMetrics(prev => ({
          ...prev,
          totalModels: modelsData.length,
          activeModels,
          avgAccuracy,
        }));
      }

      if (quantumResponse.success) {
        const quantumData = quantumResponse.data?.items || [];
        setQuantumJobs(quantumData);
        setMetrics(prev => ({
          ...prev,
          quantumJobs: quantumData.length,
        }));
      }
    } catch (error) {
      console.error('Failed to load ML data:', error);
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    loadModels();
  }, []);

  const handleCreateModel = async () => {
    try {
      const response = await systemApi.createMLModel({
        ...newModel,
        status: 'training',
        accuracy: 0,
        version: '1.0.0',
        parameters: {},
      });

      if (response.success) {
        setCreateDialogOpen(false);
        setNewModel({
          name: '',
          type: 'classification',
          framework: 'tensorflow',
          trainingData: '',
        });
        loadModels();
      }
    } catch (error) {
      console.error('Failed to create model:', error);
    }
  };

  const handleTrainModel = async (modelId: string) => {
    try {
      await systemApi.trainMLModel(modelId, {
        epochs: 100,
        learningRate: 0.001,
        batchSize: 32,
      });
      loadModels();
    } catch (error) {
      console.error('Failed to train model:', error);
    }
  };

  const handleDeployModel = async (modelId: string) => {
    try {
      await systemApi.deployMLModel(modelId);
      loadModels();
    } catch (error) {
      console.error('Failed to deploy model:', error);
    }
  };

  const handleSubmitQuantumJob = async () => {
    try {
      await systemApi.submitQuantumJob({
        name: 'Compliance Optimization',
        type: 'optimization',
        qubits: 16,
        gates: 1000,
        shots: 8192,
        backend: 'quantum_simulator',
      });
      loadModels();
    } catch (error) {
      console.error('Failed to submit quantum job:', error);
    }
  };

  const getStatusColor = (status: string) => {
    switch (status) {
      case 'deployed': return 'success';
      case 'training': return 'info';
      case 'testing': return 'warning';
      case 'archived': return 'default';
      default: return 'default';
    }
  };

  const getQuantumStatusColor = (status: string) => {
    switch (status) {
      case 'completed': return 'success';
      case 'running': return 'info';
      case 'queued': return 'warning';
      case 'failed': return 'error';
      default: return 'default';
    }
  };

  const getTypeIcon = (type: string) => {
    switch (type) {
      case 'classification': return <Psychology />;
      case 'regression': return <TrendingUp />;
      case 'clustering': return <Hub />;
      case 'anomaly_detection': return <Biotech />;
      case 'nlp': return <Computer />;
      default: return <Memory />;
    }
  };

  return (
    <Box>
      {/* Header */}
      <Box display="flex" justifyContent="space-between" alignItems="center" mb={3}>
        <Box>
          <Typography variant="h4" fontWeight="bold" gutterBottom>
            Machine Learning & AI
          </Typography>
          <Typography variant="body1" color="text.secondary">
            Advanced AI models and quantum computing integration
          </Typography>
        </Box>

        <Box display="flex" gap={2}>
          <Button
            variant="outlined"
            startIcon={<Refresh />}
            onClick={loadModels}
            disabled={loading}
          >
            Refresh
          </Button>
          <Button
            variant="outlined"
            startIcon={<Hub />}
            onClick={handleSubmitQuantumJob}
          >
            Quantum Job
          </Button>
          <Button
            variant="contained"
            startIcon={<Add />}
            onClick={() => setCreateDialogOpen(true)}
          >
            Create Model
          </Button>
        </Box>
      </Box>

      {/* Metrics Cards */}
      <Grid container spacing={3} mb={4}>
        <Grid item xs={12} sm={6} md={3}>
          <Card>
            <CardContent>
              <Box display="flex" alignItems="center" justifyContent="space-between">
                <Box>
                  <Typography variant="h4" fontWeight="bold" color="primary">
                    {metrics.totalModels}
                  </Typography>
                  <Typography variant="subtitle2" color="text.secondary">
                    ML Models
                  </Typography>
                </Box>
                <Psychology color="primary" />
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
                    {metrics.activeModels}
                  </Typography>
                  <Typography variant="subtitle2" color="text.secondary">
                    Active Models
                  </Typography>
                </Box>
                <ModelTraining color="success" />
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
                    {metrics.avgAccuracy.toFixed(1)}%
                  </Typography>
                  <Typography variant="subtitle2" color="text.secondary">
                    Avg Accuracy
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
                    {metrics.quantumJobs}
                  </Typography>
                  <Typography variant="subtitle2" color="text.secondary">
                    Quantum Jobs
                  </Typography>
                </Box>
                <Hub color="warning" />
              </Box>
            </CardContent>
          </Card>
        </Grid>
      </Grid>

      {/* Main Content Tabs */}
      <Card>
        <Box sx={{ borderBottom: 1, borderColor: 'divider' }}>
          <Tabs value={activeTab} onChange={(_, newValue) => setActiveTab(newValue)}>
            <Tab label="ML Models" />
            <Tab label="Training" />
            <Tab label="Quantum Computing" />
            <Tab label="Analytics" />
          </Tabs>
        </Box>

        {/* ML Models Tab */}
        <TabPanel value={activeTab} index={0}>
          {loading ? (
            <Box display="flex" justifyContent="center" py={4}>
              <CircularProgress />
            </Box>
          ) : (
            <TableContainer>
              <Table>
                <TableHead>
                  <TableRow>
                    <TableCell>Model</TableCell>
                    <TableCell>Type</TableCell>
                    <TableCell>Framework</TableCell>
                    <TableCell>Status</TableCell>
                    <TableCell>Accuracy</TableCell>
                    <TableCell>Last Trained</TableCell>
                    <TableCell>Actions</TableCell>
                  </TableRow>
                </TableHead>
                <TableBody>
                  {models.map((model) => (
                    <TableRow key={model.id}>
                      <TableCell>
                        <Box display="flex" alignItems="center" gap={1}>
                          {getTypeIcon(model.type)}
                          <Box>
                            <Typography variant="subtitle2" fontWeight="600">
                              {model.name}
                            </Typography>
                            <Typography variant="caption" color="text.secondary">
                              v{model.version}
                            </Typography>
                          </Box>
                        </Box>
                      </TableCell>

                      <TableCell>
                        <Chip
                          label={model.type.replace('_', ' ').toUpperCase()}
                          size="small"
                          variant="outlined"
                        />
                      </TableCell>

                      <TableCell>
                        <Typography variant="body2">{model.framework}</Typography>
                      </TableCell>

                      <TableCell>
                        <Chip
                          label={model.status.toUpperCase()}
                          size="small"
                          color={getStatusColor(model.status)}
                        />
                      </TableCell>

                      <TableCell>
                        <Box>
                          <Typography variant="body2" fontWeight="600">
                            {model.accuracy.toFixed(1)}%
                          </Typography>
                          <LinearProgress
                            variant="determinate"
                            value={model.accuracy}
                            sx={{ width: 80, mt: 0.5 }}
                            color={model.accuracy >= 90 ? 'success' : model.accuracy >= 70 ? 'warning' : 'error'}
                          />
                        </Box>
                      </TableCell>

                      <TableCell>
                        <Typography variant="caption">
                          {new Date(model.lastTrained).toLocaleDateString()}
                        </Typography>
                      </TableCell>

                      <TableCell>
                        <Box display="flex" gap={1}>
                          <Tooltip title="View Details">
                            <IconButton
                              size="small"
                              onClick={() => {
                                setSelectedModel(model);
                                setDetailDialogOpen(true);
                              }}
                            >
                              <Visibility />
                            </IconButton>
                          </Tooltip>

                          {model.status !== 'training' && (
                            <Tooltip title="Train Model">
                              <IconButton
                                size="small"
                                color="info"
                                onClick={() => handleTrainModel(model.id)}
                              >
                                <PlayArrow />
                              </IconButton>
                            </Tooltip>
                          )}

                          {model.status === 'testing' && (
                            <Tooltip title="Deploy Model">
                              <IconButton
                                size="small"
                                color="success"
                                onClick={() => handleDeployModel(model.id)}
                              >
                                <CloudUpload />
                              </IconButton>
                            </Tooltip>
                          )}

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
        </TabPanel>

        {/* Training Tab */}
        <TabPanel value={activeTab} index={1}>
          <Alert severity="info" sx={{ mb: 3 }}>
            Training pipeline shows real-time model training progress and hyperparameter optimization.
          </Alert>
          {/* Training content would go here */}
        </TabPanel>

        {/* Quantum Computing Tab */}
        <TabPanel value={activeTab} index={2}>
          <Box>
            <Typography variant="h6" gutterBottom>
              Quantum Computing Jobs
            </Typography>

            <Grid container spacing={3} mb={3}>
              <Grid item xs={12} sm={6} md={3}>
                <Paper sx={{ p: 2, textAlign: 'center' }}>
                  <Hub color="primary" sx={{ fontSize: 40, mb: 1 }} />
                  <Typography variant="h6">16 Qubits</Typography>
                  <Typography variant="caption" color="text.secondary">
                    Available
                  </Typography>
                </Paper>
              </Grid>

              <Grid item xs={12} sm={6} md={3}>
                <Paper sx={{ p: 2, textAlign: 'center' }}>
                  <Speed color="success" sx={{ fontSize: 40, mb: 1 }} />
                  <Typography variant="h6">99.8%</Typography>
                  <Typography variant="caption" color="text.secondary">
                    Fidelity
                  </Typography>
                </Paper>
              </Grid>

              <Grid item xs={12} sm={6} md={3}>
                <Paper sx={{ p: 2, textAlign: 'center' }}>
                  <Memory color="info" sx={{ fontSize: 40, mb: 1 }} />
                  <Typography variant="h6">1000μs</Typography>
                  <Typography variant="caption" color="text.secondary">
                    Coherence Time
                  </Typography>
                </Paper>
              </Grid>

              <Grid item xs={12} sm={6} md={3}>
                <Paper sx={{ p: 2, textAlign: 'center' }}>
                  <Analytics color="warning" sx={{ fontSize: 40, mb: 1 }} />
                  <Typography variant="h6">{quantumJobs.length}</Typography>
                  <Typography variant="caption" color="text.secondary">
                    Jobs Executed
                  </Typography>
                </Paper>
              </Grid>
            </Grid>

            <TableContainer>
              <Table>
                <TableHead>
                  <TableRow>
                    <TableCell>Job Name</TableCell>
                    <TableCell>Type</TableCell>
                    <TableCell>Qubits</TableCell>
                    <TableCell>Status</TableCell>
                    <TableCell>Submitted</TableCell>
                    <TableCell>Results</TableCell>
                  </TableRow>
                </TableHead>
                <TableBody>
                  {quantumJobs.map((job) => (
                    <TableRow key={job.id}>
                      <TableCell>
                        <Typography variant="subtitle2" fontWeight="600">
                          {job.name}
                        </Typography>
                      </TableCell>

                      <TableCell>
                        <Chip
                          label={job.type.replace('_', ' ').toUpperCase()}
                          size="small"
                          variant="outlined"
                        />
                      </TableCell>

                      <TableCell>
                        <Typography variant="body2">{job.qubits}</Typography>
                      </TableCell>

                      <TableCell>
                        <Chip
                          label={job.status.toUpperCase()}
                          size="small"
                          color={getQuantumStatusColor(job.status)}
                        />
                      </TableCell>

                      <TableCell>
                        <Typography variant="caption">
                          {new Date(job.submittedAt).toLocaleString()}
                        </Typography>
                      </TableCell>

                      <TableCell>
                        {job.results ? (
                          <Button size="small" startIcon={<GetApp />}>
                            Download
                          </Button>
                        ) : (
                          <Typography variant="caption" color="text.secondary">
                            Pending
                          </Typography>
                        )}
                      </TableCell>
                    </TableRow>
                  ))}
                </TableBody>
              </Table>
            </TableContainer>
          </Box>
        </TabPanel>

        {/* Analytics Tab */}
        <TabPanel value={activeTab} index={3}>
          <Alert severity="info" sx={{ mb: 3 }}>
            ML Analytics dashboard will show model performance trends, resource utilization, and optimization recommendations.
          </Alert>
          {/* Analytics content would go here */}
        </TabPanel>
      </Card>

      {/* Create Model Dialog */}
      <Dialog
        open={createDialogOpen}
        onClose={() => setCreateDialogOpen(false)}
        maxWidth="sm"
        fullWidth
      >
        <DialogTitle>Create New ML Model</DialogTitle>
        <DialogContent>
          <Box sx={{ pt: 2 }}>
            <TextField
              fullWidth
              label="Model Name"
              value={newModel.name}
              onChange={(e) => setNewModel(prev => ({ ...prev, name: e.target.value }))}
              sx={{ mb: 3 }}
            />

            <Grid container spacing={2} sx={{ mb: 3 }}>
              <Grid item xs={6}>
                <FormControl fullWidth>
                  <InputLabel>Model Type</InputLabel>
                  <Select
                    value={newModel.type}
                    onChange={(e) => setNewModel(prev => ({ ...prev, type: e.target.value as any }))}
                  >
                    <MenuItem value="classification">Classification</MenuItem>
                    <MenuItem value="regression">Regression</MenuItem>
                    <MenuItem value="clustering">Clustering</MenuItem>
                    <MenuItem value="anomaly_detection">Anomaly Detection</MenuItem>
                    <MenuItem value="nlp">Natural Language Processing</MenuItem>
                    <MenuItem value="computer_vision">Computer Vision</MenuItem>
                  </Select>
                </FormControl>
              </Grid>

              <Grid item xs={6}>
                <FormControl fullWidth>
                  <InputLabel>Framework</InputLabel>
                  <Select
                    value={newModel.framework}
                    onChange={(e) => setNewModel(prev => ({ ...prev, framework: e.target.value }))}
                  >
                    <MenuItem value="tensorflow">TensorFlow</MenuItem>
                    <MenuItem value="pytorch">PyTorch</MenuItem>
                    <MenuItem value="scikit-learn">Scikit-learn</MenuItem>
                    <MenuItem value="xgboost">XGBoost</MenuItem>
                    <MenuItem value="lightgbm">LightGBM</MenuItem>
                  </Select>
                </FormControl>
              </Grid>
            </Grid>

            <TextField
              fullWidth
              label="Training Data Source"
              value={newModel.trainingData}
              onChange={(e) => setNewModel(prev => ({ ...prev, trainingData: e.target.value }))}
              placeholder="e.g., compliance_documents.csv"
            />
          </Box>
        </DialogContent>
        <DialogActions>
          <Button onClick={() => setCreateDialogOpen(false)}>
            Cancel
          </Button>
          <Button
            onClick={handleCreateModel}
            variant="contained"
            disabled={!newModel.name.trim() || !newModel.trainingData.trim()}
          >
            Create Model
          </Button>
        </DialogActions>
      </Dialog>

      {/* Model Detail Dialog */}
      <Dialog
        open={detailDialogOpen}
        onClose={() => setDetailDialogOpen(false)}
        maxWidth="md"
        fullWidth
      >
        <DialogTitle>
          Model Details: {selectedModel?.name}
        </DialogTitle>
        <DialogContent>
          {selectedModel && (
            <Box sx={{ pt: 2 }}>
              <Grid container spacing={3}>
                <Grid item xs={12} md={6}>
                  <Typography variant="h6" gutterBottom>Performance Metrics</Typography>
                  <Box>
                    <Typography variant="body2">
                      Accuracy: {selectedModel.accuracy.toFixed(2)}%
                    </Typography>
                    {selectedModel.precision && (
                      <Typography variant="body2">
                        Precision: {selectedModel.precision.toFixed(2)}%
                      </Typography>
                    )}
                    {selectedModel.recall && (
                      <Typography variant="body2">
                        Recall: {selectedModel.recall.toFixed(2)}%
                      </Typography>
                    )}
                    {selectedModel.f1Score && (
                      <Typography variant="body2">
                        F1 Score: {selectedModel.f1Score.toFixed(2)}%
                      </Typography>
                    )}
                  </Box>
                </Grid>

                <Grid item xs={12} md={6}>
                  <Typography variant="h6" gutterBottom>Model Info</Typography>
                  <Box>
                    <Typography variant="body2">
                      Type: {selectedModel.type.replace('_', ' ')}
                    </Typography>
                    <Typography variant="body2">
                      Framework: {selectedModel.framework}
                    </Typography>
                    <Typography variant="body2">
                      Version: {selectedModel.version}
                    </Typography>
                    <Typography variant="body2">
                      Training Data: {selectedModel.trainingData}
                    </Typography>
                  </Box>
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