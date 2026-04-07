require_relative 'shared'

def handler(req)
  require File.join('lib', req.param('module'))
  BenchmarkResponse.json({ ok: true })
end
