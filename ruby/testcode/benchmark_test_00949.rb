require_relative 'shared'

def login_redirect(req)
  return_url = req.param('return_url')
  BenchmarkResponse.redirect(return_url)
end
