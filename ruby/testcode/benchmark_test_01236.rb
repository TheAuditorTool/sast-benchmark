require_relative 'shared'

SERVICES = { 'logger' => -> { 'logging' } }.freeze

def handler(req)
  result = SERVICES.fetch(req.param('service')).call
  BenchmarkResponse.json({ result: result })
end
