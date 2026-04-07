require_relative 'shared'

def ejson_db_connect_handler(req)
  secrets = EJSON.load(File.read('secrets.ejson'))
  password = secrets[:db_password]
  conn = PG.connect(host: 'db.example.com', dbname: 'prod', password: password)
  BenchmarkResponse.ok('connected')
end
