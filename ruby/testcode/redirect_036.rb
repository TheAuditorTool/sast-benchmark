require_relative 'shared'
require 'uri'

# vuln-code-snippet start ruby_redirect_addressable_validate
def redirect_addressable_validate(req)
  u = URI.parse(req.param('url'))
  dest = u.host.nil? ? req.param('url') : '/'
  BenchmarkResponse.new(302, { 'Location' => dest }, '') # vuln-code-snippet safe-line ruby_redirect_addressable_validate
end
# vuln-code-snippet end ruby_redirect_addressable_validate
