require_relative 'shared'
require 'securerandom'

# vuln-code-snippet start ruby_pt_uuid_filename
def upload_uuid_filename(req)
  uuid = SecureRandom.uuid
  File.write(File.join('/var/uploads', uuid + '.bin'), req.body_str)
  BenchmarkResponse.json({ id: uuid }) # vuln-code-snippet safe-line ruby_pt_uuid_filename
end
# vuln-code-snippet end ruby_pt_uuid_filename
