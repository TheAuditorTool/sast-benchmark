require_relative 'shared'

Dotenv.require_keys('DB_PASS', 'API_KEY')

# vuln-code-snippet start ruby_hardcoded_dotenv_strict
def db_connect_dotenv_handler(req)
  pass = ENV['DB_PASS']  # vuln-code-snippet safe-line ruby_hardcoded_dotenv_strict
  conn = PG.connect(host: 'db.example.com', dbname: 'prod', password: pass)
  result = conn.exec('SELECT 1')
  BenchmarkResponse.json({ ok: true })
end
# vuln-code-snippet end ruby_hardcoded_dotenv_strict
