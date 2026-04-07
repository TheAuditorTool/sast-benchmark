require_relative 'shared'

# vuln-code-snippet start ruby_redirect_rack_location
def redirect_rack_location(req)
  [302, { 'Location' => req.param('url') }, ['']] # vuln-code-snippet vuln-line ruby_redirect_rack_location
end
# vuln-code-snippet end ruby_redirect_rack_location
