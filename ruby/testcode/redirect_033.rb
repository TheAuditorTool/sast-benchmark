require_relative 'shared'

# vuln-code-snippet start ruby_redirect_root_fallback
def redirect_root_fallback(req)
  url = req.param('url')
  dest = url.start_with?('/') && !url.start_with?('//') ? url : '/'
  BenchmarkResponse.new(302, { 'Location' => dest }, '') # vuln-code-snippet safe-line ruby_redirect_root_fallback
end
# vuln-code-snippet end ruby_redirect_root_fallback
