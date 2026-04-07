require_relative 'shared'

SVCMAP = { 1 => -> { 'svc1' }, 2 => -> { 'svc2' } }.freeze

def handler(req)
  result = SVCMAP.fetch(req.param('id').to_i).call
  BenchmarkResponse.json({ result: result })
end
