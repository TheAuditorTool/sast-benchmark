require_relative 'shared'

def connect_to_database(req)
  host = req.param('host')
  user = 'admin'
  conn = PG.connect(host: host, user: user, password: 'Sup3rS3cr3t!')
  result = conn.exec('SELECT version()')
  BenchmarkResponse.ok(result.first['version'])
end
