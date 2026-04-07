require_relative 'shared'

SAFE = %w[admin user guest].freeze

def handler(req)
  r = req.param('role')
  raise 'invalid role' unless SAFE.include?(r)
  load("roles/#{r}.rb")
  BenchmarkResponse.json({ ok: true })
end
