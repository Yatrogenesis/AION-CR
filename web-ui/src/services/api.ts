// AION-CR API Service
// Maximum Autonomy Level 255 - Complete System Integration

import axios, { AxiosInstance, AxiosRequestConfig, AxiosResponse } from 'axios';
import {
  ApiResponse,
  SystemStatus,
  User,
  ComplianceRule,
  ConflictResolution,
  AutonomousAgent,
  MLModel,
  AuditLog,
  FilterOptions,
  SearchResult,
  QuantumJob,
  SmartContract,
  BlockchainTransaction,
  ComplianceRecord,
} from '../types';

class ApiService {
  private api: AxiosInstance;
  private baseURL: string;

  constructor() {
    this.baseURL = process.env.REACT_APP_API_URL || 'http://localhost:8080/api/v1';

    this.api = axios.create({
      baseURL: this.baseURL,
      timeout: 30000,
      headers: {
        'Content-Type': 'application/json',
        'X-Client': 'AION-CR-WebUI',
        'X-Autonomy-Level': '255',
      },
    });

    this.setupInterceptors();
  }

  private setupInterceptors() {
    // Request interceptor
    this.api.interceptors.request.use(
      (config) => {
        const token = localStorage.getItem('aion_token');
        if (token) {
          config.headers.Authorization = `Bearer ${token}`;
        }

        // Add request timestamp
        config.headers['X-Request-Time'] = new Date().toISOString();

        return config;
      },
      (error) => {
        return Promise.reject(error);
      }
    );

    // Response interceptor
    this.api.interceptors.response.use(
      (response: AxiosResponse) => {
        return response;
      },
      (error) => {
        if (error.response?.status === 401) {
          // Handle unauthorized access
          localStorage.removeItem('aion_token');
          window.location.href = '/login';
        }
        return Promise.reject(error);
      }
    );
  }

  // Generic API methods
  private async request<T>(config: AxiosRequestConfig): Promise<ApiResponse<T>> {
    try {
      const response = await this.api.request<ApiResponse<T>>(config);
      return response.data;
    } catch (error: any) {
      console.error('API Request failed:', error);
      throw new Error(error.response?.data?.message || error.message || 'API request failed');
    }
  }

  private async get<T>(url: string, params?: any): Promise<ApiResponse<T>> {
    return this.request<T>({ method: 'GET', url, params });
  }

  private async post<T>(url: string, data?: any): Promise<ApiResponse<T>> {
    return this.request<T>({ method: 'POST', url, data });
  }

  private async put<T>(url: string, data?: any): Promise<ApiResponse<T>> {
    return this.request<T>({ method: 'PUT', url, data });
  }

  private async delete<T>(url: string): Promise<ApiResponse<T>> {
    return this.request<T>({ method: 'DELETE', url });
  }

  // Authentication
  async login(credentials: { username: string; password: string }) {
    const response = await this.post<{ token: string; user: User }>('/auth/login', credentials);
    if (response.success && response.data?.token) {
      localStorage.setItem('aion_token', response.data.token);
    }
    return response;
  }

  async logout() {
    await this.post('/auth/logout');
    localStorage.removeItem('aion_token');
  }

  async getCurrentUser(): Promise<User> {
    const response = await this.get<User>('/auth/me');
    if (!response.success || !response.data) {
      throw new Error('Failed to get current user');
    }
    return response.data;
  }

  async refreshToken() {
    const response = await this.post<{ token: string }>('/auth/refresh');
    if (response.success && response.data?.token) {
      localStorage.setItem('aion_token', response.data.token);
    }
    return response;
  }

  // System Status
  async getSystemStatus(): Promise<SystemStatus> {
    const response = await this.get<SystemStatus>('/system/status');
    if (!response.success || !response.data) {
      throw new Error('Failed to get system status');
    }
    return response.data;
  }

  async getSystemHealth() {
    return this.get('/system/health');
  }

  async getSystemMetrics(timeRange?: string) {
    return this.get('/system/metrics', { timeRange });
  }

  // Compliance Rules
  async getComplianceRules(filters?: FilterOptions): Promise<SearchResult<ComplianceRule>> {
    const response = await this.get<SearchResult<ComplianceRule>>('/compliance/rules', filters);
    if (!response.success || !response.data) {
      throw new Error('Failed to get compliance rules');
    }
    return response.data;
  }

  async getComplianceRule(id: string): Promise<ComplianceRule> {
    const response = await this.get<ComplianceRule>(`/compliance/rules/${id}`);
    if (!response.success || !response.data) {
      throw new Error('Failed to get compliance rule');
    }
    return response.data;
  }

  async createComplianceRule(rule: Partial<ComplianceRule>) {
    return this.post('/compliance/rules', rule);
  }

  async updateComplianceRule(id: string, rule: Partial<ComplianceRule>) {
    return this.put(`/compliance/rules/${id}`, rule);
  }

  async deleteComplianceRule(id: string) {
    return this.delete(`/compliance/rules/${id}`);
  }

  async validateCompliance(data: any) {
    return this.post('/compliance/validate', data);
  }

  // Conflict Resolution
  async getConflicts(filters?: FilterOptions): Promise<SearchResult<ConflictResolution>> {
    const response = await this.get<SearchResult<ConflictResolution>>('/conflicts', filters);
    if (!response.success || !response.data) {
      throw new Error('Failed to get conflicts');
    }
    return response.data;
  }

  async getConflict(id: string): Promise<ConflictResolution> {
    const response = await this.get<ConflictResolution>(`/conflicts/${id}`);
    if (!response.success || !response.data) {
      throw new Error('Failed to get conflict');
    }
    return response.data;
  }

  async resolveConflict(id: string, resolution: string) {
    return this.post(`/conflicts/${id}/resolve`, { resolution });
  }

  async escalateConflict(id: string, reason: string) {
    return this.post(`/conflicts/${id}/escalate`, { reason });
  }

  // Autonomous Agents
  async getAgents(filters?: FilterOptions): Promise<SearchResult<AutonomousAgent>> {
    const response = await this.get<SearchResult<AutonomousAgent>>('/agents', filters);
    if (!response.success || !response.data) {
      throw new Error('Failed to get agents');
    }
    return response.data;
  }

  async getAgent(id: string): Promise<AutonomousAgent> {
    const response = await this.get<AutonomousAgent>(`/agents/${id}`);
    if (!response.success || !response.data) {
      throw new Error('Failed to get agent');
    }
    return response.data;
  }

  async createAgent(agent: Partial<AutonomousAgent>) {
    return this.post('/agents', agent);
  }

  async updateAgent(id: string, agent: Partial<AutonomousAgent>) {
    return this.put(`/agents/${id}`, agent);
  }

  async deleteAgent(id: string) {
    return this.delete(`/agents/${id}`);
  }

  async startAgent(id: string) {
    return this.post(`/agents/${id}/start`);
  }

  async stopAgent(id: string) {
    return this.post(`/agents/${id}/stop`);
  }

  async trainAgent(id: string, trainingData: any) {
    return this.post(`/agents/${id}/train`, trainingData);
  }

  // Machine Learning
  async getMLModels(filters?: FilterOptions): Promise<SearchResult<MLModel>> {
    const response = await this.get<SearchResult<MLModel>>('/ml/models', filters);
    if (!response.success || !response.data) {
      throw new Error('Failed to get ML models');
    }
    return response.data;
  }

  async getMLModel(id: string): Promise<MLModel> {
    const response = await this.get<MLModel>(`/ml/models/${id}`);
    if (!response.success || !response.data) {
      throw new Error('Failed to get ML model');
    }
    return response.data;
  }

  async trainMLModel(id: string, config: any) {
    return this.post(`/ml/models/${id}/train`, config);
  }

  async deployMLModel(id: string) {
    return this.post(`/ml/models/${id}/deploy`);
  }

  async predictWithModel(id: string, data: any) {
    return this.post(`/ml/models/${id}/predict`, data);
  }

  async getModelPerformance(id: string) {
    return this.get(`/ml/models/${id}/performance`);
  }

  // Audit Logs
  async getAuditLogs(filters?: FilterOptions): Promise<SearchResult<AuditLog>> {
    const response = await this.get<SearchResult<AuditLog>>('/audit/logs', filters);
    if (!response.success || !response.data) {
      throw new Error('Failed to get audit logs');
    }
    return response.data;
  }

  async getAuditLog(id: string): Promise<AuditLog> {
    const response = await this.get<AuditLog>(`/audit/logs/${id}`);
    if (!response.success || !response.data) {
      throw new Error('Failed to get audit log');
    }
    return response.data;
  }

  async exportAuditLogs(filters?: FilterOptions, format: 'json' | 'csv' | 'pdf' = 'json') {
    return this.get('/audit/export', { ...filters, format });
  }

  // Quantum Computing
  async getQuantumJobs(filters?: FilterOptions): Promise<SearchResult<QuantumJob>> {
    const response = await this.get<SearchResult<QuantumJob>>('/quantum/jobs', filters);
    if (!response.success || !response.data) {
      throw new Error('Failed to get quantum jobs');
    }
    return response.data;
  }

  async submitQuantumJob(job: Partial<QuantumJob>) {
    return this.post('/quantum/jobs', job);
  }

  async getQuantumJob(id: string): Promise<QuantumJob> {
    const response = await this.get<QuantumJob>(`/quantum/jobs/${id}`);
    if (!response.success || !response.data) {
      throw new Error('Failed to get quantum job');
    }
    return response.data;
  }

  async cancelQuantumJob(id: string) {
    return this.delete(`/quantum/jobs/${id}`);
  }

  // Blockchain
  async getSmartContracts(): Promise<SmartContract[]> {
    const response = await this.get<SmartContract[]>('/blockchain/contracts');
    if (!response.success || !response.data) {
      throw new Error('Failed to get smart contracts');
    }
    return response.data;
  }

  async deploySmartContract(contract: Partial<SmartContract>) {
    return this.post('/blockchain/contracts', contract);
  }

  async getBlockchainTransactions(filters?: FilterOptions): Promise<SearchResult<BlockchainTransaction>> {
    const response = await this.get<SearchResult<BlockchainTransaction>>('/blockchain/transactions', filters);
    if (!response.success || !response.data) {
      throw new Error('Failed to get blockchain transactions');
    }
    return response.data;
  }

  async createComplianceRecord(record: Partial<ComplianceRecord>) {
    return this.post('/blockchain/compliance-records', record);
  }

  async getComplianceRecords(filters?: FilterOptions): Promise<SearchResult<ComplianceRecord>> {
    const response = await this.get<SearchResult<ComplianceRecord>>('/blockchain/compliance-records', filters);
    if (!response.success || !response.data) {
      throw new Error('Failed to get compliance records');
    }
    return response.data;
  }

  async verifyComplianceRecord(id: string) {
    return this.post(`/blockchain/compliance-records/${id}/verify`);
  }

  // Analytics and Reports
  async getAnalytics(type: string, timeRange?: string) {
    return this.get(`/analytics/${type}`, { timeRange });
  }

  async generateReport(type: string, config: any) {
    return this.post(`/reports/${type}`, config);
  }

  async getReports(filters?: FilterOptions) {
    return this.get('/reports', filters);
  }

  async downloadReport(id: string, format: 'pdf' | 'csv' | 'xlsx' = 'pdf') {
    return this.get(`/reports/${id}/download`, { format });
  }

  // Real-time Data
  async subscribeToUpdates(callback: (data: any) => void) {
    // WebSocket implementation for real-time updates
    const wsUrl = this.baseURL.replace('http', 'ws') + '/ws';
    const ws = new WebSocket(wsUrl);

    ws.onmessage = (event) => {
      try {
        const data = JSON.parse(event.data);
        callback(data);
      } catch (error) {
        console.error('Failed to parse WebSocket message:', error);
      }
    };

    ws.onerror = (error) => {
      console.error('WebSocket error:', error);
    };

    return ws;
  }

  // File Upload
  async uploadFile(file: File, category?: string) {
    const formData = new FormData();
    formData.append('file', file);
    if (category) {
      formData.append('category', category);
    }

    return this.api.post('/files/upload', formData, {
      headers: {
        'Content-Type': 'multipart/form-data',
      },
    });
  }

  // System Configuration
  async getConfiguration() {
    return this.get('/config');
  }

  async updateConfiguration(config: any) {
    return this.put('/config', config);
  }

  // Monitoring
  async getAlerts(filters?: FilterOptions) {
    return this.get('/monitoring/alerts', filters);
  }

  async acknowledgeAlert(id: string) {
    return this.post(`/monitoring/alerts/${id}/acknowledge`);
  }

  async resolveAlert(id: string, resolution: string) {
    return this.post(`/monitoring/alerts/${id}/resolve`, { resolution });
  }
}

// Create singleton instance
export const systemApi = new ApiService();

// Export individual service methods for easier testing
export const {
  login,
  logout,
  getCurrentUser,
  getSystemStatus,
  getComplianceRules,
  getConflicts,
  getAgents,
  getMLModels,
  getAuditLogs,
} = systemApi;