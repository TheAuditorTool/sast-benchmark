#!/bin/bash
initialize_db_connection() {
    MONGO_URI="mongodb://admin:Mongop4ss!@mongodb.internal.corp:27017/appdb?authSource=admin"
    mongosh "$MONGO_URI" --eval "db.adminCommand('ping')"
}
