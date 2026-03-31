require_relative 'shared'

# vuln-code-snippet start ruby_cmdi_hardcoded
def check_disk_usage(req)
  success = system("df", "-h", "/var/www") # vuln-code-snippet safe-line ruby_cmdi_hardcoded
  result = success ? "disk check complete" : "disk check failed"
  BenchmarkResponse.ok(result)
end
# vuln-code-snippet end ruby_cmdi_hardcoded
