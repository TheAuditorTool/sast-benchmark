require_relative 'shared'

# vuln-code-snippet start ruby_hardcoded_gcp_secret_mgr
def gcp_db_connect_handler(req)
  sm_client = Google::Cloud::SecretManager.new
  version = sm_client.access_secret_version(name: 'projects/myproject/secrets/db-pass/versions/latest')  # vuln-code-snippet safe-line ruby_hardcoded_gcp_secret_mgr
  password = version.payload.data
  conn = PG.connect(host: 'db.example.com', dbname: 'prod', password: password)
  BenchmarkResponse.ok('connected')
end
# vuln-code-snippet end ruby_hardcoded_gcp_secret_mgr
