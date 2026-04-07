require_relative 'shared'

def multi_service_handler(req)
  db_pass    = ENV.fetch('DATABASE_PASSWORD')
  api_key    = ENV.fetch('API_KEY')
  jwt_secret = ENV.fetch('JWT_SECRET')
  conn = PG.connect(host: 'db.example.com', dbname: 'prod', password: db_pass)
  BenchmarkResponse.json({ ok: true })
end
