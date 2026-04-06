require_relative 'shared'
require 'logger'

# vuln-code-snippet start ruby_loginj_semantic_hash
def log_semantic(req)
  user_id = req.param('user_id').to_i
  logger = Logger.new($stdout)
  logger.info({ event: :login, user_id: user_id }.inspect) # vuln-code-snippet safe-line ruby_loginj_semantic_hash
  BenchmarkResponse.ok('ok')
end
# vuln-code-snippet end ruby_loginj_semantic_hash
