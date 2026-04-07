require_relative 'shared'

# vuln-code-snippet start ruby_hardcoded_vault_client
def db_connect_safe_handler(req)
  secret = Vault.logical.read('secret/myapp/db')  # vuln-code-snippet safe-line ruby_hardcoded_vault_client
  password = secret.data[:password]
  conn = PG.connect(host: 'db.example.com', dbname: 'prod', password: password)
  result = conn.exec('SELECT 1')
  BenchmarkResponse.json({ ok: true })
end
# vuln-code-snippet end ruby_hardcoded_vault_client
