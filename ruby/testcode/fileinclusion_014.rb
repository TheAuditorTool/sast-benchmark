require_relative 'shared'

# vuln-code-snippet start ruby_fi_load_gem_internal
def load_stdlib(req)
  require 'json' # vuln-code-snippet safe-line ruby_fi_load_gem_internal
  data = JSON.generate({ status: 'ok' })
  BenchmarkResponse.ok(data)
end
# vuln-code-snippet end ruby_fi_load_gem_internal
