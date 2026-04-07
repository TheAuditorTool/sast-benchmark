require_relative 'shared'

def update_samesite(req)
  cookie_token = req.cookie('csrf_token')
  param_token = req.post('csrf_token')
  return BenchmarkResponse.error('CSRF rejected', 403) unless cookie_token && param_token && cookie_token == param_token
  BenchmarkResponse.ok('updated')
end
