require_relative 'shared'
require 'logger'

LOGGER = Logger.new($stdout)

ACTIONS = %w[login logout view].freeze

# vuln-code-snippet start ruby_loginj_enum_action
def log_enum_action(req)
  action = ACTIONS.include?(req.param('action')) ? req.param('action') : 'unknown'
  LOGGER.info("action=#{action}") # vuln-code-snippet safe-line ruby_loginj_enum_action
  BenchmarkResponse.json({ ok: true })
end
# vuln-code-snippet end ruby_loginj_enum_action
