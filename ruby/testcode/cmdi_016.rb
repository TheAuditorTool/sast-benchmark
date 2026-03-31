require_relative 'shared'

# vuln-code-snippet start ruby_cmdi_validated_regex
def fetch_log_file(req)
  filename = req.param('filename')
  unless filename =~ /\A[a-zA-Z0-9._-]+\z/ # vuln-code-snippet safe-line ruby_cmdi_validated_regex
    return BenchmarkResponse.bad_request("invalid filename")
  end
  result = `cat /var/log/#{filename}`
  BenchmarkResponse.ok(result)
end
# vuln-code-snippet end ruby_cmdi_validated_regex
