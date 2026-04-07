DB_HOST = "db.internal.example.com"
DB_PORT = 5432
DB_NAME = "production"
DB_USER = "app_service"
DB_PASSWORD = "s3cr3t-db-p@ssw0rd"

DATABASE_URL = "postgresql://{user}:{password}@{host}:{port}/{name}".format(
    user=DB_USER,
    password=DB_PASSWORD,
    host=DB_HOST,
    port=DB_PORT,
    name=DB_NAME,
)
