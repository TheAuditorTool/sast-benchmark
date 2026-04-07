require_relative 'shared'
require 'securerandom'

UPLOAD_DIR = '/var/uploads'.freeze

# vuln-code-snippet start ruby_fileupload_svg_xss
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

  BenchmarkResponse.new(200, upload[:data], { 'Content-Type' => file_content_type }) # vuln-code-snippet vuln-line ruby_fileupload_svg_xss
end
# vuln-code-snippet end ruby_fileupload_svg_xss
