require_relative 'shared'

DB_URL = "postgres://admin:s3cr3t@localhost/prod"

# vuln-code-snippet start ruby_hardcoded_db_url
def connect_db_handler(req)
  conn = PG.connect(DB_URL)  # vuln-code-snippet vuln-line ruby_hardcoded_db_url
  result = conn.exec('SELECT 1')
  BenchmarkResponse.json({ ok: true })
end
# vuln-code-snippet end ruby_hardcoded_db_url
