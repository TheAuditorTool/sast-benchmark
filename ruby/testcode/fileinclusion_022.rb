require_relative 'shared'

# vuln-code-snippet start ruby_fi_require_lib_param
def handler(req)
  require File.join('lib', req.param('module')) # vuln-code-snippet vuln-line ruby_fi_require_lib_param
  BenchmarkResponse.json({ ok: true })
end
# vuln-code-snippet end ruby_fi_require_lib_param
