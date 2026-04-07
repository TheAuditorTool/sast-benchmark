require_relative 'shared'

def json_response(req)
  BenchmarkResponse.new(200, '{}', { 'Content-Type' => 'application/json' })
end
