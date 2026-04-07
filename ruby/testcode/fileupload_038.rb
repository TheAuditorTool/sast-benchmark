require_relative 'shared'
require 'securerandom'

begin
  require 'loofah'
rescue LoadError
  # loofah not available in syntax-check mode
end

SVG_UPLOAD_DIR = '/var/uploads'.freeze

# vuln-code-snippet start ruby_fileupload_svg_sanitize
def upload_svg_safe(req)
  upload = req.file('image')
  return BenchmarkResponse.bad_request('no file') unless upload

  return BenchmarkResponse.bad_request('not svg') unless upload[:filename].end_with?('.svg')

  sanitized = if defined?(Loofah)
                Loofah.xml_fragment(upload[:data]).scrub!(:strip).to_s # vuln-code-snippet safe-line ruby_fileupload_svg_sanitize
              else
                ''
              end

  dest = File.join(SVG_UPLOAD_DIR, SecureRandom.uuid + '.svg')
  File.write(dest, sanitized)

  BenchmarkResponse.ok("stored: #{dest}")
end
# vuln-code-snippet end ruby_fileupload_svg_sanitize
