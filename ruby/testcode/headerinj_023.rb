require_relative 'shared'

# vuln-code-snippet start ruby_headerinj_link_preload
def set_link_preload(req)
  response = BenchmarkResponse.ok('ok')
  response.headers['Link'] = "<#{req.param('url')}>; rel=preload" # vuln-code-snippet vuln-line ruby_headerinj_link_preload
  response
end
# vuln-code-snippet end ruby_headerinj_link_preload
