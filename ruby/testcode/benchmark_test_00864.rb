require_relative 'shared'

def connect_service(req)
  api_key = ENV.fetch('SERVICE_API_KEY')
  BenchmarkResponse.ok("connected with key: #{api_key[0..3]}...")
end
