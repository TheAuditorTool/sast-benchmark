require_relative 'shared'
require 'logger'
require 'json'

LOGGER = Logger.new($stdout)

# vuln-code-snippet start ruby_loginj_semantic_typed
def log_semantic_typed(req)
  LOGGER.info({ message: 'done', payload: { user_id: Integer(req.param('id')) } }.to_json) # vuln-code-snippet safe-line ruby_loginj_semantic_typed
  BenchmarkResponse.json({ ok: true })
end
# vuln-code-snippet end ruby_loginj_semantic_typed
