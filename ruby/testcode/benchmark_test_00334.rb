require_relative 'shared'

HANDLERS_038 = {
  'read' => -> { 'reading' },
  'list' => -> { 'listing' }
}.freeze

def hash_proc_dispatch(req)
  handler = HANDLERS_038.fetch(req.param('action'))
  result  = handler.call
  BenchmarkResponse.json({ result: result })
end
