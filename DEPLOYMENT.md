# 🚀 AION-CR Deployment Guide

## Quick Start

### Prerequisites
- Rust 1.70+
- SQLite3 (for default database)
- PostgreSQL (optional, for production)

### Installation

1. **Clone and Build**
```bash
git clone <repository-url>
cd AION-CR
cargo build --release
```

2. **Initialize System**
```bash
# Run the startup script
./start-aion-cr.sh

# Or manually:
./target/release/aion-cr init --organization "Your Org" --sector "tech" --region "global"
./target/release/aion-cr compliance load-standards
./target/release/aion-cr server --host 0.0.0.0 --port 8080
```

### Configuration

Create `aion.toml` in the project root:

```toml
[server]
host = "0.0.0.0"
port = 8080
workers = 4

[database]
url = "sqlite:./aion_cr.db"
pool_size = 10
timeout = 30

[organization]
name = "Your Organization"
sector = "technology"
region = "global"
risk_profile = "medium"

[integration]
ectus_r_api = "https://your-ectus-api.com"
enable_auto_fix = true
```

## Production Deployment

### Docker Deployment

```dockerfile
FROM rust:1.70 as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y \
    libsqlite3-dev \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/aion-cr /usr/local/bin/
COPY --from=builder /app/aion.toml /etc/aion/aion.toml

EXPOSE 8080
CMD ["aion-cr", "server", "--config", "/etc/aion/aion.toml"]
```

### Kubernetes Deployment

```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: aion-cr
spec:
  replicas: 3
  selector:
    matchLabels:
      app: aion-cr
  template:
    metadata:
      labels:
        app: aion-cr
    spec:
      containers:
      - name: aion-cr
        image: aion-cr:latest
        ports:
        - containerPort: 8080
        env:
        - name: DATABASE_URL
          valueFrom:
            secretKeyRef:
              name: aion-secrets
              key: database-url
        volumeMounts:
        - name: config
          mountPath: /etc/aion
      volumes:
      - name: config
        configMap:
          name: aion-config
```

## Integration with Ectus-R

### API Integration

```bash
# Health check
curl http://localhost:8080/health

# Assess compliance for generated code
curl -X POST http://localhost:8080/api/v1/compliance/assess \
  -H "Content-Type: application/json" \
  -d '{
    "entity_id": "ectus-generated-001",
    "framework_ids": ["gdpr", "sox"],
    "code_artifact": {
      "language": "rust",
      "files": {...}
    }
  }'
```

### Real-time Integration

```rust
use aion_compliance::AdvancedComplianceEngine;

// In Ectus-R generation pipeline
async fn validate_generated_code(project: &GeneratedProject) -> ComplianceResult {
    let compliance_engine = AdvancedComplianceEngine::new(business_rules);

    compliance_engine.assess_compliance_comprehensive(
        &project.entity_id,
        &determine_frameworks(&project),
        &build_context(&project)
    ).await
}
```

## Monitoring and Alerts

### Health Endpoints

- `GET /health` - System health
- `GET /api/v1/frameworks` - Available frameworks
- `GET /api/v1/status` - Detailed system status

### Metrics Integration

AION-CR exposes Prometheus metrics at `/metrics`:

- `aion_assessments_total` - Total compliance assessments
- `aion_violations_total` - Total violations detected
- `aion_frameworks_loaded` - Number of loaded frameworks
- `aion_response_time_seconds` - API response times

### Logging

Configure via `RUST_LOG` environment variable:

```bash
export RUST_LOG=info
export AION_LOG_LEVEL=info
```

## Security Considerations

1. **Database Security**
   - Use encrypted connections for PostgreSQL
   - Regular database backups
   - Access control and authentication

2. **API Security**
   - JWT authentication for API endpoints
   - Rate limiting and request validation
   - HTTPS in production

3. **Compliance Data**
   - Audit trail for all operations
   - Data retention policies
   - Encrypted storage of sensitive compliance data

## Troubleshooting

### Common Issues

1. **Build Failures**
```bash
# Clean and rebuild
cargo clean
cargo build --release
```

2. **Database Connection Issues**
```bash
# Check database status
./target/release/aion-cr database status

# Reinitialize if needed
./target/release/aion-cr database init --database-url "sqlite:./aion_cr.db"
```

3. **Framework Loading Issues**
```bash
# Reload standard frameworks
./target/release/aion-cr compliance load-standards --force
```

### Logs and Debugging

```bash
# Enable debug logging
export RUST_LOG=debug

# Check system status
./target/release/aion-cr --help
./target/release/aion-cr framework list
./target/release/aion-cr database status
```

## Performance Tuning

### Database Optimization

- Use PostgreSQL for production workloads
- Configure connection pooling
- Regular maintenance and vacuuming
- Proper indexing for compliance queries

### Server Configuration

```toml
[server]
workers = 8  # Adjust based on CPU cores
timeout = 30  # Request timeout in seconds

[database]
pool_size = 20  # Database connection pool
timeout = 60    # Database operation timeout
```

### Caching

- Framework definitions are cached in memory
- Assessment results can be cached for 24 hours
- Configure Redis for distributed caching in multi-instance deployments

## Support and Maintenance

### Regular Tasks

1. **Daily**
   - Monitor system health and logs
   - Check compliance assessment success rates

2. **Weekly**
   - Review and update compliance frameworks
   - Database maintenance and optimization

3. **Monthly**
   - Security updates and patches
   - Performance analysis and tuning
   - Backup verification and testing

### Backup and Recovery

```bash
# Database backup
./target/release/aion-cr database backup --output ./backups/

# Framework export
./target/release/aion-cr framework export --output ./backups/frameworks.json

# Full system backup (recommended)
tar -czf aion-cr-backup-$(date +%Y%m%d).tar.gz \
  ./data/ \
  ./logs/ \
  ./aion.toml \
  ./backups/
```