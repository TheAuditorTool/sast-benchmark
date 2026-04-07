require_relative 'shared'

# vuln-code-snippet start ruby_csrf_subdomain_bypass
def api_endpoint_with_cors(req)
  origin = req.header('Origin')
  # Wildcard subdomain CORS — attacker.example.com can send credentialed CORS requests
  if origin&.end_with?('.example.com')
    cors_origin = origin  # vuln-code-snippet vuln-line ruby_csrf_subdomain_bypass
  else
    cors_origin = 'null'
  end
  BenchmarkResponse.json({ result: cors_origin })
end
# vuln-code-snippet end ruby_csrf_subdomain_bypass
