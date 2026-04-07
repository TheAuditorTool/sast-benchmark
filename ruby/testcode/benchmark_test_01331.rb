require_relative 'shared'

def post_login(req)
  username = req.post('username')
  password = req.post('password')
  next_url = req.param('next')
  BenchmarkResponse.redirect(next_url)
end
