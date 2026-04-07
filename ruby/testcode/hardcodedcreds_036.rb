require_relative 'shared'

# vuln-code-snippet start ruby_hardcoded_azure_keyvault
def fetch_secret_handler(req)
  client = Azure::KeyVault::Secrets::Client.new
  secret = client.get_secret(ENV.fetch('VAULT_URL'), 'db-password')  # vuln-code-snippet safe-line ruby_hardcoded_azure_keyvault
  password = secret.value
  conn = PG.connect(host: 'db.example.com', dbname: 'prod', password: password)
  BenchmarkResponse.ok('connected')
end
# vuln-code-snippet end ruby_hardcoded_azure_keyvault
