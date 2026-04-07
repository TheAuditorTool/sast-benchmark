require_relative 'shared'

# vuln-code-snippet start ruby_redirect_no_open_redirect
def redirect_no_open_redirect(req)
  BenchmarkResponse.new(302, { 'Location' => '/thank-you' }, '') # vuln-code-snippet safe-line ruby_redirect_no_open_redirect
end
# vuln-code-snippet end ruby_redirect_no_open_redirect
