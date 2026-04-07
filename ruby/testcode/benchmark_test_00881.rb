require_relative 'shared'
require 'digest'

def check_file_integrity(req)
  file_content = req.body_str
  checksum = Digest::SHA1.hexdigest(file_content)
  BenchmarkResponse.json({ checksum: checksum })
end
