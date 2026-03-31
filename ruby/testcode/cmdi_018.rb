require_relative 'shared'

# vuln-code-snippet start ruby_cmdi_sanitized_path
def process_upload(req)
  raw_name = req.param('filename')
  safe_name = File.basename(raw_name)
  unless safe_name =~ /\A[a-zA-Z0-9._-]+\z/ # vuln-code-snippet safe-line ruby_cmdi_sanitized_path
    return BenchmarkResponse.bad_request("invalid filename")
  end
  result = `file /uploads/#{safe_name}`
  BenchmarkResponse.ok(result)
end
# vuln-code-snippet end ruby_cmdi_sanitized_path
