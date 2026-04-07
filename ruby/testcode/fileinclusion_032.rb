require_relative 'shared'

# vuln-code-snippet start ruby_fi_config_load
def handler(req)
  load(File.join('config', req.param('env') + '.rb')) # vuln-code-snippet vuln-line ruby_fi_config_load
  BenchmarkResponse.json({ ok: true })
end
# vuln-code-snippet end ruby_fi_config_load
