#!/bin/bash
configure_production_env() {
    source <(echo "export APP_ENV=production"; echo "export LOG_LEVEL=warn")
}
