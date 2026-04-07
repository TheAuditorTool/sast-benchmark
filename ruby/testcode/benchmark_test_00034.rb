require_relative 'shared'
require 'logger'

LOGGER = Logger.new($stdout)

def log_strip_crlf_null(req)
  msg = req.param('msg').gsub(/[\r\n\x00]/, ' ')
  LOGGER.info(msg)
  BenchmarkResponse.json({ ok: true })
end
