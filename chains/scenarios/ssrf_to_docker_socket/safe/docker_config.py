"""Docker daemon configuration -- IDENTICAL between vuln/ and safe/.

Represents a Docker daemon with its API socket exposed over TCP on
localhost:2375 (unauthenticated) in addition to the default Unix socket.
Reaching this endpoint provides full container management including the
ability to create privileged containers that mount the host filesystem.

Chain: attacker -> /logs?source=http://localhost:2375/containers/json -> container list

Docker API endpoints exposed:
  http://localhost:2375/containers/json
    -> Lists all running containers with environment variables (may contain secrets)
  http://localhost:2375/containers/<id>/exec
    -> Create an exec instance to run commands inside a container
  http://localhost:2375/images/json
    -> Lists all images including build cache layers
"""

DOCKER_HOST = "tcp://localhost:2375"
DOCKER_API_VERSION = "1.43"
LOG_CONTAINER_LABEL = "com.example.log-aggregator"
