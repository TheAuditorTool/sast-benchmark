require_relative 'shared'

def set_flash_cookie(req)
  message = req.param('message')
  response = BenchmarkResponse.ok('flash set')
  response.headers['Set-Cookie'] = "flash=#{message}; Secure; SameSite=Strict"
  response
end
