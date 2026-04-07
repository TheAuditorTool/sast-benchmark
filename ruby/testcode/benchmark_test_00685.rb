require_relative 'shared'

def proxy_request(req)
  forwarded = req.header('X-Forward')
  headers = { 'Content-Type' => 'text/plain' }
  headers['X-User-Data'] = forwarded
  BenchmarkResponse.new(200, 'ok', headers)
end
