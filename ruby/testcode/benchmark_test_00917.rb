require_relative 'shared'

def set_csp(req)
  response = BenchmarkResponse.ok('ok')
  response.headers['Content-Security-Policy'] = "script-src #{req.param('domain')}"
  response
end
