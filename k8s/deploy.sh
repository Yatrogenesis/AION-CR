#!/bin/bash

set -e

echo "🚀 Deploying AION-CR Production Stack"

# Check if kubectl is available
if ! command -v kubectl &> /dev/null; then
    echo "❌ kubectl is not installed"
    exit 1
fi

# Check if helm is available
if ! command -v helm &> /dev/null; then
    echo "❌ Helm is not installed"
    exit 1
fi

# Function to create secrets
create_secrets() {
    echo "🔐 Creating secrets..."

    # PostgreSQL secret
    kubectl create secret generic postgresql-secret \
        --namespace=aion-cr \
        --from-literal=postgres-password="$(openssl rand -base64 32)" \
        --dry-run=client -o yaml | kubectl apply -f -

    # Redis secret
    kubectl create secret generic redis-secret \
        --namespace=aion-cr \
        --from-literal=redis-password="$(openssl rand -base64 32)" \
        --dry-run=client -o yaml | kubectl apply -f -

    # AION-CR application secrets
    kubectl create secret generic aion-cr-secrets \
        --namespace=aion-cr \
        --from-literal=database-url="postgresql://aion:$(openssl rand -base64 32)@postgresql:5432/aion_cr" \
        --from-literal=redis-url="redis://:$(openssl rand -base64 32)@redis:6379" \
        --from-literal=jwt-secret="$(openssl rand -base64 64)" \
        --dry-run=client -o yaml | kubectl apply -f -

    # Grafana secret
    kubectl create secret generic grafana-secret \
        --namespace=aion-cr-monitoring \
        --from-literal=admin-password="$(openssl rand -base64 32)" \
        --dry-run=client -o yaml | kubectl apply -f -

    # Elasticsearch secret
    kubectl create secret generic elasticsearch-secret \
        --namespace=aion-cr-monitoring \
        --from-literal=password="$(openssl rand -base64 32)" \
        --dry-run=client -o yaml | kubectl apply -f -

    echo "✅ Secrets created"
}

# Function to create service accounts and RBAC
create_rbac() {
    echo "🔒 Creating RBAC resources..."

    # AION-CR service account
    cat <<EOF | kubectl apply -f -
apiVersion: v1
kind: ServiceAccount
metadata:
  name: aion-cr-service-account
  namespace: aion-cr
---
apiVersion: rbac.authorization.k8s.io/v1
kind: ClusterRole
metadata:
  name: aion-cr-cluster-role
rules:
- apiGroups: [""]
  resources: ["pods", "services", "endpoints", "configmaps", "secrets"]
  verbs: ["get", "list", "watch", "create", "update", "patch", "delete"]
- apiGroups: ["apps"]
  resources: ["deployments", "replicasets"]
  verbs: ["get", "list", "watch", "create", "update", "patch", "delete"]
- apiGroups: ["networking.k8s.io"]
  resources: ["ingresses"]
  verbs: ["get", "list", "watch"]
---
apiVersion: rbac.authorization.k8s.io/v1
kind: ClusterRoleBinding
metadata:
  name: aion-cr-cluster-role-binding
roleRef:
  apiGroup: rbac.authorization.k8s.io
  kind: ClusterRole
  name: aion-cr-cluster-role
subjects:
- kind: ServiceAccount
  name: aion-cr-service-account
  namespace: aion-cr
EOF

    # Prometheus service account
    cat <<EOF | kubectl apply -f -
apiVersion: v1
kind: ServiceAccount
metadata:
  name: prometheus
  namespace: aion-cr-monitoring
---
apiVersion: rbac.authorization.k8s.io/v1
kind: ClusterRole
metadata:
  name: prometheus
rules:
- apiGroups: [""]
  resources: ["nodes", "services", "endpoints", "pods"]
  verbs: ["get", "list", "watch"]
- apiGroups: [""]
  resources: ["configmaps"]
  verbs: ["get"]
- nonResourceURLs: ["/metrics"]
  verbs: ["get"]
---
apiVersion: rbac.authorization.k8s.io/v1
kind: ClusterRoleBinding
metadata:
  name: prometheus
roleRef:
  apiGroup: rbac.authorization.k8s.io
  kind: ClusterRole
  name: prometheus
subjects:
- kind: ServiceAccount
  name: prometheus
  namespace: aion-cr-monitoring
EOF

    echo "✅ RBAC resources created"
}

# Function to create storage classes
create_storage() {
    echo "💾 Creating storage classes..."

    cat <<EOF | kubectl apply -f -
apiVersion: storage.k8s.io/v1
kind: StorageClass
metadata:
  name: fast-ssd
provisioner: kubernetes.io/aws-ebs
parameters:
  type: gp3
  iops: "3000"
  throughput: "125"
volumeBindingMode: WaitForFirstConsumer
allowVolumeExpansion: true
reclaimPolicy: Retain
EOF

    echo "✅ Storage classes created"
}

# Function to install monitoring tools
install_monitoring_tools() {
    echo "📊 Installing monitoring tools..."

    # Add Helm repositories
    helm repo add prometheus-community https://prometheus-community.github.io/helm-charts
    helm repo add grafana https://grafana.github.io/helm-charts
    helm repo add elastic https://helm.elastic.co
    helm repo update

    # Install Elasticsearch
    helm upgrade --install elasticsearch elastic/elasticsearch \
        --namespace aion-cr-monitoring \
        --create-namespace \
        --set replicas=1 \
        --set minimumMasterNodes=1 \
        --set resources.requests.cpu="1000m" \
        --set resources.requests.memory="2Gi" \
        --set resources.limits.cpu="2000m" \
        --set resources.limits.memory="4Gi" \
        --set volumeClaimTemplate.resources.requests.storage="50Gi" \
        --set volumeClaimTemplate.storageClassName="fast-ssd"

    # Install Node Exporter
    helm upgrade --install node-exporter prometheus-community/prometheus-node-exporter \
        --namespace aion-cr-monitoring \
        --create-namespace

    # Install Postgres Exporter
    helm upgrade --install postgres-exporter prometheus-community/prometheus-postgres-exporter \
        --namespace aion-cr \
        --set config.datasource.host="postgresql" \
        --set config.datasource.user="postgres" \
        --set config.datasource.passwordSecret.name="postgresql-secret" \
        --set config.datasource.passwordSecret.key="postgres-password"

    # Install Redis Exporter
    helm upgrade --install redis-exporter prometheus-community/prometheus-redis-exporter \
        --namespace aion-cr \
        --set redisAddress="redis://redis:6379"

    echo "✅ Monitoring tools installed"
}

# Main deployment function
deploy_stack() {
    echo "🏗️ Deploying AION-CR stack..."

    # Create namespaces
    kubectl apply -f namespace.yaml

    # Create storage and RBAC
    create_storage
    create_rbac
    create_secrets

    # Deploy core infrastructure
    echo "📦 Deploying PostgreSQL..."
    kubectl apply -f postgresql.yaml

    echo "📦 Deploying Redis..."
    kubectl apply -f redis.yaml

    # Wait for databases to be ready
    echo "⏳ Waiting for databases to be ready..."
    kubectl wait --for=condition=ready pod -l app=postgresql -n aion-cr --timeout=300s
    kubectl wait --for=condition=ready pod -l app=redis -n aion-cr --timeout=300s

    # Deploy applications
    echo "📦 Deploying AION-CR API..."
    kubectl apply -f aion-cr-deployment.yaml

    echo "📦 Deploying Web UI..."
    kubectl apply -f web-ui-deployment.yaml

    # Install monitoring
    install_monitoring_tools

    # Deploy monitoring stack
    echo "📦 Deploying Prometheus..."
    kubectl apply -f monitoring/prometheus.yaml

    echo "📦 Deploying Grafana..."
    kubectl apply -f monitoring/grafana.yaml

    echo "📦 Deploying Jaeger..."
    kubectl apply -f monitoring/jaeger.yaml

    # Wait for applications to be ready
    echo "⏳ Waiting for applications to be ready..."
    kubectl wait --for=condition=ready pod -l app=aion-cr-api -n aion-cr --timeout=300s
    kubectl wait --for=condition=ready pod -l app=aion-cr-web-ui -n aion-cr --timeout=300s

    echo "✅ AION-CR stack deployed successfully!"
}

# Function to show deployment status
show_status() {
    echo ""
    echo "📊 Deployment Status:"
    echo "===================="

    echo ""
    echo "AION-CR Namespace:"
    kubectl get pods -n aion-cr -o wide

    echo ""
    echo "Monitoring Namespace:"
    kubectl get pods -n aion-cr-monitoring -o wide

    echo ""
    echo "Services:"
    kubectl get svc -n aion-cr
    kubectl get svc -n aion-cr-monitoring

    echo ""
    echo "Ingresses:"
    kubectl get ingress -n aion-cr
    kubectl get ingress -n aion-cr-monitoring

    echo ""
    echo "🌐 Access URLs:"
    echo "==============="
    echo "API: https://api.aion-cr.ai"
    echo "Dashboard: https://dashboard.aion-cr.ai"
    echo "Monitoring: https://monitoring.aion-cr.ai"
    echo "Tracing: https://tracing.aion-cr.ai"

    echo ""
    echo "🔐 Secrets created for production use"
    echo "📊 Full observability stack deployed"
    echo "🤖 Maximum autonomy mode enabled"
    echo ""
    echo "✅ AION-CR Production Stack is fully operational!"
}

# Main execution
case "${1:-deploy}" in
    "deploy")
        deploy_stack
        show_status
        ;;
    "status")
        show_status
        ;;
    "secrets")
        create_secrets
        ;;
    "monitoring")
        install_monitoring_tools
        ;;
    *)
        echo "Usage: $0 [deploy|status|secrets|monitoring]"
        exit 1
        ;;
esac