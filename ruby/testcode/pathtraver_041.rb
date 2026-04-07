require_relative 'shared'
require 'pathname'

begin
  require 'zip'
rescue LoadError
  module Zip
    class File
      def self.open(path)
        yield new
      end
      def each; end
    end
  end
end

# vuln-code-snippet start ruby_pt_zip_safe_extract
def zip_safe_extract(req)
  base = '/var/extracts'
  Zip::File.open('/uploads/safe.zip') do |z|
    z.each do |e|
      dest = File.join(base, e.name)
      raise unless Pathname.new(dest).cleanpath.to_s.start_with?(base)
      e.extract(dest) # vuln-code-snippet safe-line ruby_pt_zip_safe_extract
    end
  end
  BenchmarkResponse.json({ ok: true })
end
# vuln-code-snippet end ruby_pt_zip_safe_extract
