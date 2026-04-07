require_relative 'shared'

# vuln-code-snippet start ruby_hardcoded_identity_token
def s3_list_handler(req)
  # No credentials in code — IRSA injects IAM role via environment
  client = Aws::S3::Client.new  # vuln-code-snippet safe-line ruby_hardcoded_identity_token
  objects = client.list_objects_v2(bucket: req[:bucket])
  BenchmarkResponse.json({ keys: objects.contents.map(&:key) })
end
# vuln-code-snippet end ruby_hardcoded_identity_token
