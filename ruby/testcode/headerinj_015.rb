require_relative 'shared'

# vuln-code-snippet start ruby_headerinj_cache_control
def set_cache(req)
  cache_val = req.param('cache')
  BenchmarkResponse.new(200, 'ok', { 'Cache-Control' => cache_val }) # vuln-code-snippet vuln-line ruby_headerinj_cache_control
end
# vuln-code-snippet end ruby_headerinj_cache_control
