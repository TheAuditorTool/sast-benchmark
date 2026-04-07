require_relative 'shared'

# vuln-code-snippet start ruby_redirect_internal_path_regex
def redirect_internal_path_regex(req)
  url = req.param('url')
  raise unless url =~ /\A\/[^\/]/
  BenchmarkResponse.new(302, { 'Location' => url }, '') # vuln-code-snippet safe-line ruby_redirect_internal_path_regex
end
# vuln-code-snippet end ruby_redirect_internal_path_regex
