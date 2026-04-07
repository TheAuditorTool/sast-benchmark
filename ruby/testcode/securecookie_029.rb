require_relative 'shared'

# vuln-code-snippet start ruby_securecookie_flash_insecure
def set_flash_cookie(req)
  message = req.param('message')
  response = BenchmarkResponse.ok('flash set')
  response.headers['Set-Cookie'] = "flash=#{message}; Secure; SameSite=Strict" # vuln-code-snippet vuln-line ruby_securecookie_flash_insecure
  response
end
# vuln-code-snippet end ruby_securecookie_flash_insecure
