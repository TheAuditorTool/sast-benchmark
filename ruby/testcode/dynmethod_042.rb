require_relative 'shared'

FROZEN_REGISTRY_042 = {
  export: -> { 'exporting' },
  import: -> { 'importing' }
}.freeze

# vuln-code-snippet start ruby_dynmethod_frozen_registry
def frozen_registry_dispatch(req)
  key    = req.param('op').to_sym
  result = FROZEN_REGISTRY_042.fetch(key).call # vuln-code-snippet safe-line ruby_dynmethod_frozen_registry
  BenchmarkResponse.json({ result: result })
end
# vuln-code-snippet end ruby_dynmethod_frozen_registry
