require_relative 'shared'

def connect_to_database_env(req)
  host = req.param('host')
  user = ENV['DB_USER']
  conn = PG.connect(host: host, user: user, password: ENV['DB_PASSWORD'])
  result = conn.exec('SELECT version()')
  BenchmarkResponse.ok(result.first['version'])
end
