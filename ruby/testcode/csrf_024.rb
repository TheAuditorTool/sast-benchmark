require_relative 'shared'

# vuln-code-snippet start ruby_csrf_multipart_skip
def upload_document(req)
  # CSRF verification skipped for multipart — attacker can trick browser to submit
  # multipart form cross-site without token
  filename = req.param('filename')
  content = req.post('content')
  File.write(File.join('/app/uploads', File.basename(filename)), content)  # vuln-code-snippet vuln-line ruby_csrf_multipart_skip
  BenchmarkResponse.json({ result: filename })
end
# vuln-code-snippet end ruby_csrf_multipart_skip
