require_relative 'shared'

def connect_database(req)
  db_url = ENV.fetch('DATABASE_URL')
  BenchmarkResponse.ok("connected to #{db_url.split('@').last}")
end
