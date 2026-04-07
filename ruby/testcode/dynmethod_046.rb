require_relative 'shared'

CONTAINER_046 = {
  'logger' => -> { 'log' },
  'mailer' => -> { 'mail' }
}.freeze

# vuln-code-snippet start ruby_dynmethod_dry_container
def dry_container_dispatch(req)
  service = req.param('service')
  result  = CONTAINER_046.fetch(service).call # vuln-code-snippet safe-line ruby_dynmethod_dry_container
  BenchmarkResponse.json({ result: result })
end
# vuln-code-snippet end ruby_dynmethod_dry_container
