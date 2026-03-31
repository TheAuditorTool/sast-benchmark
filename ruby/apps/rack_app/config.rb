require_relative '../../testcode/shared'

# rack_app - Database configuration

# vuln-code-snippet start rk_hardcoded_db_pass
def db_connect_hardcoded(env)
  req = Rack::Request.new(env)
  host = req.params['host'] || 'localhost'
  conn = PG.connect(host: host, dbname: 'blog', user: 'admin', password: 'BlogP@ss2024!') # vuln-code-snippet vuln-line rk_hardcoded_db_pass
  result = conn.exec('SELECT version()')
  [200, { 'Content-Type' => 'text/plain' }, [result.first['version']]]
end
# vuln-code-snippet end rk_hardcoded_db_pass

# vuln-code-snippet start rk_hardcoded_env
def db_connect_env(env)
  req = Rack::Request.new(env)
  host = ENV['DB_HOST'] || 'localhost'
  pass = ENV['DB_PASS']
  conn = PG.connect(host: host, dbname: ENV['DB_NAME'], user: ENV['DB_USER'], password: pass) # vuln-code-snippet safe-line rk_hardcoded_env
  result = conn.exec('SELECT version()')
  [200, { 'Content-Type' => 'text/plain' }, [result.first['version']]]
end
# vuln-code-snippet end rk_hardcoded_env
