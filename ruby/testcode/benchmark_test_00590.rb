require_relative 'shared'
require 'logger'

def log_header_value(req)
  referer = req.header('Referer')
  logger = Logger.new($stdout)
  logger.info("Referer: #{referer}")
  BenchmarkResponse.ok('ok')
end
