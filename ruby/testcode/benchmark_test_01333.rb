require_relative 'shared'

def handle_oauth_callback(req)
  return_to = req.param('return_to')
  code = req.param('code')
  BenchmarkResponse.redirect(return_to)
end
