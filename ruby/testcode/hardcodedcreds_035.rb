require 'json'
require_relative 'shared'

# vuln-code-snippet start ruby_hardcoded_aws_secrets_mgr
def rds_connect_handler(req)
  client = Aws::SecretsManager::Client.new(region: 'us-east-1')
  secret = client.get_secret_value(secret_id: 'myapp/db/password')  # vuln-code-snippet safe-line ruby_hardcoded_aws_secrets_mgr
  password = JSON.parse(secret.secret_string)['password']
  conn = PG.connect(host: 'db.example.com', dbname: 'prod', password: password)
  result = conn.exec('SELECT 1')
  BenchmarkResponse.json({ ok: true })
end
# vuln-code-snippet end ruby_hardcoded_aws_secrets_mgr
