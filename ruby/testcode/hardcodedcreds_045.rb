require_relative 'shared'

# vuln-code-snippet start ruby_hardcoded_infisical
def infisical_db_connect_handler(req)
  password = Infisical.get('DB_PASSWORD')  # vuln-code-snippet safe-line ruby_hardcoded_infisical
  conn = PG.connect(host: 'db.example.com', dbname: 'prod', password: password)
  result = conn.exec('SELECT 1')
  BenchmarkResponse.json({ ok: true })
end
# vuln-code-snippet end ruby_hardcoded_infisical
