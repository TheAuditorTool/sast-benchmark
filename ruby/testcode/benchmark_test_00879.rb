require_relative 'shared'

DB_URL = "postgres://admin:s3cr3t@localhost/prod"

def connect_db_handler(req)
  conn = PG.connect(DB_URL)
  result = conn.exec('SELECT 1')
  BenchmarkResponse.json({ ok: true })
end
