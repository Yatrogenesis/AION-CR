// AION-CR Conflict Resolution Page
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
  Alert,
  CircularProgress,
  Timeline,
  TimelineItem,
  TimelineSeparator,
  TimelineConnector,
  TimelineContent,
  TimelineDot,
  Paper,
  Accordion,
  AccordionSummary,
  AccordionDetails,
} from '@mui/material';
import {
  Warning,
  CheckCircle,
  Schedule,
  TrendingUp,
  Visibility,
  Edit,
  Gavel,
  AutoFixHigh,
  ExpandMore,
  Refresh,
  Add,
  Timeline as TimelineIcon,
  Assignment,
  Error,
  Info,
} from '@mui/icons-material';
import { ConflictResolution } from '../types';
import { systemApi } from '../services/api';

export default function Conflicts() {
  const [conflicts, setConflicts] = useState<ConflictResolution[]>([]);
  const [loading, setLoading] = useState(true);
  const [selectedConflict, setSelectedConflict] = useState<ConflictResolution | null>(null);
  const [detailDialogOpen, setDetailDialogOpen] = useState(false);
  const [resolveDialogOpen, setResolveDialogOpen] = useState(false);
  const [resolution, setResolution] = useState('');

  const [metrics, setMetrics] = useState({
    totalConflicts: 0,
    pendingConflicts: 0,
    resolvedToday: 0,
    avgResolutionTime: 0,
  });

  const loadConflicts = async () => {
    try {
      setLoading(true);
      const response = await systemApi.getConflicts();
      if (response.success) {
        const conflictsData = response.data?.items || [];
        setConflicts(conflictsData);

        // Calculate metrics
        const pending = conflictsData.filter(c => c.status === 'pending' || c.status === 'in_progress').length;
        const resolvedToday = conflictsData.filter(c =>
          c.resolvedAt && new Date(c.resolvedAt).toDateString() === new Date().toDateString()
        ).length;

        const resolvedConflicts = conflictsData.filter(c => c.resolvedAt);
        const avgTime = resolvedConflicts.length > 0
          ? resolvedConflicts.reduce((sum, c) => {
              const createdTime = new Date(c.createdAt).getTime();
              const resolvedTime = new Date(c.resolvedAt!).getTime();
              return sum + (resolvedTime - createdTime);
            }, 0) / resolvedConflicts.length / (1000 * 60 * 60) // Convert to hours
          : 0;

        setMetrics({
          totalConflicts: conflictsData.length,
          pendingConflicts: pending,
          resolvedToday,
          avgResolutionTime: avgTime,
        });
      }
    } catch (error) {
      console.error('Failed to load conflicts:', error);
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    loadConflicts();
  }, []);

  const handleResolveConflict = async () => {
    if (!selectedConflict || !resolution.trim()) return;

    try {
      await systemApi.resolveConflict(selectedConflict.id, resolution);
      setResolveDialogOpen(false);
      setResolution('');
      setSelectedConflict(null);
      loadConflicts();
    } catch (error) {
      console.error('Failed to resolve conflict:', error);
    }
  };

  const handleEscalateConflict = async (conflictId: string, reason: string) => {
    try {
      await systemApi.escalateConflict(conflictId, reason);
      loadConflicts();
    } catch (error) {
      console.error('Failed to escalate conflict:', error);
    }
  };

  const getStatusColor = (status: string) => {
    switch (status) {
      case 'resolved': return 'success';
      case 'in_progress': return 'info';
      case 'escalated': return 'warning';
      case 'pending': return 'default';
      default: return 'default';
    }
  };

  const getPriorityColor = (priority: string) => {
    switch (priority) {
      case 'urgent': return 'error';
      case 'high': return 'warning';
      case 'medium': return 'info';
      case 'low': return 'success';
      default: return 'default';
    }
  };

  const getTimelineIcon = (event: string) => {
    if (event.includes('created')) return <Add />;
    if (event.includes('resolved')) return <CheckCircle />;
    if (event.includes('escalated')) return <TrendingUp />;
    if (event.includes('assigned')) return <Assignment />;
    return <Info />;
  };

  const getTimelineColor = (event: string) => {
    if (event.includes('created')) return 'primary';
    if (event.includes('resolved')) return 'success';
    if (event.includes('escalated')) return 'warning';
    if (event.includes('error')) return 'error';
    return 'grey';
  };

  return (
    <Box>
      {/* Header */}
      <Box display="flex" justifyContent="space-between" alignItems="center" mb={3}>
        <Box>
          <Typography variant="h4" fontWeight="bold" gutterBottom>
            Conflict Resolution
          </Typography>
          <Typography variant="body1" color="text.secondary">
            Automated conflict detection and resolution management
          </Typography>
        </Box>

        <Box display="flex" gap={2}>
          <Button
            variant="outlined"
            startIcon={<Refresh />}
            onClick={loadConflicts}
            disabled={loading}
          >
            Refresh
          </Button>
          <Button
            variant="contained"
            startIcon={<AutoFixHigh />}
            color="warning"
          >
            Auto-Resolve
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
                    {metrics.totalConflicts}
                  </Typography>
                  <Typography variant="subtitle2" color="text.secondary">
                    Total Conflicts
                  </Typography>
                </Box>
                <Warning color="primary" />
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
                    {metrics.pendingConflicts}
                  </Typography>
                  <Typography variant="subtitle2" color="text.secondary">
                    Pending Resolution
                  </Typography>
                </Box>
                <Schedule color="warning" />
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
                    {metrics.resolvedToday}
                  </Typography>
                  <Typography variant="subtitle2" color="text.secondary">
                    Resolved Today
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
                    {metrics.avgResolutionTime.toFixed(1)}h
                  </Typography>
                  <Typography variant="subtitle2" color="text.secondary">
                    Avg Resolution Time
                  </Typography>
                </Box>
                <TrendingUp color="info" />
              </Box>
            </CardContent>
          </Card>
        </Grid>
      </Grid>

      {/* Conflicts Table */}
      <Card>
        <CardContent>
          <Typography variant="h6" fontWeight="600" gutterBottom>
            Active Conflicts
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
                    <TableCell>Conflict</TableCell>
                    <TableCell>Type</TableCell>
                    <TableCell>Priority</TableCell>
                    <TableCell>Status</TableCell>
                    <TableCell>Created</TableCell>
                    <TableCell>Involved Rules</TableCell>
                    <TableCell>Actions</TableCell>
                  </TableRow>
                </TableHead>
                <TableBody>
                  {conflicts.map((conflict) => (
                    <TableRow key={conflict.id}>
                      <TableCell>
                        <Box>
                          <Typography variant="subtitle2" fontWeight="600">
                            {conflict.title}
                          </Typography>
                          <Typography variant="caption" color="text.secondary">
                            {conflict.description}
                          </Typography>
                        </Box>
                      </TableCell>

                      <TableCell>
                        <Chip
                          label={conflict.conflictType.replace('_', ' ').toUpperCase()}
                          size="small"
                          variant="outlined"
                        />
                      </TableCell>

                      <TableCell>
                        <Chip
                          label={conflict.priority.toUpperCase()}
                          size="small"
                          color={getPriorityColor(conflict.priority)}
                        />
                      </TableCell>

                      <TableCell>
                        <Chip
                          label={conflict.status.replace('_', ' ').toUpperCase()}
                          size="small"
                          color={getStatusColor(conflict.status)}
                        />
                      </TableCell>

                      <TableCell>
                        <Typography variant="caption">
                          {new Date(conflict.createdAt).toLocaleString()}
                        </Typography>
                      </TableCell>

                      <TableCell>
                        <Typography variant="caption">
                          {conflict.involvedRules.length} rules
                        </Typography>
                      </TableCell>

                      <TableCell>
                        <Box display="flex" gap={1}>
                          <Tooltip title="View Details">
                            <IconButton
                              size="small"
                              onClick={() => {
                                setSelectedConflict(conflict);
                                setDetailDialogOpen(true);
                              }}
                            >
                              <Visibility />
                            </IconButton>
                          </Tooltip>

                          {(conflict.status === 'pending' || conflict.status === 'in_progress') && (
                            <Tooltip title="Resolve">
                              <IconButton
                                size="small"
                                color="success"
                                onClick={() => {
                                  setSelectedConflict(conflict);
                                  setResolveDialogOpen(true);
                                }}
                              >
                                <Gavel />
                              </IconButton>
                            </Tooltip>
                          )}

                          {conflict.status !== 'escalated' && conflict.status !== 'resolved' && (
                            <Tooltip title="Escalate">
                              <IconButton
                                size="small"
                                color="warning"
                                onClick={() => handleEscalateConflict(conflict.id, 'Manual escalation')}
                              >
                                <TrendingUp />
                              </IconButton>
                            </Tooltip>
                          )}
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

      {/* Conflict Detail Dialog */}
      <Dialog
        open={detailDialogOpen}
        onClose={() => setDetailDialogOpen(false)}
        maxWidth="md"
        fullWidth
      >
        <DialogTitle>
          Conflict Details: {selectedConflict?.title}
        </DialogTitle>
        <DialogContent>
          {selectedConflict && (
            <Box sx={{ pt: 2 }}>
              <Grid container spacing={3}>
                <Grid item xs={12}>
                  <Alert severity="info" sx={{ mb: 2 }}>
                    <Typography variant="body2">
                      <strong>Description:</strong> {selectedConflict.description}
                    </Typography>
                  </Alert>
                </Grid>

                <Grid item xs={6}>
                  <Typography variant="h6" gutterBottom>Conflict Type</Typography>
                  <Chip label={selectedConflict.conflictType.replace('_', ' ')} />
                </Grid>

                <Grid item xs={6}>
                  <Typography variant="h6" gutterBottom>Priority</Typography>
                  <Chip
                    label={selectedConflict.priority.toUpperCase()}
                    color={getPriorityColor(selectedConflict.priority)}
                  />
                </Grid>

                <Grid item xs={12}>
                  <Typography variant="h6" gutterBottom>Involved Rules</Typography>
                  <Box display="flex" flexWrap="wrap" gap={1}>
                    {selectedConflict.involvedRules.map((ruleId, index) => (
                      <Chip
                        key={index}
                        label={`Rule ${ruleId.slice(-8)}`}
                        size="small"
                        variant="outlined"
                      />
                    ))}
                  </Box>
                </Grid>

                {selectedConflict.resolution && (
                  <Grid item xs={12}>
                    <Typography variant="h6" gutterBottom>Resolution</Typography>
                    <Paper sx={{ p: 2, backgroundColor: 'success.light', color: 'success.contrastText' }}>
                      <Typography variant="body2">
                        {selectedConflict.resolution}
                      </Typography>
                      {selectedConflict.resolvedBy && (
                        <Typography variant="caption" display="block" sx={{ mt: 1 }}>
                          Resolved by: {selectedConflict.resolvedBy}
                        </Typography>
                      )}
                    </Paper>
                  </Grid>
                )}

                <Grid item xs={12}>
                  <Typography variant="h6" gutterBottom>Timeline</Typography>
                  <Timeline>
                    {selectedConflict.timeline.map((event, index) => (
                      <TimelineItem key={index}>
                        <TimelineSeparator>
                          <TimelineDot color={getTimelineColor(event.event) as any}>
                            {getTimelineIcon(event.event)}
                          </TimelineDot>
                          {index < selectedConflict.timeline.length - 1 && <TimelineConnector />}
                        </TimelineSeparator>
                        <TimelineContent>
                          <Typography variant="subtitle2" fontWeight="600">
                            {event.event}
                          </Typography>
                          <Typography variant="body2" color="text.secondary">
                            {event.description}
                          </Typography>
                          <Typography variant="caption" color="text.secondary">
                            {new Date(event.timestamp).toLocaleString()}
                            {event.user && ` • ${event.user}`}
                            {event.automated && ' • Automated'}
                          </Typography>
                        </TimelineContent>
                      </TimelineItem>
                    ))}
                  </Timeline>
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

      {/* Resolve Conflict Dialog */}
      <Dialog
        open={resolveDialogOpen}
        onClose={() => setResolveDialogOpen(false)}
        maxWidth="sm"
        fullWidth
      >
        <DialogTitle>
          Resolve Conflict: {selectedConflict?.title}
        </DialogTitle>
        <DialogContent>
          <Box sx={{ pt: 2 }}>
            <Alert severity="warning" sx={{ mb: 3 }}>
              Please provide a detailed resolution explanation for audit purposes.
            </Alert>

            <TextField
              fullWidth
              label="Resolution Details"
              multiline
              rows={4}
              value={resolution}
              onChange={(e) => setResolution(e.target.value)}
              placeholder="Describe how this conflict was resolved..."
            />
          </Box>
        </DialogContent>
        <DialogActions>
          <Button onClick={() => setResolveDialogOpen(false)}>
            Cancel
          </Button>
          <Button
            onClick={handleResolveConflict}
            variant="contained"
            color="success"
            disabled={!resolution.trim()}
          >
            Resolve Conflict
          </Button>
        </DialogActions>
      </Dialog>
    </Box>
  );
}