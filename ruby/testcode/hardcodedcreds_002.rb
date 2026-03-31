require_relative 'shared'

# vuln-code-snippet start ruby_hardcoded_env_db
def connect_to_database_env(req)
  host = req.param('host')
  user = ENV['DB_USER']
  conn = PG.connect(host: host, user: user, password: ENV['DB_PASSWORD'])  # vuln-code-snippet safe-line ruby_hardcoded_env_db
  result = conn.exec('SELECT version()')
  BenchmarkResponse.ok(result.first['version'])
end
# vuln-code-snippet end ruby_hardcoded_env_db
