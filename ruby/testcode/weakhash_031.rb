require_relative 'shared'
require 'digest'

# vuln-code-snippet start ruby_weakhash_sha1_integrity
def check_file_integrity(req)
  file_content = req.body_str
  checksum = Digest::SHA1.hexdigest(file_content) # vuln-code-snippet vuln-line ruby_weakhash_sha1_integrity
  BenchmarkResponse.json({ checksum: checksum })
end
# vuln-code-snippet end ruby_weakhash_sha1_integrity
