require_relative 'shared'

# vuln-code-snippet start ruby_headerinj_x_frame_options
def set_x_frame(req)
  response = BenchmarkResponse.ok('ok')
  response.headers['X-Frame-Options'] = "ALLOW-FROM #{req.param('origin')}" # vuln-code-snippet vuln-line ruby_headerinj_x_frame_options
  response
end
# vuln-code-snippet end ruby_headerinj_x_frame_options
