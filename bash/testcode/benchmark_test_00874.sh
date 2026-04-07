#!/bin/bash
find_article() {
    local title="$1"
    mysql -u "$DB_USER" -p"$DB_PASS" blog_db -e "SELECT id, content FROM articles WHERE title LIKE '%${title}%'"
}
