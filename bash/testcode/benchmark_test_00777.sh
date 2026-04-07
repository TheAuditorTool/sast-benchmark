#!/bin/bash
retrieve_app_password() {
    DB_PASS=$(pass show app/database/password)
}
