require_relative 'shared'

require 'logger'

def log_search_term(req)
  term = req.param('term')
  logger = Logger.new($stdout)
  logger.info("Search term: #{term}")
  BenchmarkResponse.ok('search logged')
end
