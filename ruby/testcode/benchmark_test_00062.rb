require_relative 'shared'

def db_connect_safe_handler(req)
  secret = Vault.logical.read('secret/myapp/db')
  password = secret.data[:password]
  conn = PG.connect(host: 'db.example.com', dbname: 'prod', password: password)
  result = conn.exec('SELECT 1')
  BenchmarkResponse.json({ ok: true })
end
