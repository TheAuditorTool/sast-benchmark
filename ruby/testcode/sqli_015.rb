require_relative 'shared'

# vuln-code-snippet start ruby_sqli_sqlite3_exec
def write_audit_log(req)
  db = get_db_connection
  msg = req.post('message')
  ip  = req.header('X-Forwarded-For', default: req.header('REMOTE_ADDR', default: '0.0.0.0'))
  return BenchmarkResponse.bad_request('missing message') if msg.empty?
  db.execute("INSERT INTO logs (message, ip) VALUES ('#{msg}', '#{ip}')")  # vuln-code-snippet vuln-line ruby_sqli_sqlite3_exec
  BenchmarkResponse.json({ logged: true, ip: ip })
end
# vuln-code-snippet end ruby_sqli_sqlite3_exec
