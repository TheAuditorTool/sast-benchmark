require_relative 'shared'

# vuln-code-snippet start ruby_headerinj_x_frame_sameorigin
def set_x_frame_sameorigin(req)
  response = BenchmarkResponse.ok('ok')
  response.headers['X-Frame-Options'] = 'SAMEORIGIN' # vuln-code-snippet safe-line ruby_headerinj_x_frame_sameorigin
  response
end
# vuln-code-snippet end ruby_headerinj_x_frame_sameorigin
