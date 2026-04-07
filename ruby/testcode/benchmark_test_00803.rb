require_relative 'shared'

def update_with_double_submit(req)
  cookie_token = req.cookie('csrf_token')
  header_token = req.header('X-CSRF-Token')
  return BenchmarkResponse.error('CSRF rejected', 403) unless cookie_token == header_token
  BenchmarkResponse.ok('updated')
end
