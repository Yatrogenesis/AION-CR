// AION-CR Notifications Hook
// Maximum Autonomy Level 255 - Complete System Integration

import { useState, useCallback, useEffect } from 'react';
import { Notification } from '../types';

interface NotificationState {
  notifications: Notification[];
  unreadCount: number;
}

export function useNotifications() {
  const [state, setState] = useState<NotificationState>({
    notifications: [],
    unreadCount: 0,
  });

  // Add notification
  const addNotification = useCallback((notification: Omit<Notification, 'id'>) => {
    const newNotification: Notification = {
      ...notification,
      id: notification.id || `notification-${Date.now()}-${Math.random()}`,
      timestamp: notification.timestamp || new Date(),
    };

    setState(prev => ({
      notifications: [newNotification, ...prev.notifications],
      unreadCount: prev.unreadCount + 1,
    }));

    // Auto-remove notification if autoHide is true
    if (newNotification.autoHide) {
      setTimeout(() => {
        removeNotification(newNotification.id);
      }, 6000);
    }
  }, []);

  // Remove notification
  const removeNotification = useCallback((id: string) => {
    setState(prev => {
      const notification = prev.notifications.find(n => n.id === id);
      const wasUnread = notification && !notification.read;

      return {
        notifications: prev.notifications.filter(n => n.id !== id),
        unreadCount: wasUnread ? Math.max(0, prev.unreadCount - 1) : prev.unreadCount,
      };
    });
  }, []);

  // Mark notification as read
  const markAsRead = useCallback((id: string) => {
    setState(prev => {
      const notifications = prev.notifications.map(notification =>
        notification.id === id ? { ...notification, read: true } : notification
      );

      const wasUnread = prev.notifications.find(n => n.id === id && !n.read);
      const unreadCount = wasUnread ? Math.max(0, prev.unreadCount - 1) : prev.unreadCount;

      return { notifications, unreadCount };
    });
  }, []);

  // Mark all notifications as read
  const markAllAsRead = useCallback(() => {
    setState(prev => ({
      notifications: prev.notifications.map(notification => ({ ...notification, read: true })),
      unreadCount: 0,
    }));
  }, []);

  // Clear all notifications
  const clearAll = useCallback(() => {
    setState({
      notifications: [],
      unreadCount: 0,
    });
  }, []);

  // Get notifications by type
  const getNotificationsByType = useCallback((type: Notification['type']) => {
    return state.notifications.filter(notification => notification.type === type);
  }, [state.notifications]);

  // Get unread notifications
  const getUnreadNotifications = useCallback(() => {
    return state.notifications.filter(notification => !notification.read);
  }, [state.notifications]);

  // Show success notification
  const showSuccess = useCallback((title: string, message: string, autoHide = true) => {
    addNotification({
      type: 'success',
      title,
      message,
      autoHide,
      timestamp: new Date(),
    });
  }, [addNotification]);

  // Show error notification
  const showError = useCallback((title: string, message: string, autoHide = false) => {
    addNotification({
      type: 'error',
      title,
      message,
      autoHide,
      timestamp: new Date(),
    });
  }, [addNotification]);

  // Show warning notification
  const showWarning = useCallback((title: string, message: string, autoHide = true) => {
    addNotification({
      type: 'warning',
      title,
      message,
      autoHide,
      timestamp: new Date(),
    });
  }, [addNotification]);

  // Show info notification
  const showInfo = useCallback((title: string, message: string, autoHide = true) => {
    addNotification({
      type: 'info',
      title,
      message,
      autoHide,
      timestamp: new Date(),
    });
  }, [addNotification]);

  // Load persisted notifications on mount
  useEffect(() => {
    const savedNotifications = localStorage.getItem('aion_notifications');
    if (savedNotifications) {
      try {
        const parsed = JSON.parse(savedNotifications);
        if (Array.isArray(parsed)) {
          const notifications = parsed.map(n => ({
            ...n,
            timestamp: new Date(n.timestamp),
          }));
          const unreadCount = notifications.filter(n => !n.read).length;
          setState({ notifications, unreadCount });
        }
      } catch (error) {
        console.error('Failed to load saved notifications:', error);
      }
    }
  }, []);

  // Persist notifications to localStorage
  useEffect(() => {
    const notificationsToSave = state.notifications.slice(0, 50); // Keep only last 50
    localStorage.setItem('aion_notifications', JSON.stringify(notificationsToSave));
  }, [state.notifications]);

  return {
    notifications: state.notifications,
    unreadCount: state.unreadCount,
    addNotification,
    removeNotification,
    markAsRead,
    markAllAsRead,
    clearAll,
    getNotificationsByType,
    getUnreadNotifications,
    showSuccess,
    showError,
    showWarning,
    showInfo,
  };
}