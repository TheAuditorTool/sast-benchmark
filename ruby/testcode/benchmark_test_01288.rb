require_relative 'shared'

TYPE_MAP = { 'string' => -> { String.new }, 'int' => -> { 0 } }.freeze

def handler(req)
  result = TYPE_MAP.fetch(req.param('type')).call
  BenchmarkResponse.json({ result: result.class.name })
end
