require_relative 'shared'

# vuln-code-snippet start ruby_hardcoded_config_gem
def config_gem_db_handler(req)
  password = Settings.database.password  # vuln-code-snippet safe-line ruby_hardcoded_config_gem
  conn = PG.connect(
    host: Settings.database.host,
    dbname: Settings.database.name,
    password: password
  )
  result = conn.exec('SELECT 1')
  BenchmarkResponse.json({ ok: true })
end
# vuln-code-snippet end ruby_hardcoded_config_gem
