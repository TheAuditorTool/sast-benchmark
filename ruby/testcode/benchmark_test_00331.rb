require_relative 'shared'

def oauth_callback(req)
  code = req.param('code')
  BenchmarkResponse.ok("oauth code: #{code}")
end
