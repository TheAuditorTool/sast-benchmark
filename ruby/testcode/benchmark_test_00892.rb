require_relative 'shared'
require 'logger'

LOGGER = Logger.new($stdout)

ACTIONS = %w[login logout view].freeze

def log_enum_action(req)
  action = ACTIONS.include?(req.param('action')) ? req.param('action') : 'unknown'
  LOGGER.info("action=#{action}")
  BenchmarkResponse.json({ ok: true })
end
