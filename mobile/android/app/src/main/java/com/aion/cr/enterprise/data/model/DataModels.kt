package com.aion.cr.enterprise.data.model

import android.os.Parcelable
import androidx.room.Entity
import androidx.room.PrimaryKey
import com.google.gson.annotations.SerializedName
import kotlinx.parcelize.Parcelize
import java.time.Instant
import java.util.*

// Authentication Models
@Parcelize
data class LoginRequest(
    val email: String,
    val password: String,
    val deviceId: String,
    val biometricEnabled: Boolean = false
) : Parcelable

@Parcelize
data class AuthResponse(
    val accessToken: String,
    val refreshToken: String,
    val expiresIn: Long,
    val tokenType: String = "Bearer",
    val user: User,
    val permissions: List<String>,
    val quantumKeyId: String
) : Parcelable

@Parcelize
data class RefreshTokenRequest(
    val refreshToken: String,
    val deviceId: String
) : Parcelable

@Parcelize
data class User(
    val id: String,
    val email: String,
    val firstName: String,
    val lastName: String,
    val role: String,
    val department: String,
    val organization: Organization,
    val lastLogin: String?,
    val profileImageUrl: String?
) : Parcelable

@Parcelize
data class Organization(
    val id: String,
    val name: String,
    val tier: String,
    val jurisdiction: String,
    val complianceFrameworks: List<String>
) : Parcelable

// Biometric Authentication Models
@Parcelize
data class BiometricRegistrationRequest(
    val biometricType: String,
    val encryptedBiometricData: String,
    val deviceInfo: DeviceInfo
) : Parcelable

@Parcelize
data class BiometricResponse(
    val success: Boolean,
    val biometricId: String?,
    val message: String
) : Parcelable

@Parcelize
data class DeviceInfo(
    val deviceId: String,
    val deviceName: String,
    val osVersion: String,
    val appVersion: String,
    val securityLevel: String
) : Parcelable

// Compliance Models
@Parcelize
data class ComplianceDashboard(
    val overallScore: Int,
    val riskLevel: String,
    val activeAlerts: Int,
    val pendingAssessments: Int,
    val recentActivities: List<ComplianceActivity>,
    val complianceMetrics: ComplianceMetrics,
    val trendData: List<TrendDataPoint>
) : Parcelable

@Parcelize
data class ComplianceMetrics(
    val totalAssessments: Int,
    val passedAssessments: Int,
    val failedAssessments: Int,
    val pendingAssessments: Int,
    val averageScore: Double,
    val improvementPercentage: Double
) : Parcelable

@Parcelize
data class TrendDataPoint(
    val timestamp: String,
    val score: Int,
    val riskLevel: String
) : Parcelable

@Entity(tableName = "compliance_assessments")
@Parcelize
data class ComplianceAssessment(
    @PrimaryKey val id: String,
    val title: String,
    val description: String,
    val framework: String,
    val status: AssessmentStatus,
    val score: Int?,
    val riskLevel: RiskLevel,
    val assignedTo: String,
    val dueDate: String,
    val createdAt: String,
    val updatedAt: String,
    val requirements: List<ComplianceRequirement>,
    val evidence: List<Evidence>,
    val tags: List<String>
) : Parcelable

@Parcelize
data class ComplianceRequirement(
    val id: String,
    val title: String,
    val description: String,
    val mandatory: Boolean,
    val status: RequirementStatus,
    val evidence: List<Evidence>,
    val notes: String?
) : Parcelable

@Parcelize
data class Evidence(
    val id: String,
    val type: EvidenceType,
    val title: String,
    val description: String,
    val fileUrl: String?,
    val uploadedBy: String,
    val uploadedAt: String,
    val verified: Boolean
) : Parcelable

@Parcelize
data class ApprovalRequest(
    val approved: Boolean,
    val comments: String,
    val conditions: List<String> = emptyList()
) : Parcelable

@Parcelize
data class ComplianceActivity(
    val id: String,
    val type: String,
    val title: String,
    val description: String,
    val timestamp: String,
    val user: String,
    val severity: String
) : Parcelable

// Risk Management Models
@Entity(tableName = "risk_alerts")
@Parcelize
data class RiskAlert(
    @PrimaryKey val id: String,
    val title: String,
    val description: String,
    val severity: RiskSeverity,
    val category: String,
    val source: String,
    val status: AlertStatus,
    val createdAt: String,
    val updatedAt: String,
    val dueDate: String?,
    val assignedTo: String?,
    val impactAreas: List<String>,
    val recommendedActions: List<String>,
    val acknowledged: Boolean = false,
    val acknowledgedBy: String? = null,
    val acknowledgedAt: String? = null
) : Parcelable

@Parcelize
data class RiskAssessment(
    val id: String,
    val title: String,
    val riskType: String,
    val likelihood: Int,
    val impact: Int,
    val riskScore: Int,
    val mitigationPlan: String,
    val owner: String,
    val status: String,
    val lastReviewed: String
) : Parcelable

@Parcelize
data class AlertAcknowledgment(
    val notes: String,
    val plannedActions: List<String>,
    val escalateToManagement: Boolean = false
) : Parcelable

// Regulatory Updates Models
@Entity(tableName = "regulatory_updates")
@Parcelize
data class RegulatoryUpdate(
    @PrimaryKey val id: String,
    val title: String,
    val description: String,
    val jurisdiction: String,
    val framework: String,
    val impactLevel: ImpactLevel,
    val effectiveDate: String,
    val publishedDate: String,
    val source: String,
    val sourceUrl: String?,
    val categories: List<String>,
    val impactedAreas: List<String>,
    val status: UpdateStatus,
    val reviewedBy: String?,
    val reviewedAt: String?,
    val implementationDeadline: String?
) : Parcelable

@Parcelize
data class RegulatoryChange(
    val id: String,
    val title: String,
    val changeType: String,
    val description: String,
    val beforeText: String?,
    val afterText: String?,
    val impactAnalysis: String,
    val actionRequired: Boolean,
    val estimatedEffort: String?
) : Parcelable

@Parcelize
data class RegulatoryReview(
    val reviewStatus: String,
    val comments: String,
    val actionItems: List<String>,
    val assignTo: String?
) : Parcelable

// Document Management Models
@Entity(tableName = "compliance_documents")
@Parcelize
data class ComplianceDocument(
    @PrimaryKey val id: String,
    val title: String,
    val description: String,
    val category: String,
    val type: DocumentType,
    val version: String,
    val status: DocumentStatus,
    val createdBy: String,
    val createdAt: String,
    val updatedAt: String,
    val fileSize: Long,
    val mimeType: String,
    val downloadUrl: String,
    val tags: List<String>,
    val approvals: List<DocumentApproval>,
    val retentionPeriod: String?,
    val confidentialityLevel: String
) : Parcelable

@Parcelize
data class DocumentApproval(
    val id: String,
    val approver: String,
    val approved: Boolean,
    val comments: String,
    val timestamp: String,
    val digitalSignature: String?
) : Parcelable

// Audit Trail Models
@Parcelize
data class AuditEntry(
    val id: String,
    val entityType: String,
    val entityId: String,
    val action: String,
    val timestamp: String,
    val userId: String,
    val userEmail: String,
    val ipAddress: String,
    val userAgent: String,
    val details: Map<String, Any>,
    val quantumSignature: String
) : Parcelable

@Parcelize
data class ComplianceScore(
    val currentScore: Int,
    val previousScore: Int,
    val trend: String,
    val breakdown: Map<String, Int>,
    val benchmarkScore: Int,
    val industryAverage: Int,
    val lastCalculated: String
) : Parcelable

// Notification Models
@Entity(tableName = "notifications")
@Parcelize
data class Notification(
    @PrimaryKey val id: String,
    val title: String,
    val message: String,
    val type: NotificationType,
    val priority: NotificationPriority,
    val read: Boolean = false,
    val createdAt: String,
    val actionUrl: String?,
    val metadata: Map<String, String> = emptyMap()
) : Parcelable

// Monitoring Models
@Parcelize
data class MonitoringStatus(
    val systemHealth: String,
    val activeMonitors: Int,
    val lastUpdate: String,
    val alertsCount: Int,
    val systemLoad: Double,
    val quantumEncryptionStatus: String
) : Parcelable

@Parcelize
data class MonitoringMetrics(
    val timestamp: String,
    val cpuUsage: Double,
    val memoryUsage: Double,
    val diskUsage: Double,
    val networkLatency: Double,
    val apiResponseTime: Double,
    val activeUsers: Int,
    val quantumOperations: Int
) : Parcelable

// Quantum Security Models
@Parcelize
data class QuantumKeyExchangeRequest(
    val algorithm: String,
    val publicKey: String,
    val nonce: String
) : Parcelable

@Parcelize
data class QuantumKeyExchangeResponse(
    val keyId: String,
    val publicKey: String,
    val encryptedSharedSecret: String,
    val algorithm: String,
    val expiresAt: String
) : Parcelable

@Parcelize
data class QuantumEncryptionRequest(
    val data: String,
    val keyId: String,
    val algorithm: String = "CRYSTALS-Kyber"
) : Parcelable

@Parcelize
data class QuantumEncryptionResponse(
    val encryptedData: String,
    val encryptionMetadata: EncryptionMetadata
) : Parcelable

@Parcelize
data class QuantumDecryptionRequest(
    val encryptedData: String,
    val keyId: String,
    val encryptionMetadata: EncryptionMetadata
) : Parcelable

@Parcelize
data class QuantumDecryptionResponse(
    val decryptedData: String,
    val verified: Boolean
) : Parcelable

@Parcelize
data class EncryptionMetadata(
    val algorithm: String,
    val keyVersion: String,
    val timestamp: String,
    val nonce: String,
    val signature: String
) : Parcelable

// Settings Models
@Parcelize
data class UserSettings(
    val userId: String,
    val theme: String = "system",
    val notifications: NotificationSettings,
    val security: SecuritySettings,
    val privacy: PrivacySettings,
    val language: String = "en",
    val timezone: String,
    val biometricEnabled: Boolean = false
) : Parcelable

@Parcelize
data class NotificationSettings(
    val pushEnabled: Boolean = true,
    val emailEnabled: Boolean = true,
    val alertsEnabled: Boolean = true,
    val updatesEnabled: Boolean = true,
    val quietHours: QuietHours?
) : Parcelable

@Parcelize
data class SecuritySettings(
    val autoLockEnabled: Boolean = true,
    val autoLockTimeoutMinutes: Int = 5,
    val biometricRequired: Boolean = false,
    val quantumEncryptionEnabled: Boolean = true,
    val sessionTimeoutMinutes: Int = 30
) : Parcelable

@Parcelize
data class PrivacySettings(
    val analyticsEnabled: Boolean = true,
    val crashReportingEnabled: Boolean = true,
    val performanceMonitoringEnabled: Boolean = true
) : Parcelable

@Parcelize
data class QuietHours(
    val enabled: Boolean = false,
    val startTime: String = "22:00",
    val endTime: String = "08:00"
) : Parcelable

@Parcelize
data class OrganizationSettings(
    val organizationId: String,
    val name: String,
    val complianceFrameworks: List<String>,
    val defaultRoles: List<String>,
    val securityPolicies: Map<String, Any>,
    val auditRetentionDays: Int,
    val quantumSecurityEnabled: Boolean
) : Parcelable

// Health Check Models
@Parcelize
data class HealthStatus(
    val status: String,
    val timestamp: String,
    val version: String
) : Parcelable

@Parcelize
data class DetailedHealthStatus(
    val status: String,
    val timestamp: String,
    val version: String,
    val services: Map<String, ServiceHealth>,
    val quantum: QuantumHealth,
    val database: DatabaseHealth,
    val cache: CacheHealth
) : Parcelable

@Parcelize
data class ServiceHealth(
    val status: String,
    val responseTime: Double,
    val lastCheck: String
) : Parcelable

@Parcelize
data class QuantumHealth(
    val status: String,
    val activeConnections: Int,
    val encryptionOpsPerSecond: Double,
    val keyRotationStatus: String
) : Parcelable

@Parcelize
data class DatabaseHealth(
    val status: String,
    val connectionPool: Int,
    val activeQueries: Int,
    val avgQueryTime: Double
) : Parcelable

@Parcelize
data class CacheHealth(
    val status: String,
    val hitRate: Double,
    val memoryUsage: Double
) : Parcelable

// Utility Models
@Parcelize
data class PagedResponse<T : Parcelable>(
    val content: List<T>,
    val totalElements: Long,
    val totalPages: Int,
    val currentPage: Int,
    val pageSize: Int,
    val hasNext: Boolean,
    val hasPrevious: Boolean
) : Parcelable

// Enums
enum class AssessmentStatus { DRAFT, IN_PROGRESS, PENDING_REVIEW, APPROVED, REJECTED, ARCHIVED }
enum class RequirementStatus { NOT_STARTED, IN_PROGRESS, COMPLETED, NOT_APPLICABLE, FAILED }
enum class RiskLevel { LOW, MEDIUM, HIGH, CRITICAL }
enum class RiskSeverity { LOW, MEDIUM, HIGH, CRITICAL }
enum class AlertStatus { OPEN, IN_PROGRESS, RESOLVED, DISMISSED }
enum class ImpactLevel { LOW, MEDIUM, HIGH, CRITICAL }
enum class UpdateStatus { NEW, UNDER_REVIEW, IMPLEMENTED, IGNORED }
enum class DocumentType { POLICY, PROCEDURE, EVIDENCE, REPORT, CERTIFICATION }
enum class DocumentStatus { DRAFT, UNDER_REVIEW, APPROVED, ARCHIVED, EXPIRED }
enum class EvidenceType { DOCUMENT, SCREENSHOT, REPORT, CERTIFICATION, AUDIT_LOG }
enum class NotificationType { INFO, WARNING, ERROR, SUCCESS, REGULATORY_UPDATE }
enum class NotificationPriority { LOW, NORMAL, HIGH, URGENT }