#!/bin/bash
scrape_local_metrics() {
    curl -s "http://localhost:9090/api/v1/query?query=up"
}
