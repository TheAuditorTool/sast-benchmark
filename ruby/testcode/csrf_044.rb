require_relative 'shared'

# vuln-code-snippet start ruby_csrf_rack_protection
def protected_endpoint_rack(req)
  # Rack::Protection::AuthenticityToken middleware validates token before this runs
  # If token is invalid, middleware returns 403 before reaching handler
  authenticity_token = req.header('X-CSRF-Token') || req.post('authenticity_token')
  raise 'CSRF validation must happen in middleware' unless authenticity_token  # vuln-code-snippet safe-line ruby_csrf_rack_protection
  data = req.post('data')
  BenchmarkResponse.json({ result: data })
end
# vuln-code-snippet end ruby_csrf_rack_protection
