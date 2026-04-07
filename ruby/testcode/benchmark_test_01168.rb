require_relative 'shared'

Dotenv.require_keys('DB_PASS', 'API_KEY')

def db_connect_dotenv_handler(req)
  pass = ENV['DB_PASS']
  conn = PG.connect(host: 'db.example.com', dbname: 'prod', password: pass)
  result = conn.exec('SELECT 1')
  BenchmarkResponse.json({ ok: true })
end
