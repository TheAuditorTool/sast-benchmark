require_relative 'shared'
require 'securerandom'

UPLOAD_DIR = '/var/uploads'.freeze

def upload_image(req)
  upload = req.file('image')
  return BenchmarkResponse.bad_request('no file') unless upload

  original_name = upload[:filename]
  ext = File.extname(original_name).downcase
  dest = File.join(UPLOAD_DIR, SecureRandom.uuid + ext)
  File.write(dest, upload[:data])

  file_content_type = case ext
                      when '.jpg', '.jpeg' then 'image/jpeg'
                      when '.png'          then 'image/png'
                      when '.svg'          then 'image/svg+xml'
                      else 'application/octet-stream'
                      end

  BenchmarkResponse.new(200, upload[:data], { 'Content-Type' => file_content_type })
end
