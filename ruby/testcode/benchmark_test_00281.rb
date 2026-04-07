require_relative 'shared'

FROZEN_REGISTRY_042 = {
  export: -> { 'exporting' },
  import: -> { 'importing' }
}.freeze

def frozen_registry_dispatch(req)
  key    = req.param('op').to_sym
  result = FROZEN_REGISTRY_042.fetch(key).call
  BenchmarkResponse.json({ result: result })
end
