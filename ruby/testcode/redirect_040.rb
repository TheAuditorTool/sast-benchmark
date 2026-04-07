require_relative 'shared'

DESTS = { 'home' => '/', 'dashboard' => '/dashboard', 'profile' => '/profile' }.freeze

# vuln-code-snippet start ruby_redirect_enum_destination
def redirect_enum_destination(req)
  dest = DESTS.fetch(req.param('dest'), '/')
  BenchmarkResponse.new(302, { 'Location' => dest }, '') # vuln-code-snippet safe-line ruby_redirect_enum_destination
end
# vuln-code-snippet end ruby_redirect_enum_destination
