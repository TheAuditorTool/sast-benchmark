require_relative 'shared'

# vuln-code-snippet start ruby_hardcoded_env_fetch
def connect_service(req)
  api_key = ENV.fetch('SERVICE_API_KEY') # vuln-code-snippet safe-line ruby_hardcoded_env_fetch
  BenchmarkResponse.ok("connected with key: #{api_key[0..3]}...")
end
# vuln-code-snippet end ruby_hardcoded_env_fetch
