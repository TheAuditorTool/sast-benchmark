require_relative 'shared'
require 'json'
require 'logger'

# vuln-code-snippet start ruby_loginj_lograge_fmt
def log_lograge_style(req)
  user_id = req.param('user_id').to_i
  logger = Logger.new($stdout)
  payload = { event: 'login', user_id: user_id, status: 'success' }
  logger.info(JSON.generate(payload)) # vuln-code-snippet safe-line ruby_loginj_lograge_fmt
  BenchmarkResponse.ok('logged in')
end
# vuln-code-snippet end ruby_loginj_lograge_fmt
