require_relative 'shared'

def redirect_after_login_open(req)
  stored_url = req.param('next')
  BenchmarkResponse.new(302, { 'Location' => stored_url }, '')
end
