require_relative 'shared'
require 'logger'

LOGGER = Logger.new($stdout)

def risky_op(data)
  raise "bad input: #{data}" if data.include?('!')
end

def log_exception_message(req)
  begin
    risky_op(req.param('data'))
  rescue => e
    LOGGER.error(e.message)
  end
  BenchmarkResponse.json({ ok: true })
end
