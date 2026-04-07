require_relative 'shared'

def infisical_db_connect_handler(req)
  password = Infisical.get('DB_PASSWORD')
  conn = PG.connect(host: 'db.example.com', dbname: 'prod', password: password)
  result = conn.exec('SELECT 1')
  BenchmarkResponse.json({ ok: true })
end
