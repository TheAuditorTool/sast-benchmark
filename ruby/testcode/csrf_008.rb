require_relative 'shared'

# vuln-code-snippet start ruby_csrf_samesite_strict
def set_session_cookie(req)
  session_id = req.param('session_id')
  response = BenchmarkResponse.ok('session set')
  response.headers['Set-Cookie'] = Rack::Utils.add_cookie_to_header(  # vuln-code-snippet safe-line ruby_csrf_samesite_strict
    nil, 'session', { value: session_id, same_site: 'Strict', secure: true, http_only: true }
  )
  response
end
# vuln-code-snippet end ruby_csrf_samesite_strict
