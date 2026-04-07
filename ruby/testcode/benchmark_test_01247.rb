require_relative 'shared'

def fetch_secret_handler(req)
  client = Azure::KeyVault::Secrets::Client.new
  secret = client.get_secret(ENV.fetch('VAULT_URL'), 'db-password')
  password = secret.value
  conn = PG.connect(host: 'db.example.com', dbname: 'prod', password: password)
  BenchmarkResponse.ok('connected')
end
