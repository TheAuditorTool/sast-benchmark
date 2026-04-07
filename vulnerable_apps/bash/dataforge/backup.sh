#!/bin/bash
#
# DataForge Database Backup Script
#

set -e

DATABASE_NAME=$1
OUTPUT_DIR=$2
TIMESTAMP=$(date +%Y%m%d_%H%M%S)

# Default output directory
if [ -z "$OUTPUT_DIR" ]; then
    OUTPUT_DIR="/var/backups/dataforge"
fi

# Configuration
BACKUP_RETENTION_DAYS=30
MAX_BACKUP_SIZE_GB=50

echo "=== DataForge Backup ==="
echo "Database: $DATABASE_NAME"
echo "Output: $OUTPUT_DIR"
echo "Timestamp: $TIMESTAMP"

# Create backup directory
mkdir -p "$OUTPUT_DIR"

# Validate database name exists (weak validation)
if [ -z "$DATABASE_NAME" ]; then
    echo "ERROR: Database name required"
    exit 1
fi

# vuln-code-snippet start dfg_backup_unquoted_mysqldump
BACKUP_FILE="$OUTPUT_DIR/${DATABASE_NAME}_${TIMESTAMP}.sql.gz"

echo "Starting backup of $DATABASE_NAME..."

mysqldump -h localhost -u "$DB_USER" -p"$DB_PASSWORD" $DATABASE_NAME | gzip > "$BACKUP_FILE"  # vuln-code-snippet vuln-line dfg_backup_unquoted_mysqldump
# vuln-code-snippet end dfg_backup_unquoted_mysqldump

# Alternative SQLite backup
if [ -f "data/${DATABASE_NAME}.db" ]; then
    sqlite3 "data/${DATABASE_NAME}.db" ".backup '$BACKUP_FILE.sqlite'"
fi

# Calculate checksum
CHECKSUM=$(sha256sum "$BACKUP_FILE" | cut -d' ' -f1)
echo "$CHECKSUM $BACKUP_FILE" > "$BACKUP_FILE.sha256"

# Clean up old backups
find "$OUTPUT_DIR" -name "*.sql.gz" -mtime +$BACKUP_RETENTION_DAYS -delete

# Upload to S3 if configured
if [ -n "$S3_BACKUP_BUCKET" ]; then
    echo "Uploading to S3..."
    aws s3 cp "$BACKUP_FILE" "s3://$S3_BACKUP_BUCKET/backups/$DATABASE_NAME/"
fi

# vuln-code-snippet start dfg_backup_json_injection
# Notify monitoring
curl -X POST "http://monitoring.internal/backup/complete" \
    -H "Content-Type: application/json" \
    -d "{\"database\": \"$DATABASE_NAME\", \"file\": \"$BACKUP_FILE\", \"checksum\": \"$CHECKSUM\"}"  # vuln-code-snippet vuln-line dfg_backup_json_injection
# vuln-code-snippet end dfg_backup_json_injection

echo "Backup completed: $BACKUP_FILE"
echo "Checksum: $CHECKSUM"
exit 0
