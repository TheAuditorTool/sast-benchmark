require_relative 'shared'
require 'securerandom'

MAGIC_UPLOAD_DIR = '/var/uploads'.freeze

MAGIC_SIGNATURES = {
  "\xFF\xD8\xFF".b             => { ext: '.jpg', mime: 'image/jpeg' },
  "\x89PNG\r\n\x1A\n".b       => { ext: '.png', mime: 'image/png' },
  "\x25\x50\x44\x46".b        => { ext: '.pdf', mime: 'application/pdf' },
  "GIF87a".b                  => { ext: '.gif', mime: 'image/gif' },
  "GIF89a".b                  => { ext: '.gif', mime: 'image/gif' }
}.freeze

def detect_type(data)
  header = data.byteslice(0, 512).b
  MAGIC_SIGNATURES.each do |sig, info|
    return info if header.start_with?(sig)
  end
  nil
end

def upload_with_magic_check(req)
  upload = req.file('file')
  return BenchmarkResponse.bad_request('no file') unless upload

  type_info = detect_type(upload[:data])
  return BenchmarkResponse.bad_request('unrecognized file type') unless type_info

  dest = File.join(MAGIC_UPLOAD_DIR, SecureRandom.uuid + type_info[:ext])
  File.write(dest, upload[:data])

  BenchmarkResponse.json({ path: dest, mime: type_info[:mime] })
end
