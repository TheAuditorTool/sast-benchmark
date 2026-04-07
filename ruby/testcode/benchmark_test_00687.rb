require_relative 'shared'
require 'logger'
require 'json'

LOGGER = Logger.new($stdout)

def log_semantic_typed(req)
  LOGGER.info({ message: 'done', payload: { user_id: Integer(req.param('id')) } }.to_json)
  BenchmarkResponse.json({ ok: true })
end
