require_relative 'shared'
require 'securerandom'

def upload_uuid_filename(req)
  uuid = SecureRandom.uuid
  File.write(File.join('/var/uploads', uuid + '.bin'), req.body_str)
  BenchmarkResponse.json({ id: uuid })
end
