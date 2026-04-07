require_relative 'shared'

def handle_oauth_callback(req)
  return_to = req.param('return_to')
  code = req.param('code')
  safe_destination = return_to.to_s.start_with?('/') ? return_to : '/home'
  BenchmarkResponse.redirect(safe_destination)
end
