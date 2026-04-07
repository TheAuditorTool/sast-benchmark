require_relative 'shared'
require 'securerandom'

begin
  require 'loofah'
rescue LoadError
end

SVG_UPLOAD_DIR = '/var/uploads'.freeze

def upload_svg_safe(req)
  upload = req.file('image')
  return BenchmarkResponse.bad_request('no file') unless upload

  return BenchmarkResponse.bad_request('not svg') unless upload[:filename].end_with?('.svg')

  sanitized = if defined?(Loofah)
                Loofah.xml_fragment(upload[:data]).scrub!(:strip).to_s
              else
                ''
              end

  dest = File.join(SVG_UPLOAD_DIR, SecureRandom.uuid + '.svg')
  File.write(dest, sanitized)

  BenchmarkResponse.ok("stored: #{dest}")
end
