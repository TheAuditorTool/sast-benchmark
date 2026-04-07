require_relative 'shared'

require 'openssl'

# vuln-code-snippet start ruby_csrf_hmac_token
def process_form_hmac(req)
  session_key = req.cookie('session_key')
  token = req.post('csrf_token')
  expected = OpenSSL::HMAC.hexdigest('SHA256', session_key.to_s, 'csrf')
  unless Rack::Utils.secure_compare(token.to_s, expected)  # vuln-code-snippet safe-line ruby_csrf_hmac_token
    return BenchmarkResponse.bad_request('invalid CSRF token')
  end
  BenchmarkResponse.json({ result: true })
end
# vuln-code-snippet end ruby_csrf_hmac_token
