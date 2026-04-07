require_relative 'shared'

# vuln-code-snippet start ruby_csrf_double_submit_hdr
def api_update_safe(req)
  cookie_token = req.cookie('csrf_token')
  header_token = req.header('X-CSRF-Token')
  # Both cookie and header must be present and match
  unless cookie_token && header_token && Rack::Utils.secure_compare(cookie_token, header_token)  # vuln-code-snippet safe-line ruby_csrf_double_submit_hdr
    return BenchmarkResponse.bad_request('CSRF validation failed')
  end
  data = req.post('data')
  BenchmarkResponse.json({ result: data })
end
# vuln-code-snippet end ruby_csrf_double_submit_hdr
