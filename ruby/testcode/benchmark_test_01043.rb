require_relative 'shared'

COMMANDS = { 'create' => -> { 'created' }, 'delete' => -> { 'deleted' } }.freeze

def handler(req)
  result = COMMANDS.fetch(req.param('cmd')).call
  BenchmarkResponse.json({ result: result })
end
