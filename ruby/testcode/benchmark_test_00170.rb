require_relative 'shared'

PROCS = { 'export' => -> { 'done' }, 'report' => -> { 'generated' } }.freeze

def handler(req)
  result = PROCS.fetch(req.param('action')).call
  BenchmarkResponse.json({ result: result })
end
