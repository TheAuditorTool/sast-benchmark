require_relative 'shared'

# vuln-code-snippet start ruby_fi_load_plugin_dir
def handler(req)
  load(File.join('plugins', req.param('plugin') + '.rb')) # vuln-code-snippet vuln-line ruby_fi_load_plugin_dir
  BenchmarkResponse.json({ ok: true })
end
# vuln-code-snippet end ruby_fi_load_plugin_dir
