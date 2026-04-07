require_relative 'shared'
require 'uri'

# vuln-code-snippet start ruby_redirect_only_path_helper
def redirect_only_path_helper(req)
  url = req.param('next')
  path = URI.parse(url.start_with?('/') ? url : '/').path
  BenchmarkResponse.new(302, { 'Location' => path }, '') # vuln-code-snippet safe-line ruby_redirect_only_path_helper
end
# vuln-code-snippet end ruby_redirect_only_path_helper
