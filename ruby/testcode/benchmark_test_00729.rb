require_relative 'shared'

DESTS = { 'home' => '/', 'dashboard' => '/dashboard', 'profile' => '/profile' }.freeze

def redirect_enum_destination(req)
  dest = DESTS.fetch(req.param('dest'), '/')
  BenchmarkResponse.new(302, { 'Location' => dest }, '')
end
