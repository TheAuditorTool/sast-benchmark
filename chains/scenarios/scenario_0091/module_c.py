import os
import sqlite3

DB_PATH = "/tmp/app_data.db"

def init_db():
    conn = sqlite3.connect(DB_PATH)
    conn.execute(
        "CREATE TABLE IF NOT EXISTS users "
        "(id INTEGER PRIMARY KEY, username TEXT, role TEXT)"
    )
    conn.commit()
    conn.close()
    os.chmod(DB_PATH, 0o666)
