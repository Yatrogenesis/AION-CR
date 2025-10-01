package com.aion.cr.enterprise.data.api

import com.aion.cr.enterprise.data.model.*
import retrofit2.Response
import retrofit2.http.*

/**
 * AION-CR Enterprise API Service
 *
 * Provides secure communication with AION-CR backend services
 * using quantum-resistant encryption and enterprise authentication.
 */
interface AionApiService {

    // Authentication Endpoints
    @POST("auth/login")
    suspend fun login(@Body loginRequest: LoginRequest): Response<AuthResponse>

    @POST("auth/refresh")
    suspend fun refreshToken(@Body refreshRequest: RefreshTokenRequest): Response<AuthResponse>

    @POST("auth/logout")
    suspend fun logout(@Header("Authorization") token: String): Response<Unit>

    @POST("auth/biometric/register")
    suspend fun registerBiometric(
        @Header("Authorization") token: String,
        @Body biometricData: BiometricRegistrationRequest
    ): Response<BiometricResponse>

    // Compliance Monitoring Endpoints
    @GET("compliance/dashboard")
    suspend fun getComplianceDashboard(
        @Header("Authorization") token: String,
        @Query("timeframe") timeframe: String = "24h"
    ): Response<ComplianceDashboard>

    @GET("compliance/assessments")
    suspend fun getComplianceAssessments(
        @Header("Authorization") token: String,
        @Query("page") page: Int = 0,
        @Query("size") size: Int = 20,
        @Query("status") status: String? = null
    ): Response<PagedResponse<ComplianceAssessment>>

    @GET("compliance/assessments/{id}")
    suspend fun getComplianceAssessment(
        @Header("Authorization") token: String,
        @Path("id") assessmentId: String
    ): Response<ComplianceAssessment>

    @POST("compliance/assessments/{id}/approve")
    suspend fun approveAssessment(
        @Header("Authorization") token: String,
        @Path("id") assessmentId: String,
        @Body approvalRequest: ApprovalRequest
    ): Response<ComplianceAssessment>

    // Risk Management Endpoints
    @GET("risk/alerts")
    suspend fun getRiskAlerts(
        @Header("Authorization") token: String,
        @Query("severity") severity: String? = null,
        @Query("limit") limit: Int = 50
    ): Response<List<RiskAlert>>

    @GET("risk/assessments")
    suspend fun getRiskAssessments(
        @Header("Authorization") token: String,
        @Query("department") department: String? = null
    ): Response<List<RiskAssessment>>

    @POST("risk/alerts/{id}/acknowledge")
    suspend fun acknowledgeRiskAlert(
        @Header("Authorization") token: String,
        @Path("id") alertId: String,
        @Body acknowledgment: AlertAcknowledgment
    ): Response<RiskAlert>

    // Regulatory Updates Endpoints
    @GET("regulatory/updates")
    suspend fun getRegulatoryUpdates(
        @Header("Authorization") token: String,
        @Query("jurisdiction") jurisdiction: String? = null,
        @Query("since") since: String? = null
    ): Response<List<RegulatoryUpdate>>

    @GET("regulatory/changes")
    suspend fun getRegulatoryChanges(
        @Header("Authorization") token: String,
        @Query("impact_level") impactLevel: String? = null
    ): Response<List<RegulatoryChange>>

    @POST("regulatory/updates/{id}/review")
    suspend fun reviewRegulatoryUpdate(
        @Header("Authorization") token: String,
        @Path("id") updateId: String,
        @Body review: RegulatoryReview
    ): Response<RegulatoryUpdate>

    // Document Management Endpoints
    @GET("documents")
    suspend fun getDocuments(
        @Header("Authorization") token: String,
        @Query("category") category: String? = null,
        @Query("search") search: String? = null
    ): Response<List<ComplianceDocument>>

    @GET("documents/{id}")
    suspend fun getDocument(
        @Header("Authorization") token: String,
        @Path("id") documentId: String
    ): Response<ComplianceDocument>

    @GET("documents/{id}/download")
    suspend fun downloadDocument(
        @Header("Authorization") token: String,
        @Path("id") documentId: String
    ): Response<okhttp3.ResponseBody>

    @POST("documents/{id}/approve")
    suspend fun approveDocument(
        @Header("Authorization") token: String,
        @Path("id") documentId: String,
        @Body approval: DocumentApproval
    ): Response<ComplianceDocument>

    // Audit Trail Endpoints
    @GET("audit/trail")
    suspend fun getAuditTrail(
        @Header("Authorization") token: String,
        @Query("entity_type") entityType: String? = null,
        @Query("entity_id") entityId: String? = null,
        @Query("from_date") fromDate: String? = null,
        @Query("to_date") toDate: String? = null
    ): Response<List<AuditEntry>>

    @GET("audit/compliance-score")
    suspend fun getComplianceScore(
        @Header("Authorization") token: String,
        @Query("period") period: String = "current"
    ): Response<ComplianceScore>

    // Notifications Endpoints
    @GET("notifications")
    suspend fun getNotifications(
        @Header("Authorization") token: String,
        @Query("unread_only") unreadOnly: Boolean = false
    ): Response<List<Notification>>

    @POST("notifications/{id}/mark-read")
    suspend fun markNotificationAsRead(
        @Header("Authorization") token: String,
        @Path("id") notificationId: String
    ): Response<Unit>

    @POST("notifications/mark-all-read")
    suspend fun markAllNotificationsAsRead(
        @Header("Authorization") token: String
    ): Response<Unit>

    // Real-time Monitoring Endpoints
    @GET("monitoring/status")
    suspend fun getMonitoringStatus(
        @Header("Authorization") token: String
    ): Response<MonitoringStatus>

    @GET("monitoring/metrics")
    suspend fun getMonitoringMetrics(
        @Header("Authorization") token: String,
        @Query("timeframe") timeframe: String = "1h"
    ): Response<MonitoringMetrics>

    // Quantum Security Endpoints
    @POST("quantum/key-exchange")
    suspend fun initiateQuantumKeyExchange(
        @Header("Authorization") token: String,
        @Body keyExchangeRequest: QuantumKeyExchangeRequest
    ): Response<QuantumKeyExchangeResponse>

    @POST("quantum/encrypt")
    suspend fun quantumEncrypt(
        @Header("Authorization") token: String,
        @Body encryptionRequest: QuantumEncryptionRequest
    ): Response<QuantumEncryptionResponse>

    @POST("quantum/decrypt")
    suspend fun quantumDecrypt(
        @Header("Authorization") token: String,
        @Body decryptionRequest: QuantumDecryptionRequest
    ): Response<QuantumDecryptionResponse>

    // Settings & Configuration Endpoints
    @GET("settings/user")
    suspend fun getUserSettings(
        @Header("Authorization") token: String
    ): Response<UserSettings>

    @PUT("settings/user")
    suspend fun updateUserSettings(
        @Header("Authorization") token: String,
        @Body settings: UserSettings
    ): Response<UserSettings>

    @GET("settings/organization")
    suspend fun getOrganizationSettings(
        @Header("Authorization") token: String
    ): Response<OrganizationSettings>

    // Health Check Endpoints
    @GET("health")
    suspend fun healthCheck(): Response<HealthStatus>

    @GET("health/detailed")
    suspend fun detailedHealthCheck(
        @Header("Authorization") token: String
    ): Response<DetailedHealthStatus>
}