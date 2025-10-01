// AION-CR Web UI Types
// Maximum Autonomy Level 255 - Complete System Integration

export interface User {
  id: string;
  username: string;
  email: string;
  role: 'admin' | 'operator' | 'analyst' | 'viewer';
  permissions: string[];
  lastLogin?: Date;
  preferences: UserPreferences;
}

export interface UserPreferences {
  theme: 'light' | 'dark';
  language: string;
  notifications: boolean;
  autoRefresh: boolean;
  dashboardLayout: string;
}

export interface SystemStatus {
  id: string;
  timestamp: Date;
  status: 'online' | 'degraded' | 'offline' | 'maintenance';
  uptime: number;
  version: string;
  health: {
    cpu: number;
    memory: number;
    disk: number;
    network: number;
  };
  services: ServiceStatus[];
  alerts: SystemAlert[];
  autonomyLevel: number; // 0-255
  aiModules: AIModuleStatus[];
  blockchainConnections: BlockchainStatus[];
}

export interface ServiceStatus {
  name: string;
  status: 'running' | 'stopped' | 'error' | 'starting';
  port?: number;
  endpoint?: string;
  lastCheck: Date;
  responseTime?: number;
  errorMessage?: string;
}

export interface SystemAlert {
  id: string;
  title: string;
  message: string;
  severity: 'low' | 'medium' | 'high' | 'critical';
  timestamp: Date;
  source: string;
  resolved: boolean;
  assignedTo?: string;
}

export interface AIModuleStatus {
  module: string;
  status: 'active' | 'inactive' | 'training' | 'error';
  performance: number;
  accuracy?: number;
  lastUpdate: Date;
  model: string;
  version: string;
}

export interface BlockchainStatus {
  network: string;
  connected: boolean;
  blockHeight: number;
  peers: number;
  gasPrice?: string;
  lastSync: Date;
}

export interface ComplianceRule {
  id: string;
  name: string;
  description: string;
  framework: string;
  category: string;
  severity: 'low' | 'medium' | 'high' | 'critical';
  enabled: boolean;
  conditions: RuleCondition[];
  actions: RuleAction[];
  createdAt: Date;
  updatedAt: Date;
  createdBy: string;
}

export interface RuleCondition {
  field: string;
  operator: 'equals' | 'not_equals' | 'contains' | 'not_contains' | 'greater_than' | 'less_than' | 'regex';
  value: string | number | boolean;
  logicalOperator?: 'AND' | 'OR';
}

export interface RuleAction {
  type: 'alert' | 'block' | 'quarantine' | 'log' | 'escalate';
  parameters: Record<string, any>;
}

export interface ConflictResolution {
  id: string;
  title: string;
  description: string;
  status: 'pending' | 'in_progress' | 'resolved' | 'escalated';
  priority: 'low' | 'medium' | 'high' | 'urgent';
  conflictType: string;
  involvedRules: string[];
  resolution?: string;
  resolvedBy?: string;
  createdAt: Date;
  resolvedAt?: Date;
  timeline: ConflictEvent[];
}

export interface ConflictEvent {
  timestamp: Date;
  event: string;
  description: string;
  user?: string;
  automated: boolean;
}

export interface AutonomousAgent {
  id: string;
  name: string;
  type: 'compliance' | 'monitoring' | 'analysis' | 'optimization' | 'security';
  status: 'active' | 'inactive' | 'training' | 'error';
  autonomyLevel: number; // 0-255
  capabilities: string[];
  lastAction: Date;
  performance: AgentPerformance;
  configuration: AgentConfiguration;
}

export interface AgentPerformance {
  tasksCompleted: number;
  successRate: number;
  averageResponseTime: number;
  accuracy: number;
  uptime: number;
}

export interface AgentConfiguration {
  maxAutonomy: number;
  allowedActions: string[];
  escalationRules: string[];
  learningEnabled: boolean;
  parameters: Record<string, any>;
}

export interface MLModel {
  id: string;
  name: string;
  type: 'classification' | 'regression' | 'clustering' | 'anomaly_detection' | 'nlp' | 'computer_vision';
  status: 'training' | 'deployed' | 'testing' | 'archived';
  version: string;
  accuracy: number;
  precision?: number;
  recall?: number;
  f1Score?: number;
  trainingData: string;
  lastTrained: Date;
  deployedAt?: Date;
  framework: string;
  parameters: Record<string, any>;
}

export interface AuditLog {
  id: string;
  timestamp: Date;
  user: string;
  action: string;
  resource: string;
  details: Record<string, any>;
  ipAddress: string;
  userAgent: string;
  outcome: 'success' | 'failure' | 'warning';
  blockchainHash?: string;
}

export interface Notification {
  id: string;
  type: 'info' | 'success' | 'warning' | 'error';
  title: string;
  message: string;
  timestamp: Date;
  autoHide: boolean;
  read?: boolean;
  actionUrl?: string;
}

export interface DashboardWidget {
  id: string;
  type: 'chart' | 'metric' | 'table' | 'alert' | 'status';
  title: string;
  size: 'small' | 'medium' | 'large';
  position: { x: number; y: number; w: number; h: number };
  config: Record<string, any>;
  dataSource: string;
  refreshInterval: number;
}

export interface ApiResponse<T> {
  success: boolean;
  data?: T;
  error?: string;
  message?: string;
  timestamp: Date;
}

export interface ChartData {
  labels: string[];
  datasets: ChartDataset[];
}

export interface ChartDataset {
  label: string;
  data: number[];
  backgroundColor?: string | string[];
  borderColor?: string | string[];
  borderWidth?: number;
  fill?: boolean;
}

export interface FilterOptions {
  dateRange?: {
    start: Date;
    end: Date;
  };
  status?: string[];
  category?: string[];
  severity?: string[];
  user?: string[];
  limit?: number;
  offset?: number;
  sortBy?: string;
  sortOrder?: 'asc' | 'desc';
}

export interface PaginationInfo {
  page: number;
  limit: number;
  total: number;
  pages: number;
  hasNext: boolean;
  hasPrev: boolean;
}

export interface SearchResult<T> {
  items: T[];
  pagination: PaginationInfo;
  filters: FilterOptions;
  aggregations?: Record<string, any>;
}

// Quantum Computing Types
export interface QuantumJob {
  id: string;
  name: string;
  type: 'optimization' | 'cryptography' | 'simulation' | 'machine_learning';
  status: 'queued' | 'running' | 'completed' | 'failed' | 'cancelled';
  qubits: number;
  gates: number;
  shots: number;
  backend: string;
  submittedAt: Date;
  completedAt?: Date;
  results?: QuantumResults;
  error?: string;
}

export interface QuantumResults {
  counts: Record<string, number>;
  executionTime: number;
  fidelity?: number;
  errorRate?: number;
  metadata: Record<string, any>;
}

// Blockchain Types
export interface SmartContract {
  id: string;
  name: string;
  address: string;
  network: string;
  abi: any[];
  bytecode: string;
  status: 'deployed' | 'pending' | 'failed';
  gasUsed: number;
  transactionHash: string;
  deployedAt: Date;
  interactions: number;
}

export interface BlockchainTransaction {
  hash: string;
  from: string;
  to: string;
  value: string;
  gasUsed: number;
  gasPrice: string;
  blockNumber: number;
  timestamp: Date;
  status: 'pending' | 'confirmed' | 'failed';
  type: 'audit' | 'compliance' | 'smart_contract' | 'transfer';
}

export interface ComplianceRecord {
  id: string;
  recordType: string;
  data: Record<string, any>;
  hash: string;
  blockchainHash?: string;
  timestamp: Date;
  verified: boolean;
  signatures: string[];
  metadata: Record<string, any>;
}