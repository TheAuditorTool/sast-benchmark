require_relative 'shared'

AWS_SECRET_KEY = 'wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY'

# vuln-code-snippet start ruby_hardcoded_aws_secret
def connect_aws(req)
  client = { access_key: 'AKIAIOSFODNN7EXAMPLE', secret_key: AWS_SECRET_KEY } # vuln-code-snippet vuln-line ruby_hardcoded_aws_secret
  BenchmarkResponse.ok("connected: #{client[:access_key]}")
end
# vuln-code-snippet end ruby_hardcoded_aws_secret
