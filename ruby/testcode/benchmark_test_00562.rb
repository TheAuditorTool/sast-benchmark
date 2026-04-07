require_relative 'shared'

def set_x_frame(req)
  response = BenchmarkResponse.ok('ok')
  response.headers['X-Frame-Options'] = "ALLOW-FROM #{req.param('origin')}"
  response
end
