require_relative 'shared'
require 'logger'

LOGGER = Logger.new($stdout)

# vuln-code-snippet start ruby_loginj_semantic_level
def log_semantic_level(req)
  LOGGER.warn("user=#{req.param('class_name')} accessed admin area") # vuln-code-snippet vuln-line ruby_loginj_semantic_level
  BenchmarkResponse.json({ ok: true })
end
# vuln-code-snippet end ruby_loginj_semantic_level
