require_relative 'shared'
require 'logger'

LOGGER = Logger.new($stdout)

def risky_op(data)
  raise "bad input: #{data}" if data.include?('!')
end

# vuln-code-snippet start ruby_loginj_exception_message
def log_exception_message(req)
  begin
    risky_op(req.param('data'))
  rescue => e
    LOGGER.error(e.message) # vuln-code-snippet vuln-line ruby_loginj_exception_message
  end
  BenchmarkResponse.json({ ok: true })
end
# vuln-code-snippet end ruby_loginj_exception_message
