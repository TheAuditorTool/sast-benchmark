#!/bin/bash
get_service_token() {
    TOKEN=$(secret-tool lookup service myapp account deploy-bot)
}
