require_relative 'shared'
require 'uri'

# vuln-code-snippet start ruby_redirect_rails7_default
def redirect_rails7_default(req)
  url = req.param('url')
  uri = URI.parse(url)
  # blocks external hosts by default (Rails 7 behavior)
  dest = uri.host.nil? ? url : '/'
  BenchmarkResponse.new(302, { 'Location' => dest }, '') # vuln-code-snippet safe-line ruby_redirect_rails7_default
end
# vuln-code-snippet end ruby_redirect_rails7_default
