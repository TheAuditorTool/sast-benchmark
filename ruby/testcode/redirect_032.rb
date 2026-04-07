require_relative 'shared'
require 'uri'

# vuln-code-snippet start ruby_redirect_path_only_uri
def redirect_path_only_uri(req)
  uri = URI.parse(req.param('next'))
  path = uri.path || '/'
  BenchmarkResponse.new(302, { 'Location' => path }, '') # vuln-code-snippet safe-line ruby_redirect_path_only_uri
end
# vuln-code-snippet end ruby_redirect_path_only_uri
