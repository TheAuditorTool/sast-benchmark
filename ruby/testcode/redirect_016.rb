require_relative 'shared'
require 'uri'

# vuln-code-snippet start ruby_redirect_parsed_path
def redirect_path_parsed(req)
  url = req.param('url')
  parsed = URI.parse(url) rescue nil
  safe_path = parsed&.path || '/'
  BenchmarkResponse.redirect(safe_path) # vuln-code-snippet safe-line ruby_redirect_parsed_path
end
# vuln-code-snippet end ruby_redirect_parsed_path
