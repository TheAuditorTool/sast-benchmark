require_relative 'shared'
require 'json'

def load_api_payload(req)
  input = req.body_str
  data = JSON.parse(input, symbolize_names: true)
  user = data.fetch(:user, 'anonymous').to_s
  BenchmarkResponse.ok("user: #{user}")
end
