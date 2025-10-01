// AION-CR Compliance Management Page
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
  LinearProgress,
  Tabs,
  Tab,
  Accordion,
  AccordionSummary,
  AccordionDetails,
} from '@mui/material';
import {
  Security,
  Add,
  Edit,
  Delete,
  PlayArrow,
  Stop,
  Visibility,
  ExpandMore,
  Warning,
  CheckCircle,
  Error,
  Info,
  Refresh,
  GetApp,
  Upload,
  Analytics,
  Rule,
  Assessment,
  Gavel,
} from '@mui/icons-material';
import { ComplianceRule } from '../types';
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
      id={`compliance-tabpanel-${index}`}
      aria-labelledby={`compliance-tab-${index}`}
      {...other}
    >
      {value === index && <Box sx={{ p: 3 }}>{children}</Box>}
    </div>
  );
}

export default function Compliance() {
  const [activeTab, setActiveTab] = useState(0);
  const [rules, setRules] = useState<ComplianceRule[]>([]);
  const [loading, setLoading] = useState(true);
  const [selectedRule, setSelectedRule] = useState<ComplianceRule | null>(null);
  const [createDialogOpen, setCreateDialogOpen] = useState(false);
  const [detailDialogOpen, setDetailDialogOpen] = useState(false);
  const [frameworks, setFrameworks] = useState([
    'SOX', 'GDPR', 'HIPAA', 'PCI-DSS', 'ISO-27001', 'NIST', 'COBIT', 'ITIL'
  ]);

  const [newRule, setNewRule] = useState({
    name: '',
    description: '',
    framework: '',
    category: '',
    severity: 'medium' as const,
    enabled: true,
  });

  const [complianceMetrics, setComplianceMetrics] = useState({
    overallScore: 87,
    totalRules: 0,
    activeRules: 0,
    violations: 0,
    frameworks: {} as Record<string, number>,
  });

  const loadRules = async () => {
    try {
      setLoading(true);
      const response = await systemApi.getComplianceRules();
      if (response.success) {
        const rulesData = response.data?.items || [];
        setRules(rulesData);

        // Calculate metrics
        const activeRules = rulesData.filter(r => r.enabled).length;
        const violations = rulesData.filter(r => r.severity === 'high' || r.severity === 'critical').length;
        const frameworkCount = rulesData.reduce((acc, rule) => {
          acc[rule.framework] = (acc[rule.framework] || 0) + 1;
          return acc;
        }, {} as Record<string, number>);

        setComplianceMetrics({
          overallScore: Math.max(60, 100 - (violations * 5)),
          totalRules: rulesData.length,
          activeRules,
          violations,
          frameworks: frameworkCount,
        });
      }
    } catch (error) {
      console.error('Failed to load compliance rules:', error);
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    loadRules();
  }, []);

  const handleCreateRule = async () => {
    try {
      const response = await systemApi.createComplianceRule({
        ...newRule,
        conditions: [],
        actions: [{ type: 'alert', parameters: {} }],
      });

      if (response.success) {
        setCreateDialogOpen(false);
        setNewRule({
          name: '',
          description: '',
          framework: '',
          category: '',
          severity: 'medium',
          enabled: true,
        });
        loadRules();
      }
    } catch (error) {
      console.error('Failed to create rule:', error);
    }
  };

  const handleToggleRule = async (ruleId: string, enabled: boolean) => {
    try {
      await systemApi.updateComplianceRule(ruleId, { enabled });
      loadRules();
    } catch (error) {
      console.error('Failed to toggle rule:', error);
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

  const getComplianceScoreColor = (score: number) => {
    if (score >= 90) return 'success';
    if (score >= 70) return 'warning';
    return 'error';
  };

  return (
    <Box>
      {/* Header */}
      <Box display="flex" justifyContent="space-between" alignItems="center" mb={3}>
        <Box>
          <Typography variant="h4" fontWeight="bold" gutterBottom>
            Compliance Management
          </Typography>
          <Typography variant="body1" color="text.secondary">
            Regulatory compliance monitoring and rule management
          </Typography>
        </Box>

        <Box display="flex" gap={2}>
          <Button variant="outlined" startIcon={<Refresh />} onClick={loadRules}>
            Refresh
          </Button>
          <Button variant="outlined" startIcon={<GetApp />}>
            Export Rules
          </Button>
          <Button variant="contained" startIcon={<Add />} onClick={() => setCreateDialogOpen(true)}>
            Create Rule
          </Button>
        </Box>
      </Box>

      {/* Compliance Overview Cards */}
      <Grid container spacing={3} mb={4}>
        <Grid item xs={12} sm={6} md={3}>
          <Card>
            <CardContent>
              <Box display="flex" alignItems="center" justifyContent="space-between">
                <Box>
                  <Typography variant="h4" fontWeight="bold" color={getComplianceScoreColor(complianceMetrics.overallScore)}>
                    {complianceMetrics.overallScore}%
                  </Typography>
                  <Typography variant="subtitle2" color="text.secondary">
                    Compliance Score
                  </Typography>
                </Box>
                <Security color={getComplianceScoreColor(complianceMetrics.overallScore)} />
              </Box>
              <LinearProgress
                variant="determinate"
                value={complianceMetrics.overallScore}
                color={getComplianceScoreColor(complianceMetrics.overallScore)}
                sx={{ mt: 1 }}
              />
            </CardContent>
          </Card>
        </Grid>

        <Grid item xs={12} sm={6} md={3}>
          <Card>
            <CardContent>
              <Box display="flex" alignItems="center" justifyContent="space-between">
                <Box>
                  <Typography variant="h4" fontWeight="bold" color="info">
                    {complianceMetrics.activeRules}
                  </Typography>
                  <Typography variant="subtitle2" color="text.secondary">
                    Active Rules
                  </Typography>
                </Box>
                <Rule color="info" />
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
                    {complianceMetrics.violations}
                  </Typography>
                  <Typography variant="subtitle2" color="text.secondary">
                    Violations
                  </Typography>
                </Box>
                <Warning color="warning" />
              </Box>
            </CardContent>
          </Card>
        </Grid>

        <Grid item xs={12} sm={6} md={3}>
          <Card>
            <CardContent>
              <Box display="flex" alignItems="center" justifyContent="space-between">
                <Box>
                  <Typography variant="h4" fontWeight="bold" color="primary">
                    {Object.keys(complianceMetrics.frameworks).length}
                  </Typography>
                  <Typography variant="subtitle2" color="text.secondary">
                    Frameworks
                  </Typography>
                </Box>
                <Gavel color="primary" />
              </Box>
            </CardContent>
          </Card>
        </Grid>
      </Grid>

      {/* Main Content Tabs */}
      <Card>
        <Box sx={{ borderBottom: 1, borderColor: 'divider' }}>
          <Tabs value={activeTab} onChange={(_, newValue) => setActiveTab(newValue)}>
            <Tab label="Rules Engine" />
            <Tab label="Frameworks" />
            <Tab label="Audit Trail" />
            <Tab label="Analytics" />
          </Tabs>
        </Box>

        {/* Rules Engine Tab */}
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
                    <TableCell>Rule Name</TableCell>
                    <TableCell>Framework</TableCell>
                    <TableCell>Category</TableCell>
                    <TableCell>Severity</TableCell>
                    <TableCell>Status</TableCell>
                    <TableCell>Created</TableCell>
                    <TableCell>Actions</TableCell>
                  </TableRow>
                </TableHead>
                <TableBody>
                  {rules.map((rule) => (
                    <TableRow key={rule.id}>
                      <TableCell>
                        <Box>
                          <Typography variant="subtitle2" fontWeight="600">
                            {rule.name}
                          </Typography>
                          <Typography variant="caption" color="text.secondary">
                            {rule.description}
                          </Typography>
                        </Box>
                      </TableCell>

                      <TableCell>
                        <Chip label={rule.framework} size="small" variant="outlined" />
                      </TableCell>

                      <TableCell>
                        <Typography variant="body2">{rule.category}</Typography>
                      </TableCell>

                      <TableCell>
                        <Chip
                          label={rule.severity.toUpperCase()}
                          size="small"
                          color={getSeverityColor(rule.severity)}
                        />
                      </TableCell>

                      <TableCell>
                        <FormControlLabel
                          control={
                            <Switch
                              checked={rule.enabled}
                              onChange={(e) => handleToggleRule(rule.id, e.target.checked)}
                              size="small"
                            />
                          }
                          label={rule.enabled ? 'Active' : 'Inactive'}
                        />
                      </TableCell>

                      <TableCell>
                        <Typography variant="caption">
                          {new Date(rule.createdAt).toLocaleDateString()}
                        </Typography>
                      </TableCell>

                      <TableCell>
                        <Box display="flex" gap={1}>
                          <Tooltip title="View Details">
                            <IconButton
                              size="small"
                              onClick={() => {
                                setSelectedRule(rule);
                                setDetailDialogOpen(true);
                              }}
                            >
                              <Visibility />
                            </IconButton>
                          </Tooltip>

                          <Tooltip title="Edit Rule">
                            <IconButton size="small">
                              <Edit />
                            </IconButton>
                          </Tooltip>

                          <Tooltip title="Delete Rule">
                            <IconButton size="small" color="error">
                              <Delete />
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

        {/* Frameworks Tab */}
        <TabPanel value={activeTab} index={1}>
          <Grid container spacing={3}>
            {frameworks.map((framework) => (
              <Grid item xs={12} sm={6} md={4} key={framework}>
                <Card>
                  <CardContent>
                    <Typography variant="h6" fontWeight="600" gutterBottom>
                      {framework}
                    </Typography>
                    <Typography variant="h4" color="primary" fontWeight="bold">
                      {complianceMetrics.frameworks[framework] || 0}
                    </Typography>
                    <Typography variant="caption" color="text.secondary">
                      Active Rules
                    </Typography>
                    <Box mt={2}>
                      <Button size="small" variant="outlined" fullWidth>
                        View Details
                      </Button>
                    </Box>
                  </CardContent>
                </Card>
              </Grid>
            ))}
          </Grid>
        </TabPanel>

        {/* Audit Trail Tab */}
        <TabPanel value={activeTab} index={2}>
          <Typography variant="h6" gutterBottom>
            Compliance Audit Trail
          </Typography>
          <Alert severity="info" sx={{ mb: 3 }}>
            Audit trail functionality will show compliance events, violations, and remediation actions.
          </Alert>
          {/* Audit trail content would go here */}
        </TabPanel>

        {/* Analytics Tab */}
        <TabPanel value={activeTab} index={3}>
          <Typography variant="h6" gutterBottom>
            Compliance Analytics
          </Typography>
          <Alert severity="info" sx={{ mb: 3 }}>
            Analytics dashboard will show compliance trends, risk assessments, and performance metrics.
          </Alert>
          {/* Analytics content would go here */}
        </TabPanel>
      </Card>

      {/* Create Rule Dialog */}
      <Dialog
        open={createDialogOpen}
        onClose={() => setCreateDialogOpen(false)}
        maxWidth="md"
        fullWidth
      >
        <DialogTitle>Create Compliance Rule</DialogTitle>
        <DialogContent>
          <Box sx={{ pt: 2 }}>
            <TextField
              fullWidth
              label="Rule Name"
              value={newRule.name}
              onChange={(e) => setNewRule(prev => ({ ...prev, name: e.target.value }))}
              sx={{ mb: 3 }}
            />

            <TextField
              fullWidth
              label="Description"
              multiline
              rows={3}
              value={newRule.description}
              onChange={(e) => setNewRule(prev => ({ ...prev, description: e.target.value }))}
              sx={{ mb: 3 }}
            />

            <Grid container spacing={2} sx={{ mb: 3 }}>
              <Grid item xs={6}>
                <FormControl fullWidth>
                  <InputLabel>Framework</InputLabel>
                  <Select
                    value={newRule.framework}
                    onChange={(e) => setNewRule(prev => ({ ...prev, framework: e.target.value }))}
                  >
                    {frameworks.map((framework) => (
                      <MenuItem key={framework} value={framework}>
                        {framework}
                      </MenuItem>
                    ))}
                  </Select>
                </FormControl>
              </Grid>

              <Grid item xs={6}>
                <TextField
                  fullWidth
                  label="Category"
                  value={newRule.category}
                  onChange={(e) => setNewRule(prev => ({ ...prev, category: e.target.value }))}
                />
              </Grid>
            </Grid>

            <FormControl fullWidth sx={{ mb: 3 }}>
              <InputLabel>Severity</InputLabel>
              <Select
                value={newRule.severity}
                onChange={(e) => setNewRule(prev => ({ ...prev, severity: e.target.value as any }))}
              >
                <MenuItem value="low">Low</MenuItem>
                <MenuItem value="medium">Medium</MenuItem>
                <MenuItem value="high">High</MenuItem>
                <MenuItem value="critical">Critical</MenuItem>
              </Select>
            </FormControl>

            <FormControlLabel
              control={
                <Switch
                  checked={newRule.enabled}
                  onChange={(e) => setNewRule(prev => ({ ...prev, enabled: e.target.checked }))}
                />
              }
              label="Enable Rule"
            />
          </Box>
        </DialogContent>
        <DialogActions>
          <Button onClick={() => setCreateDialogOpen(false)}>
            Cancel
          </Button>
          <Button
            onClick={handleCreateRule}
            variant="contained"
            disabled={!newRule.name.trim() || !newRule.framework}
          >
            Create Rule
          </Button>
        </DialogActions>
      </Dialog>

      {/* Rule Detail Dialog */}
      <Dialog
        open={detailDialogOpen}
        onClose={() => setDetailDialogOpen(false)}
        maxWidth="md"
        fullWidth
      >
        <DialogTitle>
          Rule Details: {selectedRule?.name}
        </DialogTitle>
        <DialogContent>
          {selectedRule && (
            <Box sx={{ pt: 2 }}>
              <Grid container spacing={3}>
                <Grid item xs={12}>
                  <Typography variant="h6" gutterBottom>Description</Typography>
                  <Typography variant="body2">{selectedRule.description}</Typography>
                </Grid>

                <Grid item xs={6}>
                  <Typography variant="h6" gutterBottom>Framework</Typography>
                  <Chip label={selectedRule.framework} />
                </Grid>

                <Grid item xs={6}>
                  <Typography variant="h6" gutterBottom>Severity</Typography>
                  <Chip
                    label={selectedRule.severity.toUpperCase()}
                    color={getSeverityColor(selectedRule.severity)}
                  />
                </Grid>

                <Grid item xs={12}>
                  <Typography variant="h6" gutterBottom>Conditions</Typography>
                  {selectedRule.conditions.length > 0 ? (
                    selectedRule.conditions.map((condition, index) => (
                      <Accordion key={index}>
                        <AccordionSummary expandIcon={<ExpandMore />}>
                          <Typography>
                            {condition.field} {condition.operator} {String(condition.value)}
                          </Typography>
                        </AccordionSummary>
                        <AccordionDetails>
                          <Typography variant="body2">
                            Field: {condition.field}<br />
                            Operator: {condition.operator}<br />
                            Value: {String(condition.value)}
                          </Typography>
                        </AccordionDetails>
                      </Accordion>
                    ))
                  ) : (
                    <Typography variant="body2" color="text.secondary">
                      No conditions defined
                    </Typography>
                  )}
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