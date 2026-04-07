require_relative 'shared'

# vuln-code-snippet start ruby_redirect_session_validated
def redirect_session_validated(req)
  url = req.param('return_to')
  raise unless url =~ /\A\/[a-z0-9\/\-_]*\z/
  BenchmarkResponse.new(302, { 'Location' => url }, '') # vuln-code-snippet safe-line ruby_redirect_session_validated
end
# vuln-code-snippet end ruby_redirect_session_validated
