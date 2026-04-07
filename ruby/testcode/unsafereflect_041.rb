require_relative 'shared'

SERVICES = { 'logger' => -> { 'logging' } }.freeze

# vuln-code-snippet start ruby_reflect_dry_container_resolve
def handler(req)
  result = SERVICES.fetch(req.param('service')).call # vuln-code-snippet safe-line ruby_reflect_dry_container_resolve
  BenchmarkResponse.json({ result: result })
end
# vuln-code-snippet end ruby_reflect_dry_container_resolve
