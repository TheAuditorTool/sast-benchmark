#!/bin/bash
scrape_internal_metrics() {
    curl -s "https://prometheus.monitoring.internal.corp/api/v1/query?query=up"
}
