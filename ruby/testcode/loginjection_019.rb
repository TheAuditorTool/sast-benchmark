require_relative 'shared'
require 'logger'

# vuln-code-snippet start ruby_loginj_multiline_header
def log_header_value(req)
  referer = req.header('Referer')
  logger = Logger.new($stdout)
  logger.info("Referer: #{referer}") # vuln-code-snippet vuln-line ruby_loginj_multiline_header
  BenchmarkResponse.ok('ok')
end
# vuln-code-snippet end ruby_loginj_multiline_header
