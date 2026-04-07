require_relative 'shared'

def set_retry_after_clamped(req)
  secs = Integer(req.param('secs')).clamp(0, 3600).to_s
  response = BenchmarkResponse.ok('ok')
  response.headers['Retry-After'] = secs
  response
end
