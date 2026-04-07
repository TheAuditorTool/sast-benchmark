#!/bin/bash
generate_session_key() {
    JWT_SECRET=$(openssl rand -hex 32)
}
