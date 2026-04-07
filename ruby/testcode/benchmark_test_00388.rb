require_relative 'shared'

def set_x_frame_sameorigin(req)
  response = BenchmarkResponse.ok('ok')
  response.headers['X-Frame-Options'] = 'SAMEORIGIN'
  response
end
