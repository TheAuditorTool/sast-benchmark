require_relative 'shared'

# vuln-code-snippet start ruby_redirect_relative_check
def redirect_relative_check(req)
  url = req.param('next')
  raise 'invalid' unless url.start_with?('/') && !url.start_with?('//')
  BenchmarkResponse.new(302, { 'Location' => url }, '') # vuln-code-snippet safe-line ruby_redirect_relative_check
end
# vuln-code-snippet end ruby_redirect_relative_check
