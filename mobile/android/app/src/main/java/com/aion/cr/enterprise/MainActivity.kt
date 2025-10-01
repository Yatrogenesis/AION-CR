package com.aion.cr.enterprise

import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.activity.viewModels
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.padding
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Modifier
import androidx.core.splashscreen.SplashScreen.Companion.installSplashScreen
import androidx.lifecycle.lifecycleScope
import androidx.navigation.compose.rememberNavController
import com.aion.cr.enterprise.navigation.AionNavHost
import com.aion.cr.enterprise.ui.theme.AionCRTheme
import com.aion.cr.enterprise.viewmodel.AuthViewModel
import com.aion.cr.enterprise.viewmodel.MainViewModel
import dagger.hilt.android.AndroidEntryPoint
import kotlinx.coroutines.launch

/**
 * AION-CR Enterprise Mobile Application
 *
 * Main entry point for the AION-CR Enterprise mobile application.
 * Provides quantum-safe compliance management on mobile devices.
 *
 * Features:
 * - Real-time compliance monitoring
 * - Secure document access
 * - Biometric authentication
 * - Offline capability
 * - Push notifications
 * - Quantum-resistant cryptography
 */
@AndroidEntryPoint
class MainActivity : ComponentActivity() {

    private val authViewModel: AuthViewModel by viewModels()
    private val mainViewModel: MainViewModel by viewModels()

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)

        // Install splash screen
        val splashScreen = installSplashScreen()
        splashScreen.setKeepOnScreenCondition {
            mainViewModel.isLoading.value
        }

        // Initialize security components
        initializeQuantumSecurity()

        setContent {
            AionCRTheme {
                val navController = rememberNavController()
                val authState by authViewModel.authState.collectAsState()
                val isLoading by mainViewModel.isLoading.collectAsState()

                LaunchedEffect(Unit) {
                    // Initialize app components
                    mainViewModel.initializeApp()

                    // Check authentication status
                    authViewModel.checkAuthStatus()
                }

                if (!isLoading) {
                    Surface(
                        modifier = Modifier.fillMaxSize(),
                        color = MaterialTheme.colorScheme.background
                    ) {
                        AionNavHost(
                            navController = navController,
                            authState = authState,
                            modifier = Modifier.fillMaxSize()
                        )
                    }
                }
            }
        }
    }

    /**
     * Initialize quantum-resistant security components
     */
    private fun initializeQuantumSecurity() {
        lifecycleScope.launch {
            try {
                // Initialize quantum cryptography
                mainViewModel.initializeQuantumCrypto()

                // Setup secure storage
                mainViewModel.initializeSecureStorage()

                // Configure biometric authentication
                mainViewModel.configureBiometrics()

            } catch (e: Exception) {
                // Handle security initialization failure
                mainViewModel.handleSecurityError(e)
            }
        }
    }

    override fun onResume() {
        super.onResume()
        // Refresh security tokens
        authViewModel.refreshTokensIfNeeded()

        // Resume real-time monitoring
        mainViewModel.resumeRealTimeMonitoring()
    }

    override fun onPause() {
        super.onPause()
        // Pause non-critical operations
        mainViewModel.pauseNonCriticalOperations()
    }

    override fun onDestroy() {
        super.onDestroy()
        // Cleanup security components
        mainViewModel.cleanupSecurityComponents()
    }
}