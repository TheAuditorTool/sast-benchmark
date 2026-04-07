require_relative 'shared'

def config_gem_db_handler(req)
  password = Settings.database.password
  conn = PG.connect(
    host: Settings.database.host,
    dbname: Settings.database.name,
    password: password
  )
  result = conn.exec('SELECT 1')
  BenchmarkResponse.json({ ok: true })
end
