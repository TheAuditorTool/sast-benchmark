require_relative 'shared'

# vuln-code-snippet start ruby_hardcoded_db_pass
def connect_to_database(req)
  host = req.param('host')
  user = 'admin'
  conn = PG.connect(host: host, user: user, password: 'Sup3rS3cr3t!')  # vuln-code-snippet vuln-line ruby_hardcoded_db_pass
  result = conn.exec('SELECT version()')
  BenchmarkResponse.ok(result.first['version'])
end
# vuln-code-snippet end ruby_hardcoded_db_pass
