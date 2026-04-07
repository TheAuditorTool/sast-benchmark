require_relative 'shared'

# vuln-code-snippet start ruby_hardcoded_ejson_decrypt
def ejson_db_connect_handler(req)
  secrets = EJSON.load(File.read('secrets.ejson'))  # vuln-code-snippet safe-line ruby_hardcoded_ejson_decrypt
  password = secrets[:db_password]
  conn = PG.connect(host: 'db.example.com', dbname: 'prod', password: password)
  BenchmarkResponse.ok('connected')
end
# vuln-code-snippet end ruby_hardcoded_ejson_decrypt
