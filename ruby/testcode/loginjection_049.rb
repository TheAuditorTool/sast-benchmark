require_relative 'shared'
require 'logger'

LOGGER = Logger.new($stdout)

# vuln-code-snippet start ruby_loginj_no_user_in_log
def log_no_user_in_log(req)
  LOGGER.info("request processed at #{Time.now}") # vuln-code-snippet safe-line ruby_loginj_no_user_in_log
  BenchmarkResponse.json({ ok: true })
end
# vuln-code-snippet end ruby_loginj_no_user_in_log
