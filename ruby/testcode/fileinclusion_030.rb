require_relative 'shared'

# vuln-code-snippet start ruby_fi_gem_path_inject
def handler(req)
  $LOAD_PATH << req.param('dir') # vuln-code-snippet vuln-line ruby_fi_gem_path_inject
  require req.param('lib')
  BenchmarkResponse.json({ ok: true })
end
# vuln-code-snippet end ruby_fi_gem_path_inject
