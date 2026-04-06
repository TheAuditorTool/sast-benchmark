require_relative 'shared'

# vuln-code-snippet start ruby_headerinj_content_disp
def download_file(req)
  filename = req.param('filename')
  BenchmarkResponse.new(200, 'data', { 'Content-Disposition' => "attachment; filename=\"#{filename}\"" }) # vuln-code-snippet vuln-line ruby_headerinj_content_disp
end
# vuln-code-snippet end ruby_headerinj_content_disp
