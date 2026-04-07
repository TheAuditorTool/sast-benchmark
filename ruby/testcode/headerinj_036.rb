require_relative 'shared'

# vuln-code-snippet start ruby_headerinj_integer_content_length
def set_computed_content_length(req)
  body = JSON.generate({ ok: true })
  response = BenchmarkResponse.ok(body)
  response.headers['Content-Length'] = body.bytesize.to_s # vuln-code-snippet safe-line ruby_headerinj_integer_content_length
  response
end
# vuln-code-snippet end ruby_headerinj_integer_content_length
