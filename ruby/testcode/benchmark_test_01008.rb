require_relative 'shared'
require 'logger'

def log_progname_taint(req)
  Logger.new($stdout, progname: req.param('service')).info("started")
  BenchmarkResponse.json({ ok: true })
end
