require_relative 'shared'

# vuln-code-snippet start ruby_hardcoded_no_secret_in_code
def multi_service_handler(req)
  db_pass    = ENV.fetch('DATABASE_PASSWORD')  # vuln-code-snippet safe-line ruby_hardcoded_no_secret_in_code
  api_key    = ENV.fetch('API_KEY')
  jwt_secret = ENV.fetch('JWT_SECRET')
  conn = PG.connect(host: 'db.example.com', dbname: 'prod', password: db_pass)
  BenchmarkResponse.json({ ok: true })
end
# vuln-code-snippet end ruby_hardcoded_no_secret_in_code
