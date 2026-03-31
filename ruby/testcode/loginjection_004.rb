require_relative 'shared'

require 'logger'
require 'json'

# vuln-code-snippet start ruby_loginj_structured
def login_user(req)
  user_id = req.param('user_id').to_i
  logger = Logger.new($stdout)
  logger.info(JSON.generate({ event: 'login', user_id: user_id }))  # vuln-code-snippet safe-line ruby_loginj_structured
  BenchmarkResponse.ok('logged in')
end
# vuln-code-snippet end ruby_loginj_structured
