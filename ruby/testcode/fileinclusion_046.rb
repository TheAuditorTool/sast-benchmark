require_relative 'shared'

PATH_MAP = { report: 'lib/report.rb', audit: 'lib/audit.rb' }.freeze

# vuln-code-snippet start ruby_fi_frozen_path_map
def handler(req)
  load(PATH_MAP.fetch(req.param('key').to_sym)) # vuln-code-snippet safe-line ruby_fi_frozen_path_map
  BenchmarkResponse.json({ ok: true })
end
# vuln-code-snippet end ruby_fi_frozen_path_map
