require_relative 'shared'

def proxy_forward(req)
  client_ip = req.param('ip')
  BenchmarkResponse.new(200, 'ok', { 'X-Forwarded-For' => client_ip })
end
