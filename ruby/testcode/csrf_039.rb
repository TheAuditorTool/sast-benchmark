require_relative 'shared'

# vuln-code-snippet start ruby_csrf_api_token_auth
def api_create_resource(req)
  auth_header = req.header('Authorization')
  return BenchmarkResponse.bad_request('missing token') unless auth_header&.start_with?('Bearer ')
  jwt_token = auth_header.split(' ', 2).last
  # Bearer token auth — no session cookie, CSRF not applicable to token-authenticated APIs
  payload = JWT.decode(jwt_token, ENV.fetch('JWT_SECRET'), true, algorithms: ['HS256']).first  # vuln-code-snippet safe-line ruby_csrf_api_token_auth
  BenchmarkResponse.json({ result: payload['sub'] })
end
# vuln-code-snippet end ruby_csrf_api_token_auth
