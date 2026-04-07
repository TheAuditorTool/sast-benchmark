require_relative 'shared'
require 'logger'

def log_progname_constant(req)
  Logger.new($stdout, progname: 'MyApp').info("request processed")
  BenchmarkResponse.json({ ok: true })
end
