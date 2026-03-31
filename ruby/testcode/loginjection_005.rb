require_relative 'shared'

require 'logger'

# vuln-code-snippet start ruby_loginj_ansi_inject
def log_search_term(req)
  term = req.param('term')
  logger = Logger.new($stdout)
  logger.info("Search term: #{term}")  # vuln-code-snippet vuln-line ruby_loginj_ansi_inject
  BenchmarkResponse.ok('search logged')
end
# vuln-code-snippet end ruby_loginj_ansi_inject
