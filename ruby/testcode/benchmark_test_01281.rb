require_relative 'shared'

def set_raw_cookie(req)
  response = BenchmarkResponse.ok('ok')
  response.headers['Set-Cookie'] = "s=#{req.param('v')}; HttpOnly"
  response
end
