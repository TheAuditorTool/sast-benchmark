require_relative 'shared'

def gcp_db_connect_handler(req)
  sm_client = Google::Cloud::SecretManager.new
  version = sm_client.access_secret_version(name: 'projects/myproject/secrets/db-pass/versions/latest')
  password = version.payload.data
  conn = PG.connect(host: 'db.example.com', dbname: 'prod', password: password)
  BenchmarkResponse.ok('connected')
end
