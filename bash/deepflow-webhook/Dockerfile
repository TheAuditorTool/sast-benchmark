FROM alpine:3.19

# Install required tools
RUN apk add --no-cache \
    bash \
    socat \
    netcat-openbsd \
    curl \
    jq \
    mysql-client \
    openssl \
    coreutils

# Create app directory
WORKDIR /app

# Copy application files
COPY . .

# Make scripts executable
RUN chmod +x server.sh handlers/*.sh lib/*.sh config.sh

# Create required directories
RUN mkdir -p /var/deployments /var/log

# Expose port
EXPOSE 8080

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:8080/health || exit 1

# Run server
CMD ["./server.sh"]
