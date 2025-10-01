// AION-CR Deployment Management Page
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
  FormControl,
  InputLabel,
  Select,
  MenuItem,
  Paper,
  Accordion,
  AccordionSummary,
  AccordionDetails,
} from '@mui/material';
import {
  CloudUpload,
  Memory,
  Security,
  Speed,
  Refresh,
  Add,
  Visibility,
  PlayArrow,
  Stop,
  Settings,
  ExpandMore,
  Docker,
  Cloud,
  Storage,
  NetworkCheck,
  Lock,
  Hub,
  Dns,
  Public,
} from '@mui/icons-material';
import { SmartContract, BlockchainTransaction } from '../types';
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
      id={`deployment-tabpanel-${index}`}
      aria-labelledby={`deployment-tab-${index}`}
      {...other}
    >
      {value === index && <Box sx={{ p: 3 }}>{children}</Box>}
    </div>
  );
}

export default function Deployment() {
  const [activeTab, setActiveTab] = useState(0);
  const [smartContracts, setSmartContracts] = useState<SmartContract[]>([]);
  const [blockchainTxs, setBlockchainTxs] = useState<BlockchainTransaction[]>([]);
  const [loading, setLoading] = useState(true);
  const [deployDialogOpen, setDeployDialogOpen] = useState(false);
  const [selectedContract, setSelectedContract] = useState<SmartContract | null>(null);
  const [detailDialogOpen, setDetailDialogOpen] = useState(false);

  const [newDeployment, setNewDeployment] = useState({
    name: '',
    network: 'ethereum',
    type: 'compliance',
    environment: 'testnet',
  });

  const [deploymentMetrics, setDeploymentMetrics] = useState({
    totalContracts: 0,
    activeContracts: 0,
    totalTransactions: 0,
    gasUsed: 0,
  });

  const [cloudServices, setCloudServices] = useState([
    { name: 'AION-Core', status: 'running', instances: 3, region: 'us-east-1' },
    { name: 'ECTUS-Bridge', status: 'running', instances: 2, region: 'us-west-2' },
    { name: 'AI-Engine', status: 'running', instances: 4, region: 'eu-west-1' },
    { name: 'Blockchain-Node', status: 'running', instances: 2, region: 'ap-southeast-1' },
    { name: 'Quantum-Service', status: 'running', instances: 1, region: 'us-central-1' },
  ]);

  const loadDeploymentData = async () => {
    try {
      setLoading(true);
      const [contractsResponse, transactionsResponse] = await Promise.all([
        systemApi.getSmartContracts(),
        systemApi.getBlockchainTransactions({ limit: 50 }),
      ]);

      if (contractsResponse.success) {
        const contracts = contractsResponse.data || [];
        setSmartContracts(contracts);

        const activeContracts = contracts.filter(c => c.status === 'deployed').length;
        const totalGas = contracts.reduce((sum, c) => sum + c.gasUsed, 0);

        setDeploymentMetrics(prev => ({
          ...prev,
          totalContracts: contracts.length,
          activeContracts,
          gasUsed: totalGas,
        }));
      }

      if (transactionsResponse.success) {
        const transactions = transactionsResponse.data?.items || [];
        setBlockchainTxs(transactions);
        setDeploymentMetrics(prev => ({
          ...prev,
          totalTransactions: transactions.length,
        }));
      }
    } catch (error) {
      console.error('Failed to load deployment data:', error);
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    loadDeploymentData();
  }, []);

  const handleDeployContract = async () => {
    try {
      const response = await systemApi.deploySmartContract({
        name: newDeployment.name,
        network: newDeployment.network,
        abi: [], // Would be populated with actual ABI
        bytecode: '0x608060405234801561001057600080fd5b50...', // Example bytecode
      });

      if (response.success) {
        setDeployDialogOpen(false);
        setNewDeployment({
          name: '',
          network: 'ethereum',
          type: 'compliance',
          environment: 'testnet',
        });
        loadDeploymentData();
      }
    } catch (error) {
      console.error('Failed to deploy contract:', error);
    }
  };

  const getStatusColor = (status: string) => {
    switch (status) {
      case 'deployed': case 'running': return 'success';
      case 'pending': return 'warning';
      case 'failed': case 'stopped': return 'error';
      default: return 'default';
    }
  };

  const getNetworkColor = (network: string) => {
    switch (network) {
      case 'ethereum': return 'primary';
      case 'polygon': return 'secondary';
      case 'bsc': return 'warning';
      case 'avalanche': return 'error';
      default: return 'info';
    }
  };

  return (
    <Box>
      {/* Header */}
      <Box display="flex" justifyContent="space-between" alignItems="center" mb={3}>
        <Box>
          <Typography variant="h4" fontWeight="bold" gutterBottom>
            Deployment Management
          </Typography>
          <Typography variant="body1" color="text.secondary">
            Blockchain and cloud infrastructure deployment
          </Typography>
        </Box>

        <Box display="flex" gap={2}>
          <Button
            variant="outlined"
            startIcon={<Refresh />}
            onClick={loadDeploymentData}
            disabled={loading}
          >
            Refresh
          </Button>
          <Button
            variant="contained"
            startIcon={<Add />}
            onClick={() => setDeployDialogOpen(true)}
          >
            New Deployment
          </Button>
        </Box>
      </Box>

      {/* Deployment Metrics */}
      <Grid container spacing={3} mb={4}>
        <Grid item xs={12} sm={6} md={3}>
          <Card>
            <CardContent>
              <Box display="flex" alignItems="center" justifyContent="space-between">
                <Box>
                  <Typography variant="h4" fontWeight="bold" color="primary">
                    {deploymentMetrics.totalContracts}
                  </Typography>
                  <Typography variant="subtitle2" color="text.secondary">
                    Smart Contracts
                  </Typography>
                </Box>
                <Lock color="primary" />
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
                    {deploymentMetrics.activeContracts}
                  </Typography>
                  <Typography variant="subtitle2" color="text.secondary">
                    Active Deployments
                  </Typography>
                </Box>
                <CloudUpload color="success" />
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
                    {deploymentMetrics.totalTransactions.toLocaleString()}
                  </Typography>
                  <Typography variant="subtitle2" color="text.secondary">
                    Transactions
                  </Typography>
                </Box>
                <NetworkCheck color="info" />
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
                    {(deploymentMetrics.gasUsed / 1000000).toFixed(1)}M
                  </Typography>
                  <Typography variant="subtitle2" color="text.secondary">
                    Total Gas Used
                  </Typography>
                </Box>
                <Speed color="warning" />
              </Box>
            </CardContent>
          </Card>
        </Grid>
      </Grid>

      {/* Main Content Tabs */}
      <Card>
        <Box sx={{ borderBottom: 1, borderColor: 'divider' }}>
          <Tabs value={activeTab} onChange={(_, newValue) => setActiveTab(newValue)}>
            <Tab label="Blockchain" />
            <Tab label="Cloud Services" />
            <Tab label="Infrastructure" />
            <Tab label="Monitoring" />
          </Tabs>
        </Box>

        {/* Blockchain Tab */}
        <TabPanel value={activeTab} index={0}>
          <Typography variant="h6" fontWeight="600" gutterBottom>
            Smart Contract Deployments
          </Typography>

          {loading ? (
            <Box display="flex" justifyContent="center" py={4}>
              <CircularProgress />
            </Box>
          ) : (
            <Grid container spacing={3}>
              {smartContracts.map((contract) => (
                <Grid item xs={12} md={6} lg={4} key={contract.id}>
                  <Card>
                    <CardContent>
                      <Box display="flex" alignItems="center" justifyContent="space-between" mb={2}>
                        <Typography variant="h6" fontWeight="600">
                          {contract.name}
                        </Typography>
                        <Chip
                          label={contract.status.toUpperCase()}
                          size="small"
                          color={getStatusColor(contract.status)}
                        />
                      </Box>

                      <Box mb={2}>
                        <Typography variant="body2" color="text.secondary" gutterBottom>
                          Network
                        </Typography>
                        <Chip
                          label={contract.network.toUpperCase()}
                          size="small"
                          color={getNetworkColor(contract.network)}
                          variant="outlined"
                        />
                      </Box>

                      <Box mb={2}>
                        <Typography variant="body2" color="text.secondary" gutterBottom>
                          Address
                        </Typography>
                        <Typography variant="caption" fontFamily="monospace">
                          {contract.address.slice(0, 20)}...
                        </Typography>
                      </Box>

                      <Box mb={2}>
                        <Typography variant="body2" color="text.secondary" gutterBottom>
                          Gas Used
                        </Typography>
                        <Typography variant="body2">
                          {contract.gasUsed.toLocaleString()}
                        </Typography>
                      </Box>

                      <Box mb={2}>
                        <Typography variant="body2" color="text.secondary" gutterBottom>
                          Interactions
                        </Typography>
                        <Typography variant="body2">
                          {contract.interactions.toLocaleString()}
                        </Typography>
                      </Box>

                      <Box display="flex" gap={1} mt={2}>
                        <Tooltip title="View Details">
                          <IconButton
                            size="small"
                            onClick={() => {
                              setSelectedContract(contract);
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
                    </CardContent>
                  </Card>
                </Grid>
              ))}

              {smartContracts.length === 0 && (
                <Grid item xs={12}>
                  <Box textAlign="center" py={4}>
                    <Lock sx={{ fontSize: 48, color: 'text.secondary', mb: 2 }} />
                    <Typography variant="h6" color="text.secondary">
                      No smart contracts deployed
                    </Typography>
                    <Typography variant="body2" color="text.secondary">
                      Deploy your first smart contract to get started
                    </Typography>
                  </Box>
                </Grid>
              )}
            </Grid>
          )}
        </TabPanel>

        {/* Cloud Services Tab */}
        <TabPanel value={activeTab} index={1}>
          <Typography variant="h6" fontWeight="600" gutterBottom>
            Cloud Service Deployments
          </Typography>

          <Grid container spacing={3}>
            {cloudServices.map((service, index) => (
              <Grid item xs={12} md={6} lg={4} key={index}>
                <Card>
                  <CardContent>
                    <Box display="flex" alignItems="center" justifyContent="space-between" mb={2}>
                      <Typography variant="h6" fontWeight="600">
                        {service.name}
                      </Typography>
                      <Chip
                        label={service.status.toUpperCase()}
                        size="small"
                        color={getStatusColor(service.status)}
                      />
                    </Box>

                    <Box mb={2}>
                      <Typography variant="body2" color="text.secondary" gutterBottom>
                        Instances
                      </Typography>
                      <Typography variant="h4" color="primary">
                        {service.instances}
                      </Typography>
                    </Box>

                    <Box mb={2}>
                      <Typography variant="body2" color="text.secondary" gutterBottom>
                        Region
                      </Typography>
                      <Chip label={service.region} size="small" variant="outlined" />
                    </Box>

                    <LinearProgress
                      variant="determinate"
                      value={Math.random() * 100} // Mock load
                      color="success"
                      sx={{ mb: 1 }}
                    />
                    <Typography variant="caption" color="text.secondary">
                      Load: {(Math.random() * 100).toFixed(1)}%
                    </Typography>

                    <Box display="flex" gap={1} mt={2}>
                      <Tooltip title="View Logs">
                        <IconButton size="small">
                          <Visibility />
                        </IconButton>
                      </Tooltip>

                      <Tooltip title="Scale">
                        <IconButton size="small">
                          <CloudUpload />
                        </IconButton>
                      </Tooltip>

                      <Tooltip title="Settings">
                        <IconButton size="small">
                          <Settings />
                        </IconButton>
                      </Tooltip>
                    </Box>
                  </CardContent>
                </Card>
              </Grid>
            ))}
          </Grid>
        </TabPanel>

        {/* Infrastructure Tab */}
        <TabPanel value={activeTab} index={2}>
          <Typography variant="h6" fontWeight="600" gutterBottom>
            Infrastructure Components
          </Typography>

          <Grid container spacing={3}>
            <Grid item xs={12} md={6}>
              <Accordion>
                <AccordionSummary expandIcon={<ExpandMore />}>
                  <Box display="flex" alignItems="center" gap={2}>
                    <Docker />
                    <Typography variant="h6">Container Orchestration</Typography>
                  </Box>
                </AccordionSummary>
                <AccordionDetails>
                  <Typography variant="body2" color="text.secondary">
                    Kubernetes cluster managing containerized services across multiple regions.
                    Auto-scaling enabled with load balancing and health checks.
                  </Typography>
                  <Box mt={2}>
                    <Chip label="Kubernetes 1.28" size="small" />
                    <Chip label="15 Nodes" size="small" sx={{ ml: 1 }} />
                    <Chip label="Auto-scaling" size="small" color="success" sx={{ ml: 1 }} />
                  </Box>
                </AccordionDetails>
              </Accordion>
            </Grid>

            <Grid item xs={12} md={6}>
              <Accordion>
                <AccordionSummary expandIcon={<ExpandMore />}>
                  <Box display="flex" alignItems="center" gap={2}>
                    <Storage />
                    <Typography variant="h6">Distributed Storage</Typography>
                  </Box>
                </AccordionSummary>
                <AccordionDetails>
                  <Typography variant="body2" color="text.secondary">
                    IPFS network for decentralized storage with automatic replication
                    and content addressing for compliance records.
                  </Typography>
                  <Box mt={2}>
                    <Chip label="IPFS Cluster" size="small" />
                    <Chip label="50 TB" size="small" sx={{ ml: 1 }} />
                    <Chip label="3x Replication" size="small" color="info" sx={{ ml: 1 }} />
                  </Box>
                </AccordionDetails>
              </Accordion>
            </Grid>

            <Grid item xs={12} md={6}>
              <Accordion>
                <AccordionSummary expandIcon={<ExpandMore />}>
                  <Box display="flex" alignItems="center" gap={2}>
                    <Dns />
                    <Typography variant="h6">Service Mesh</Typography>
                  </Box>
                </AccordionSummary>
                <AccordionDetails>
                  <Typography variant="body2" color="text.secondary">
                    Istio service mesh providing secure communication, traffic management,
                    and observability between microservices.
                  </Typography>
                  <Box mt={2}>
                    <Chip label="Istio 1.19" size="small" />
                    <Chip label="mTLS Enabled" size="small" color="success" sx={{ ml: 1 }} />
                    <Chip label="Circuit Breaker" size="small" sx={{ ml: 1 }} />
                  </Box>
                </AccordionDetails>
              </Accordion>
            </Grid>

            <Grid item xs={12} md={6}>
              <Accordion>
                <AccordionSummary expandIcon={<ExpandMore />}>
                  <Box display="flex" alignItems="center" gap={2}>
                    <Public />
                    <Typography variant="h6">CDN & Edge Computing</Typography>
                  </Box>
                </AccordionSummary>
                <AccordionDetails>
                  <Typography variant="body2" color="text.secondary">
                    Global content delivery network with edge computing capabilities
                    for low-latency AI inference and compliance checking.
                  </Typography>
                  <Box mt={2}>
                    <Chip label="CloudFlare" size="small" />
                    <Chip label="200+ POPs" size="small" sx={{ ml: 1 }} />
                    <Chip label="Edge Workers" size="small" color="warning" sx={{ ml: 1 }} />
                  </Box>
                </AccordionDetails>
              </Accordion>
            </Grid>
          </Grid>
        </TabPanel>

        {/* Monitoring Tab */}
        <TabPanel value={activeTab} index={3}>
          <Alert severity="info">
            Deployment monitoring dashboard will show infrastructure health, performance metrics,
            and automated scaling events.
          </Alert>
        </TabPanel>
      </Card>

      {/* Deploy Contract Dialog */}
      <Dialog
        open={deployDialogOpen}
        onClose={() => setDeployDialogOpen(false)}
        maxWidth="sm"
        fullWidth
      >
        <DialogTitle>Deploy New Smart Contract</DialogTitle>
        <DialogContent>
          <Box sx={{ pt: 2 }}>
            <TextField
              fullWidth
              label="Contract Name"
              value={newDeployment.name}
              onChange={(e) => setNewDeployment(prev => ({ ...prev, name: e.target.value }))}
              sx={{ mb: 3 }}
            />

            <Grid container spacing={2} sx={{ mb: 3 }}>
              <Grid item xs={6}>
                <FormControl fullWidth>
                  <InputLabel>Network</InputLabel>
                  <Select
                    value={newDeployment.network}
                    onChange={(e) => setNewDeployment(prev => ({ ...prev, network: e.target.value }))}
                  >
                    <MenuItem value="ethereum">Ethereum</MenuItem>
                    <MenuItem value="polygon">Polygon</MenuItem>
                    <MenuItem value="bsc">Binance Smart Chain</MenuItem>
                    <MenuItem value="avalanche">Avalanche</MenuItem>
                  </Select>
                </FormControl>
              </Grid>

              <Grid item xs={6}>
                <FormControl fullWidth>
                  <InputLabel>Environment</InputLabel>
                  <Select
                    value={newDeployment.environment}
                    onChange={(e) => setNewDeployment(prev => ({ ...prev, environment: e.target.value }))}
                  >
                    <MenuItem value="testnet">Testnet</MenuItem>
                    <MenuItem value="mainnet">Mainnet</MenuItem>
                  </Select>
                </FormControl>
              </Grid>
            </Grid>

            <FormControl fullWidth>
              <InputLabel>Contract Type</InputLabel>
              <Select
                value={newDeployment.type}
                onChange={(e) => setNewDeployment(prev => ({ ...prev, type: e.target.value }))}
              >
                <MenuItem value="compliance">Compliance Contract</MenuItem>
                <MenuItem value="audit">Audit Trail Contract</MenuItem>
                <MenuItem value="governance">Governance Contract</MenuItem>
                <MenuItem value="escrow">Escrow Contract</MenuItem>
              </Select>
            </FormControl>

            <Alert severity="warning" sx={{ mt: 3 }}>
              Deploying to mainnet will incur gas fees. Ensure contract is thoroughly tested.
            </Alert>
          </Box>
        </DialogContent>
        <DialogActions>
          <Button onClick={() => setDeployDialogOpen(false)}>
            Cancel
          </Button>
          <Button
            onClick={handleDeployContract}
            variant="contained"
            disabled={!newDeployment.name.trim()}
          >
            Deploy Contract
          </Button>
        </DialogActions>
      </Dialog>

      {/* Contract Detail Dialog */}
      <Dialog
        open={detailDialogOpen}
        onClose={() => setDetailDialogOpen(false)}
        maxWidth="md"
        fullWidth
      >
        <DialogTitle>
          Contract Details: {selectedContract?.name}
        </DialogTitle>
        <DialogContent>
          {selectedContract && (
            <Box sx={{ pt: 2 }}>
              <Grid container spacing={3}>
                <Grid item xs={12} md={6}>
                  <Typography variant="h6" gutterBottom>Contract Information</Typography>
                  <Typography variant="body2">
                    <strong>Address:</strong> {selectedContract.address}
                  </Typography>
                  <Typography variant="body2">
                    <strong>Network:</strong> {selectedContract.network}
                  </Typography>
                  <Typography variant="body2">
                    <strong>Status:</strong> {selectedContract.status}
                  </Typography>
                  <Typography variant="body2">
                    <strong>Deployed:</strong> {new Date(selectedContract.deployedAt).toLocaleString()}
                  </Typography>
                </Grid>

                <Grid item xs={12} md={6}>
                  <Typography variant="h6" gutterBottom>Usage Statistics</Typography>
                  <Typography variant="body2">
                    <strong>Gas Used:</strong> {selectedContract.gasUsed.toLocaleString()}
                  </Typography>
                  <Typography variant="body2">
                    <strong>Interactions:</strong> {selectedContract.interactions.toLocaleString()}
                  </Typography>
                  <Typography variant="body2">
                    <strong>Transaction Hash:</strong>
                  </Typography>
                  <Typography variant="caption" fontFamily="monospace">
                    {selectedContract.transactionHash}
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