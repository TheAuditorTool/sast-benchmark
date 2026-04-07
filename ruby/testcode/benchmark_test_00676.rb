require_relative 'shared'

PATH_MAP = { report: 'lib/report.rb', audit: 'lib/audit.rb' }.freeze

def handler(req)
  load(PATH_MAP.fetch(req.param('key').to_sym))
  BenchmarkResponse.json({ ok: true })
end
