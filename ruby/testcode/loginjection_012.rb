require_relative 'shared'
require 'json'
require 'logger'

# vuln-code-snippet start ruby_loginj_json_logger
def log_json_structured(req)
  username = req.param('username')
  action = req.param('action')
  logger = Logger.new($stdout)
  logger.info(JSON.generate({ event: 'action', user: username, action: action })) # vuln-code-snippet safe-line ruby_loginj_json_logger
  BenchmarkResponse.ok('ok')
end
# vuln-code-snippet end ruby_loginj_json_logger
